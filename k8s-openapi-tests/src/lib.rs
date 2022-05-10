#![cfg(test)]

#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::indexing_slicing,
	clippy::let_and_return,
	clippy::let_underscore_drop,
	clippy::let_unit_value,
	clippy::must_use_candidate,
	clippy::too_many_lines,
	clippy::unseparated_literal_suffix,
)]

use std::{future::Future, pin::Pin, task::{Context, Poll}};

use futures_core::Stream;
use futures_io::AsyncRead;
use futures_util::{StreamExt, TryStreamExt};
use k8s_openapi::{http, serde_json};

#[derive(Debug)]
enum Client {
	Recording {
		inner: reqwest::Client,
		server: http::Uri,
		replays: Vec<Replay>,
		recorder: std::io::BufWriter<std::fs::File>,
	},

	Replaying(std::iter::Enumerate<std::vec::IntoIter<Replay>>),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Replay {
	request_url: String,
	#[serde(with = "methodstring")]
	request_method: http::Method,
	#[serde(with = "bytestring")]
	request_body: Vec<u8>,
	request_content_type: Option<String>,
	response_status_code: u16,
	#[serde(with = "bytestring")]
	response_body: Vec<u8>,
}

impl Client {
	fn new(test_name: &'static str) -> Self {
		#[cfg(feature = "test_v1_18")] let replays_directory = "v1-18";
		#[cfg(feature = "test_v1_19")] let replays_directory = "v1-19";
		#[cfg(feature = "test_v1_20")] let replays_directory = "v1-20";
		#[cfg(feature = "test_v1_21")] let replays_directory = "v1-21";
		#[cfg(feature = "test_v1_22")] let replays_directory = "v1-22";
		#[cfg(feature = "test_v1_23")] let replays_directory = "v1-23";
		#[cfg(feature = "test_v1_24")] let replays_directory = "v1-24";

		let replays_directory =
			std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR")))
			.join("test-replays")
			.join(replays_directory);
		let () =
			std::fs::create_dir_all(&replays_directory)
			.map_err(|err| format!("couldn't create test-replays directory {}: {err}", replays_directory.display()))
			.unwrap();

		let replay_filename = replays_directory.join(format!("{test_name}.json"));

		if std::env::var("K8S_RECORD").is_ok() {
			let kubeconfig: KubeConfig = {
				let mut kubeconfig_path = dirs::home_dir().expect("can't find home directory");
				kubeconfig_path.push(".kube");
				kubeconfig_path.push("config");
				let kubeconfig_file = std::fs::File::open(kubeconfig_path).expect("couldn't open kube config");
				serde_yaml::from_reader(std::io::BufReader::new(kubeconfig_file)).expect("couldn't parse kube config")
			};

			let context = std::env::var("K8S_CONTEXT").unwrap_or(kubeconfig.current_context);

			let KubeConfigContext { cluster, user } =
				kubeconfig.contexts.into_iter()
				.find(|c| c.name == context).unwrap_or_else(|| panic!("couldn't find context named {context}"))
				.context;

			let KubeConfigCluster { certificate_authority, server } =
				kubeconfig.clusters.into_iter()
				.find(|c| c.name == cluster).unwrap_or_else(|| panic!("couldn't find cluster named {cluster}"))
				.cluster;

			let ca_certificate = {
				let ca_cert_pem = match certificate_authority {
					CertificateAuthority::File(path) => std::fs::read(path).expect("couldn't read CA certificate file"),
					CertificateAuthority::Inline(data) => base64::decode(&data).expect("couldn't parse CA certificate data"),
				};
				let ca_cert = reqwest::Certificate::from_pem(&ca_cert_pem).expect("couldn't create CA certificate");
				ca_cert
			};

			let server: http::Uri = server.parse().expect("couldn't parse server URL");
			if let Some(path_and_query) = server.path_and_query() {
				assert_eq!(path_and_query, "/", "server URL {server} has path and query {path_and_query}");
			}

			let KubeConfigUser { client_certificate, client_key } =
				kubeconfig.users.into_iter()
				.find(|u| u.name == user).unwrap_or_else(|| panic!("couldn't find user named {user}"))
				.user;

			let client_tls_identity = {
				// reqwest::Identity supports from_pem, which is implemented using rustls to parse the PEM.
				// This also requires the reqwest::Client to be built with use_rustls_tls(), otherwise the Identity is ignored.
				//
				// However, the client then fails to connect to kind clusters anyway, because kind clusters listen on 127.0.0.1
				// and hyper-rustls doesn't support connecting to IPs. Ref: https://github.com/ctz/hyper-rustls/issues/84
				//
				// So we need to use the native-tls backend, and thus Identity::from_pkcs12_der

				let public_key_pem = match client_certificate {
					ClientCertificate::File(path) => std::fs::read(path).expect("couldn't read client certificate file"),
					ClientCertificate::Inline(data) => base64::decode(&data).expect("couldn't parse client certificate data"),
				};
				let public_key = openssl::x509::X509::from_pem(&public_key_pem).expect("couldn't parse client certificate data");

				let private_key_pem = match client_key {
					ClientKey::File(path) => std::fs::read(path).expect("couldn't read client key file"),
					ClientKey::Inline(data) => base64::decode(&data).expect("couldn't parse client key data"),
				};
				let private_key = openssl::pkey::PKey::private_key_from_pem(&private_key_pem).expect("couldn't parse client key data");

				let pkcs12 =
					openssl::pkcs12::Pkcs12::builder()
					.build("", "admin", &private_key, &public_key).expect("couldn't construct client identity")
					.to_der().expect("couldn't construct client identity");

				let tls_identity = reqwest::Identity::from_pkcs12_der(&pkcs12, "").expect("couldn't construct client identity");
				tls_identity
			};

			let inner =
				reqwest::Client::builder()
				.use_native_tls()
				.add_root_certificate(ca_certificate)
				.identity(client_tls_identity)
				.build().expect("couldn't create client");

			let replay_file = std::fs::OpenOptions::new().create(true).truncate(true).write(true).open(replay_filename).expect("couldn't open replay file");
			let recorder = std::io::BufWriter::new(replay_file);

			Client::Recording {
				inner,
				server,
				replays: vec![],
				recorder,
			}
		}
		else {
			let replay_file = std::fs::OpenOptions::new().read(true).open(replay_filename).expect("couldn't open replay file");
			let replay_file = std::io::BufReader::new(replay_file);
			let replays: Vec<_> = serde_json::from_reader(replay_file).expect("couldn't parse replay file");

			// Make all `tokio::time::sleep` calls not actually sleep.
			tokio::time::pause();

			Client::Replaying(replays.into_iter().enumerate())
		}
	}

