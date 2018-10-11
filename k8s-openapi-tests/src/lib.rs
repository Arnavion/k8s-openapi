#![cfg(test)]

#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(
	default_trait_access,
	indexing_slicing,
	unseparated_literal_suffix,
))]

extern crate backtrace;
extern crate dirs;
#[macro_use] extern crate k8s_openapi;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

use k8s_openapi::http;
use k8s_openapi::serde_json;

#[cfg(windows)] #[path = "client_winapi.rs"] mod client;
#[cfg(not(windows))] #[path = "client_openssl.rs"] mod client;

struct Error(Box<std::error::Error>, backtrace::Backtrace);

impl<E> From<E> for Error where E: Into<Box<std::error::Error>> {
	fn from(value: E) -> Self {
		Error(value.into(), backtrace::Backtrace::new())
	}
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "{}", self.0)?;
		write!(f, "{:?}", self.1)?;
		Ok(())
	}
}

#[derive(Debug)]
struct Client {
	inner: reqwest::Client,
	server: http::Uri,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl Client {
	fn new() -> Result<Self, Error> {
		let kubeconfig: KubeConfig = {
			let mut kubeconfig_path = dirs::home_dir().ok_or("can't find home directory")?;
			kubeconfig_path.push(".kube");
			kubeconfig_path.push("config");
			serde_yaml::from_reader(std::io::BufReader::new(std::fs::File::open(kubeconfig_path)?))?
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

		let certificate_authority = client::x509_from_pem(&certificate_authority)?;

		let server: http::Uri = http::HttpTryFrom::try_from(server).map_err(|err| format!("couldn't parse server URL: {}", err))?;
		if let Some(path_and_query) = server.path_and_query() {
			if path_and_query != "/" {
				return Err(format!("server URL {} has path and query {}", server, path_and_query).into());
			}
		}

		let KubeConfigUser { client_certificate, client_key } =
			kubeconfig.users.into_iter()
			.find(|u| u.name == user).unwrap_or_else(|| panic!("couldn't find user named {}", user))
			.user;

		let client_key = client::pkcs12(&client_certificate, &client_key)?;

		let inner =
			reqwest::Client::builder()
			.danger_accept_invalid_hostnames(true)
			.add_root_certificate(reqwest::Certificate::from_der(&certificate_authority)?)
			.identity(reqwest::Identity::from_pkcs12_der(&client_key, "")?)
			.build()?;

		Ok(Client {
			inner,
			server,
		})
	}

	fn execute(&self, request: http::Request<Vec<u8>>) -> Result<reqwest::Response, Error> {
		let (method, url, body) = {
			let (mut parts, body) = request.into_parts();
			let mut url: http::uri::Parts = parts.uri.into();
			let path = url.path_and_query.take().expect("request doesn't have path and query");
			let mut url: http::uri::Parts = self.server.clone().into();
			url.path_and_query = Some(path);
			let url = http::Uri::from_parts(url)?;

			(parts.method, url.to_string(), body)
		};

		Ok(self.inner.request(method, &url).body(body).send()?)
	}
}

enum ValueResult<T> {
	GotValue(T),
	NeedMoreData,
}

fn get_single_value<R, F, T>(response: reqwest::Response, f: F) -> Result<T, Error> where
	R: k8s_openapi::Response,
	F: FnMut(R, http::StatusCode, &[u8]) -> Result<ValueResult<T>, Error>,
{
	get_multiple_values(response, f)?.next().unwrap_or_else(|| Err("unexpected EOF".into()))
}

fn get_multiple_values<R, F, T>(response: reqwest::Response, f: F) -> Result<MultipleValuesIterator<R, F, T>, Error> where
	R: k8s_openapi::Response,
	F: FnMut(R, http::StatusCode, &[u8]) -> Result<ValueResult<T>, Error>,
{
	let status_code = response.status();

	let response_body = k8s_openapi::ResponseBody::new(status_code);

	let buf = Box::new([0u8; 4096]);

	Ok(MultipleValuesIterator {
		response,
		f,
		response_body,
		buf,
		_pd: Default::default(),
	})
}

struct MultipleValuesIterator<R, F, T> {
	response: reqwest::Response,
	f: F,
	response_body: k8s_openapi::ResponseBody,
	buf: Box<[u8; 4096]>,
	_pd: std::marker::PhantomData<(R, T)>,
}

impl<R, F, T> Iterator for MultipleValuesIterator<R, F, T> where
	R: k8s_openapi::Response,
	F: FnMut(R, http::StatusCode, &[u8]) -> Result<ValueResult<T>, Error>,
{
	type Item = Result<T, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let read = match std::io::Read::read(&mut self.response, &mut *self.buf) {
				Ok(0) if self.response_body.is_empty() => return None,
				Ok(0) => return Some(Err("unexpected EOF".into())),
				Ok(read) => read,
				Err(err) => return Some(Err(err.into())),
			};

			let response = match self.response_body.append_slice_and_parse(&self.buf[..read]) {
				Ok(value) => value,
				Err(k8s_openapi::ResponseError::NeedMoreData) => continue,
				Err(err) => return Some(Err(err.into())),
			};

			match (self.f)(response, self.response_body.status_code, &*self.response_body) {
				Ok(ValueResult::GotValue(result)) => return Some(Ok(result)),
				Ok(ValueResult::NeedMoreData) => (),
				Err(err) => return Some(Err(err)),
			}
		}
	}
}

#[derive(Deserialize)]
struct KubeConfig {
	clusters: Vec<KubeConfigClusterEntry>,
	contexts: Vec<KubeConfigContextEntry>,
	#[serde(rename = "current-context")]
	current_context: String,
	users: Vec<KubeConfigUserEntry>,
}

#[derive(Deserialize)]
struct KubeConfigClusterEntry {
	cluster: KubeConfigCluster,
	name: String,
}

#[derive(Deserialize)]
struct KubeConfigCluster {
	#[serde(rename = "certificate-authority")]
	certificate_authority: std::path::PathBuf,
	server: String,
}

#[derive(Deserialize)]
struct KubeConfigContextEntry {
	context: KubeConfigContext,
	name: String,
}

#[derive(Deserialize)]
struct KubeConfigContext {
	cluster: String,
	user: String,
}

#[derive(Deserialize)]
struct KubeConfigUserEntry {
	name: String,
	user: KubeConfigUser,
}

#[derive(Deserialize)]
struct KubeConfigUser {
	#[serde(rename = "client-certificate")]
	client_certificate: std::path::PathBuf,
	#[serde(rename = "client-key")]
	client_key: std::path::PathBuf,
}

mod api_versions;

k8s_if_ge_1_8! {
	// CRDs not supported in v1.7
	mod custom_resource_definition;
}

mod deployment;

mod job;

mod logs;

mod pod;

mod special_idents;

mod watch_event;
