#![cfg(test)]

extern crate backtrace;
extern crate k8s_openapi;
extern crate http;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

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
	server: reqwest::Url,
}

impl Client {
	fn new() -> Result<Self, Error> {
		let kubeconfig: KubeConfig = {
			let mut kubeconfig_path = std::env::home_dir().ok_or("can't find home directory")?;
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

		let server = server.parse().map_err(|err| format!("couldn't parse server URL: {}", err))?;

		let KubeConfigUser { client_certificate, client_key } =
			kubeconfig.users.into_iter()
			.find(|u| u.name == user).unwrap_or_else(|| panic!("couldn't find user named {}", user))
			.user;

		let client_key = client::pkcs12(&client_certificate, &client_key)?;

		let mut inner = reqwest::Client::builder();
		inner.danger_disable_hostname_verification();
		inner.add_root_certificate(reqwest::Certificate::from_der(&certificate_authority)?);
		inner.identity(reqwest::Identity::from_pkcs12_der(&client_key, "")?);
		let inner = inner.build()?;

		Ok(Client {
			inner,
			server,
		})
	}

	fn get<T>(&self, path: &str) -> Result<T, Error> where for<'de> T: serde::Deserialize<'de> {
		let url = self.server.join(path)?;

		let mut response =
			self.inner
			.get(url)
			.header(reqwest::header::Accept::json())
			.send()?;

		let status = response.status();
		if status != reqwest::StatusCode::Ok {
			Err(status.to_string())?;
		}

		match response.headers().get() {
			Some(reqwest::header::ContentType(mime)) if *mime == reqwest::mime::APPLICATION_JSON =>
				(),
			Some(reqwest::header::ContentType(mime)) =>
				Err(format!("Unexpected Content-Type header: {}", mime))?,
			None =>
				Err("No Content-Type header")?,
		}

		Ok(response.json()?)
	}

	fn delete(&self, path: &str) -> Result<(), Error> {
		let url = self.server.join(path)?;

		let mut response =
			self.inner
			.delete(url)
			.header(reqwest::header::Accept::json())
			.send()?;

		let status = response.status();
		if status != reqwest::StatusCode::Ok {
			Err(format!("{} {}", status.to_string(), response.text()?))?;
		}

		match response.headers().get() {
			Some(reqwest::header::ContentType(mime)) if *mime == reqwest::mime::APPLICATION_JSON =>
				(),
			Some(reqwest::header::ContentType(mime)) =>
				Err(format!("Unexpected Content-Type header: {}", mime))?,
			None =>
				Err("No Content-Type header")?,
		}

		Ok(())
	}
}

impl k8s_openapi::Client for Client {
	type Response = reqwest::Response;
	type Error = reqwest::Error;

	fn base_url(&self) -> &reqwest::Url {
		&self.server
	}

	fn delete(&self, url: reqwest::Url) -> Result<Self::Response, Self::Error> {
		let response =
			self.inner
			.delete(url)
			.send()?;
		Ok(response)
	}

	fn get(&self, url: reqwest::Url) -> Result<Self::Response, Self::Error> {
		let response =
			self.inner
			.get(url)
			.send()?;
		Ok(response)
	}

	fn patch<B>(&self, url: reqwest::Url, body: &B) -> Result<Self::Response, Self::Error> where B: serde::Serialize {
		let response =
			self.inner
			.patch(url)
			.json(body)
			.send()?;
		Ok(response)
	}

	fn post<B>(&self, url: reqwest::Url, body: &B) -> Result<Self::Response, Self::Error> where B: serde::Serialize {
		let response =
			self.inner
			.post(url)
			.json(body)
			.send()?;
		Ok(response)
	}

	fn put<B>(&self, url: reqwest::Url, body: &B) -> Result<Self::Response, Self::Error> where B: serde::Serialize {
		let response =
			self.inner
			.put(url)
			.json(body)
			.send()?;
		Ok(response)
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

mod deployment;

mod job;

mod logs;

mod pod;

mod special_idents;

mod watch_event;
