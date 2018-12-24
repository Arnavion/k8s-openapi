#![cfg(test)]

#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::indexing_slicing,
	clippy::unseparated_literal_suffix,
)]

#[macro_use] extern crate k8s_openapi;

use k8s_openapi::{http, serde_json};

#[cfg_attr(windows, path = "client_winapi.rs")]
#[cfg_attr(not(windows), path = "client_openssl.rs")]
mod client;

struct Error(Box<dyn std::error::Error>, backtrace::Backtrace);

impl<E> From<E> for Error where E: Into<Box<dyn std::error::Error>> {
	fn from(value: E) -> Self {
		Error(value.into(), backtrace::Backtrace::new())
	}
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.0)?;
		write!(f, "{:?}", self.1)?;
		Ok(())
	}
}

#[derive(Debug)]
enum Client {
	Recording {
		inner: reqwest::Client,
		server: http::Uri,
		replays: Vec<Replay>,
		recorder: std::io::BufWriter<std::fs::File>,
	},

	Replaying(std::vec::IntoIter<Replay>),
}

#[derive(Debug, serde_derive::Deserialize, serde_derive::Serialize)]
struct Replay {
	request_url: String,
	#[serde(with = "bytestring")]
	request_body: Vec<u8>,
	response_status_code: u16,
	#[serde(with = "bytestring")]
	response_body: Vec<u8>,
}

impl Client {
	fn with<F>(test_name: &'static str, f: F) where F: FnOnce(&mut Self) {
		#[cfg(feature = "test_v1_8")] let replays_directory = "v1-8";
		#[cfg(feature = "test_v1_9")] let replays_directory = "v1-9";
		#[cfg(feature = "test_v1_10")] let replays_directory = "v1-10";
		#[cfg(feature = "test_v1_11")] let replays_directory = "v1-11";
		#[cfg(feature = "test_v1_12")] let replays_directory = "v1-12";
		#[cfg(feature = "test_v1_13")] let replays_directory = "v1-13";

		let replay_filename =
			std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR")))
			.join("test-replays")
			.join(replays_directory)
			.join(format!("{}.json", test_name));

		let mut client =
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
					.find(|c| c.name == context).unwrap_or_else(|| panic!("couldn't find context named {}", context))
					.context;

				let KubeConfigCluster { certificate_authority, server } =
					kubeconfig.clusters.into_iter()
					.find(|c| c.name == cluster).unwrap_or_else(|| panic!("couldn't find cluster named {}", cluster))
					.cluster;

				let certificate_authority = client::x509_from_pem(&certificate_authority).expect("couldn't parse CA cert");

				let server: http::Uri = http::HttpTryFrom::try_from(server).expect("couldn't parse server URL");
				if let Some(path_and_query) = server.path_and_query() {
					if path_and_query != "/" {
						panic!("server URL {} has path and query {}", server, path_and_query);
					}
				}

				let KubeConfigUser { client_certificate, client_key } =
					kubeconfig.users.into_iter()
					.find(|u| u.name == user).unwrap_or_else(|| panic!("couldn't find user named {}", user))
					.user;

				let client_key = client::pkcs12(&client_certificate, &client_key).expect("couldn't parse client key");

				let inner =
					reqwest::Client::builder()
					.danger_accept_invalid_hostnames(true)
					.add_root_certificate(reqwest::Certificate::from_der(&certificate_authority).expect("couldn't parse CA cert"))
					.identity(reqwest::Identity::from_pkcs12_der(&client_key, "").expect("couldn't parse client key"))
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

				Client::Replaying(replays.into_iter())
			};

		f(&mut client);

		match client {
			Client::Replaying(mut replays) => if replays.next().is_some() {
				panic!("One or more replays were not consumed");
			},

			Client::Recording { replays, mut recorder, .. } => {
				serde_json::to_writer_pretty(&mut recorder, &replays).expect("could not save replays");
				std::io::Write::write(&mut recorder, &[b'\n']).expect("could not save replays");
			},
		}
	}

	fn execute<'a>(&'a mut self, request: http::Request<Vec<u8>>) -> Result<ClientResponse<'a>, Error> {
		let (method, path, body) = {
			let (parts, body) = request.into_parts();
			let mut url: http::uri::Parts = parts.uri.into();
			let path = url.path_and_query.take().expect("request doesn't have path and query");

			(parts.method, path, body)
		};

		match self {
			Client::Recording { inner, server, replays, .. } => {
				replays.push(Replay {
					request_url: path.to_string(),
					request_body: body.clone(),
					response_status_code: 0,
					response_body: vec![],
				});

				let mut replay = replays.last_mut().unwrap();

				let mut url: http::uri::Parts = server.clone().into();
				url.path_and_query = Some(path);
				let url = http::Uri::from_parts(url)?.to_string();

				let response = inner.request(method, &url).body(body).send()?;
				replay.response_status_code = response.status().as_u16();

				Ok(ClientResponse {
					status_code: response.status(),
					body: ClientResponseBody::Recording(response, &mut replay.response_body),
				})
			},

			Client::Replaying(replays) => {
				let replay = replays.next().expect("no replay expected for this request");
				assert_eq!(path.to_string(), replay.request_url);
				assert_eq!(body, replay.request_body);
				Ok(ClientResponse {
					status_code: http::StatusCode::from_u16(replay.response_status_code).unwrap(),
					body: ClientResponseBody::Replaying(std::io::Cursor::new(replay.response_body)),
				})
			},
		}
	}

	fn sleep(&self, duration: std::time::Duration) {
		match self {
			Client::Recording { .. } => std::thread::sleep(duration),
			Client::Replaying(_) => (),
		}
	}
}