	async fn get_single_value<R>(
		&mut self,
		request: http::Request<Vec<u8>>,
		response_body: fn(http::StatusCode) -> k8s_openapi::ResponseBody<R>,
	) -> (R, http::StatusCode) where R: k8s_openapi::Response {
		let stream = self.get_multiple_values(request, response_body);
		futures_util::pin_mut!(stream);
		stream.next().await.expect("unexpected EOF")
	}

	fn get_multiple_values<'a, R>(
		&'a mut self,
		request: http::Request<Vec<u8>>,
		response_body: fn(http::StatusCode) -> k8s_openapi::ResponseBody<R>,
	) -> impl Stream<Item = (R, http::StatusCode)> + 'a where R: k8s_openapi::Response + 'a {
		MultipleValuesStream::ExecutingRequest {
			f: self.execute(request),
			response_body,
		}
	}

	async fn execute(&mut self, request: http::Request<Vec<u8>>) -> ClientResponse<'_, impl AsyncRead> {
		let (path, method, body, content_type) = {
			let content_type =
				request.headers()
				.get(http::header::CONTENT_TYPE)
				.map(|value|
					value
					.to_str().expect("Content-Type header is not set to valid utf-8 string")
					.to_owned());

			let (parts, body) = request.into_parts();
			let mut url: http::uri::Parts = parts.uri.into();
			let path = url.path_and_query.take().expect("request doesn't have path and query");

			(path, parts.method, body, content_type)
		};