#[derive(Debug)]
struct ClientResponse<'a> {
	status_code: http::StatusCode,
	body: ClientResponseBody<'a>,
}

#[derive(Debug)]
enum ClientResponseBody<'a> {
	Recording(reqwest::Response, &'a mut Vec<u8>),
	Replaying(std::io::Cursor<Vec<u8>>),
}

impl<'a> std::io::Read for ClientResponse<'a> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		match &mut self.body {
			ClientResponseBody::Recording(response, replay) => {
				let read = response.read(buf)?;
				replay.extend_from_slice(&buf[0..read]);
				Ok(read)
			},

			ClientResponseBody::Replaying(replay) => replay.read(buf),
		}
	}
}

enum ValueResult<T> {
	GotValue(T),

	#[allow(dead_code)]
	NeedMoreData,
}

fn get_single_value<'a, R, F, T>(response: ClientResponse<'a>, f: F) -> Result<T, Error> where
	R: k8s_openapi::Response,
	F: FnMut(R, http::StatusCode, &[u8]) -> Result<ValueResult<T>, Error>,
{
	get_multiple_values(response, f)?.next().unwrap_or_else(|| Err("unexpected EOF".into()))
}

fn get_multiple_values<'a, R, F, T>(response: ClientResponse<'a>, f: F) -> Result<MultipleValuesIterator<'a, R, F, T>, Error> where
	R: k8s_openapi::Response,
	F: FnMut(R, http::StatusCode, &[u8]) -> Result<ValueResult<T>, Error>,
{
	let response_body = k8s_openapi::ResponseBody::new(response.status_code);

	let buf = Box::new([0u8; 4096]);

	Ok(MultipleValuesIterator {
		response,
		f,
		response_body,
		buf,
		_pd: Default::default(),
	})
}

struct MultipleValuesIterator<'a, R, F, T> {
	response: ClientResponse<'a>,
	f: F,
	response_body: k8s_openapi::ResponseBody,
	buf: Box<[u8; 4096]>,
	_pd: std::marker::PhantomData<(R, T)>,
}

impl<'a, R, F, T> Iterator for MultipleValuesIterator<'a, R, F, T> where
	R: k8s_openapi::Response,
	F: FnMut(R, http::StatusCode, &[u8]) -> Result<ValueResult<T>, Error>,
{
	type Item = Result<T, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.response_body.is_empty() {
			match std::io::Read::read(&mut self.response, &mut *self.buf) {
				Ok(0) => return None,
				Ok(read) => self.response_body.append_slice(&self.buf[..read]),
				Err(err) => return Some(Err(err.into())),
			}
		}

		loop {
			match self.response_body.parse() {
				Ok(value) => match (self.f)(value, self.response_body.status_code, &*self.response_body) {
					Ok(ValueResult::GotValue(result)) => return Some(Ok(result)),
					Ok(ValueResult::NeedMoreData) => (),
					Err(err) => return Some(Err(err)),
				},
				Err(k8s_openapi::ResponseError::NeedMoreData) => (),
				Err(err) => return Some(Err(err.into())),
			}

			match std::io::Read::read(&mut self.response, &mut *self.buf) {
				Ok(0) if self.response_body.is_empty() => return None,
				Ok(0) => return Some(Err("unexpected EOF".into())),
				Ok(read) => self.response_body.append_slice(&self.buf[..read]),
				Err(err) => return Some(Err(err.into())),
			}
		}
	}
}

#[derive(serde_derive::Deserialize)]
struct KubeConfig {
	clusters: Vec<KubeConfigClusterEntry>,
	contexts: Vec<KubeConfigContextEntry>,
	#[serde(rename = "current-context")]
	current_context: String,
	users: Vec<KubeConfigUserEntry>,
}

#[derive(serde_derive::Deserialize)]
struct KubeConfigClusterEntry {
	cluster: KubeConfigCluster,
	name: String,
}

#[derive(serde_derive::Deserialize)]
struct KubeConfigCluster {
	#[serde(rename = "certificate-authority")]
	certificate_authority: std::path::PathBuf,
	server: String,
}

#[derive(serde_derive::Deserialize)]
struct KubeConfigContextEntry {
	context: KubeConfigContext,
	name: String,
}

#[derive(serde_derive::Deserialize)]
struct KubeConfigContext {
	cluster: String,
	user: String,
}

#[derive(serde_derive::Deserialize)]
struct KubeConfigUserEntry {
	name: String,
	user: KubeConfigUser,
}

#[derive(serde_derive::Deserialize)]
struct KubeConfigUser {
	#[serde(rename = "client-certificate")]
	client_certificate: std::path::PathBuf,
	#[serde(rename = "client-key")]
	client_key: std::path::PathBuf,
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

mod api_versions;

mod custom_resource_definition;

mod deployment;

mod job;

mod logs;

mod pod;

mod special_idents;

mod watch_event;