		match self {
			Client::Recording { inner, server, replays, .. } => {
				replays.push(Replay {
					request_url: path.to_string(),
					request_method: method.clone(),
					request_body: body.clone(),
					request_content_type: content_type.clone(),
					response_status_code: 0,
					response_body: vec![],
				});

				let mut replay = replays.last_mut().unwrap();

				let mut url: http::uri::Parts = server.clone().into();
				url.path_and_query = Some(path);
				let url = http::Uri::from_parts(url).expect("couldn't parse URL from parts");

				let request = inner.request(method, &url.to_string());
				let request =
					if let Some(content_type) = content_type {
						request.header(http::header::CONTENT_TYPE, content_type)
					}
					else {
						request
					};
				let response = request.body(body).send().await.expect("couldn't send HTTP request");
				let response_status_code = response.status();

				replay.response_status_code = response_status_code.as_u16();

				let response =
					response.bytes_stream()
					.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
					.into_async_read();

				ClientResponse {
					status_code: response_status_code,
					body: ClientResponseBody::Recording(response, &mut replay.response_body),
				}
			},

			Client::Replaying(replays) => {
				let (i, replay) = replays.next().expect("no replay expected for this request");
				assert_eq!(path.to_string(), replay.request_url, "replay #{} does not have matching request URL", i + 1);
				assert_eq!(method, replay.request_method, "replay #{} does not have matching request method", i + 1);
				assert_eq!(body, replay.request_body, "replay #{} does not have matching request body", i + 1);
				assert_eq!(content_type, replay.request_content_type, "replay #{} does not have request content type", i + 1);
				ClientResponse {
					status_code: http::StatusCode::from_u16(replay.response_status_code).unwrap(),
					body: ClientResponseBody::Replaying(std::io::Cursor::new(replay.response_body)),
				}
			},
		}
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		match self {
			Client::Recording { replays, recorder, .. } => {
				serde_json::to_writer_pretty(&mut *recorder, &replays).expect("could not save replays");
				std::io::Write::write(&mut *recorder, b"\n").expect("could not save replays");
				std::io::Write::flush(&mut *recorder).expect("could not save replays");
			},

			Client::Replaying(replays) =>
				if let Some((i, _)) = replays.next() {
					// Skip panicking here if we're already unwinding from a panic, since the original panic is from a test
					// and will cause the test to fail anyway. If we were to also panic here, it would be a double-panic and
					// abort the process without printing the panic message.

					// clippy wants `assert!(std::thread::panicking(), ...)`, which is technically the same but harder to comprehend.
					#[allow(clippy::manual_assert)]
					if !std::thread::panicking() {
						panic!("Replay #{} was not consumed", i + 1);
					}
				},
		}
	}
}

#[derive(Debug)]
#[pin_project::pin_project]
struct ClientResponse<'a, TResponse> {
	status_code: http::StatusCode,
	#[pin]
	body: ClientResponseBody<'a, TResponse>,
}

#[derive(Debug)]
#[pin_project::pin_project(project = ClientResponseBodyProj)]
enum ClientResponseBody<'a, TResponse> {
	Recording(#[pin] TResponse, &'a mut Vec<u8>),
	Replaying(std::io::Cursor<Vec<u8>>),
}

impl<'a, TResponse> AsyncRead for ClientResponse<'a, TResponse>
where
	TResponse: AsyncRead,
{
	fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
		match self.project().body.project() {
			ClientResponseBodyProj::Recording(response, replay) => response.poll_read(cx, buf).map(|read| {
				let read = read?;
				replay.extend_from_slice(&buf[0..read]);
				Ok(read)
			}),

			ClientResponseBodyProj::Replaying(replay) => Poll::Ready(std::io::Read::read(replay, buf)),
		}
	}
}

#[pin_project::pin_project(project = MultipleValuesStreamProj)]
enum MultipleValuesStream<'a, TResponseFuture, TResponse, R> {
	ExecutingRequest {
		#[pin]
		f: TResponseFuture,
		response_body: fn(http::StatusCode) -> k8s_openapi::ResponseBody<R>,
	},
	Response {
		#[pin]
		response: ClientResponse<'a, TResponse>,
		response_body: k8s_openapi::ResponseBody<R>,
		buf: Box<[u8; 4096]>,
	},
}

impl<'a, TResponseFuture, TResponse, R> Stream for MultipleValuesStream<'a, TResponseFuture, TResponse, R> where
	TResponseFuture: Future<Output = ClientResponse<'a, TResponse>>,
	ClientResponse<'a, TResponse>: AsyncRead,
	R: k8s_openapi::Response,
{
	type Item = (R, http::StatusCode);

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		loop {
			match self.as_mut().project() {
				MultipleValuesStreamProj::ExecutingRequest { f, response_body } => match f.poll(cx) {
					Poll::Ready(response) => {
						let response_body = response_body(response.status_code);
						let buf = Box::new([0_u8; 4096]);
						self.set(MultipleValuesStream::Response {
							response,
							response_body,
							buf,
						});
					},
					Poll::Pending => return Poll::Pending,
				},

				MultipleValuesStreamProj::Response { mut response, response_body, buf } => {
					// Perform one read of the response body before trying to parse it. This ensures that bodies
					// corresponding to the `Other(Option<serde_json::Value>)` variant are fully
					// parsed and printed in case of errors.
					{
						let read = match response.as_mut().poll_read(cx, &mut buf[..]) {
							Poll::Ready(Ok(read)) => read,
							Poll::Ready(Err(err)) => panic!("{err}"),
							Poll::Pending => return Poll::Pending,
						};
						response_body.append_slice(&buf[..read]);
					}

					loop {
						match response_body.parse() {
							Ok(value) => return Poll::Ready(Some((value, response_body.status_code))),
							Err(k8s_openapi::ResponseError::NeedMoreData) => (),
							Err(err) => panic!("{err}"),
						}

						match response.as_mut().poll_read(cx, &mut buf[..]) {
							Poll::Ready(Ok(0)) if response_body.is_empty() => return Poll::Ready(None),
							Poll::Ready(Ok(0)) => panic!("unexpected EOF"),
							Poll::Ready(Ok(read)) => response_body.append_slice(&buf[..read]),
							Poll::Ready(Err(err)) => panic!("{err}"),
							Poll::Pending => return Poll::Pending,
						}
					}
				},
			}
		}
	}
}

#[derive(serde::Deserialize)]
struct KubeConfig {
	clusters: Vec<KubeConfigClusterEntry>,
	contexts: Vec<KubeConfigContextEntry>,
	#[serde(rename = "current-context")]
	current_context: String,
	users: Vec<KubeConfigUserEntry>,
}

#[derive(serde::Deserialize)]
struct KubeConfigClusterEntry {
	cluster: KubeConfigCluster,
	name: String,
}

#[derive(serde::Deserialize)]
struct KubeConfigCluster {
	#[serde(flatten)]
	certificate_authority: CertificateAuthority,
	server: String,
}

#[derive(serde::Deserialize)]
enum CertificateAuthority {
	#[serde(rename = "certificate-authority")]
	File(std::path::PathBuf),
	#[serde(rename = "certificate-authority-data")]
	Inline(String),
}

#[derive(serde::Deserialize)]
struct KubeConfigContextEntry {
	context: KubeConfigContext,
	name: String,
}

#[derive(serde::Deserialize)]
struct KubeConfigContext {
	cluster: String,
	user: String,
}

#[derive(serde::Deserialize)]
struct KubeConfigUserEntry {
	name: String,
	user: KubeConfigUser,
}

#[derive(serde::Deserialize)]
struct KubeConfigUser {
	#[serde(flatten)]
	client_certificate: ClientCertificate,
	#[serde(flatten)]
	client_key: ClientKey,
}

#[derive(serde::Deserialize)]
enum ClientCertificate {
	#[serde(rename = "client-certificate")]
	File(std::path::PathBuf),
	#[serde(rename = "client-certificate-data")]
	Inline(String),
}

#[derive(serde::Deserialize)]
enum ClientKey {
	#[serde(rename = "client-key")]
	File(std::path::PathBuf),
	#[serde(rename = "client-key-data")]
	Inline(String),
}

mod bytestring {
	pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error> where D: serde::Deserializer<'de> {
		let s: String = serde::Deserialize::deserialize(deserializer)?;
		Ok(s.into_bytes())
	}

	pub(super) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let s = std::str::from_utf8(bytes).expect("bytes are not valid utf-8");
		serde::Serialize::serialize(&s, serializer)
	}
}

mod methodstring {
	use super::http;

	pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<http::Method, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = http::Method;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(formatter, "an HTTP method name")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
				v.parse().map_err(serde::de::Error::custom)
			}
		}

		deserializer.deserialize_str(Visitor)
	}

	pub(super) fn serialize<S>(method: &http::Method, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		serde::Serialize::serialize(&method.to_string(), serializer)
	}
}

mod api_versions;

mod custom_resource_definition;

mod deserialize_leniency;

mod deployment;

mod job;

mod logs;

mod patch;

mod pod;

mod resource;

mod special_idents;

mod time;

mod watch_event;
