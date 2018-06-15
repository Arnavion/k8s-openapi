#![cfg(test)]

extern crate backtrace;
extern crate k8s_openapi;
extern crate reqwest;
extern crate serde;
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
	server: String,
}

impl Client {
	fn new() -> Result<Self, Error> {
		let kubeconfig = {
			let mut kubeconfig_path = std::env::home_dir().ok_or("can't find home directory")?;
			kubeconfig_path.push(".kube");
			kubeconfig_path.push("config");
			serde_yaml::from_reader(std::io::BufReader::new(std::fs::File::open(kubeconfig_path)?))?
		};
		let mut kubeconfig = if let serde_yaml::Value::Mapping(mapping) = kubeconfig {
			mapping
		}
		else {
			return Err(format!("malformed kubeconfig {:#?}", kubeconfig).into());
		};

		let mut clusters = {
			let clusters = kubeconfig.remove(&serde_yaml::Value::String("clusters".to_string())).ok_or_else(|| format!("malformed kubeconfig {:#?}", kubeconfig))?;
			if let serde_yaml::Value::Sequence(sequence) = clusters {
				sequence
			}
			else {
				return Err(format!("malformed kubeconfig clusters {:#?}", clusters).into());
			}
		};

		let mut cluster = {
			let cluster = clusters.pop().ok_or_else(|| format!("malformed kubeconfig clusters {:#?}", clusters))?;
			let mut cluster = if let serde_yaml::Value::Mapping(mapping) = cluster {
				mapping
			}
			else {
				return Err(format!("malformed kubeconfig cluster {:#?}", cluster).into());
			};
			let cluster = cluster.remove(&serde_yaml::Value::String("cluster".to_string())).ok_or_else(|| format!("malformed kubeconfig cluster {:#?}", cluster))?;
			if let serde_yaml::Value::Mapping(mapping) = cluster {
				mapping
			}
			else {
				return Err(format!("malformed kubeconfig cluster {:#?}", cluster).into());
			}
		};

		let certificate_authority = {
			let certificate_authority = cluster.remove(&serde_yaml::Value::String("certificate-authority".to_string())).ok_or_else(|| format!("malformed kubeconfig cluster {:#?}", cluster))?;
			let certificate_authority = if let serde_yaml::Value::String(string) = certificate_authority {
				string
			}
			else {
				return Err(format!("malformed kubeconfig certificate-authority {:#?}", certificate_authority).into());
			};

			client::x509_from_pem(certificate_authority.as_ref())?
		};

		let server = {
			let server = cluster.remove(&serde_yaml::Value::String("server".to_string())).ok_or_else(|| format!("malformed kubeconfig cluster {:#?}", cluster))?;
			if let serde_yaml::Value::String(string) = server {
				string
			}
			else {
				return Err(format!("malformed kubeconfig server {:#?}", server).into());
			}
		};

		let mut users = {
			let users = kubeconfig.remove(&serde_yaml::Value::String("users".to_string())).ok_or_else(|| format!("malformed kubeconfig {:#?}", kubeconfig))?;
			if let serde_yaml::Value::Sequence(sequence) = users {
				sequence
			}
			else {
				return Err(format!("malformed kubeconfig users {:#?}", users).into());
			}
		};

		let mut user = {
			let user = users.pop().ok_or_else(|| format!("malformed kubeconfig users {:#?}", users))?;
			let mut user = if let serde_yaml::Value::Mapping(mapping) = user {
				mapping
			}
			else {
				return Err(format!("malformed kubeconfig user {:#?}", user).into());
			};
			let user = user.remove(&serde_yaml::Value::String("user".to_string())).ok_or_else(|| format!("malformed kubeconfig user {:#?}", user))?;
			if let serde_yaml::Value::Mapping(mapping) = user {
				mapping
			}
			else {
				return Err(format!("malformed kubeconfig user {:#?}", user).into());
			}
		};

		let client_certificate = {
			let client_certificate = user.remove(&serde_yaml::Value::String("client-certificate".to_string())).ok_or_else(|| format!("malformed kubeconfig user {:#?}", user))?;
			if let serde_yaml::Value::String(string) = client_certificate {
				string
			}
			else {
				return Err(format!("malformed kubeconfig client-certificate {:#?}", client_certificate).into());
			}
		};

		let client_key = {
			let client_key = user.remove(&serde_yaml::Value::String("client-key".to_string())).ok_or_else(|| format!("malformed kubeconfig user {:#?}", user))?;
			let client_key = if let serde_yaml::Value::String(string) = client_key {
				string
			}
			else {
				return Err(format!("malformed kubeconfig client-key {:#?}", client_key).into());
			};

			client::pkcs12(client_certificate.as_ref(), client_key.as_ref())?
		};

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
		let mut url = self.server.clone();
		url.push_str(path);

		let mut response =
			self.inner
			.get(&url)
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

	fn post<T>(&self, path: &str, object: &T) -> Result<T, Error> where T: serde::Serialize, for<'de> T: serde::Deserialize<'de> {
		let mut url = self.server.clone();
		url.push_str(path);

		let mut response =
			self.inner
			.post(&url)
			.header(reqwest::header::Accept::json())
			.json(object)
			.send()?;

		let status = response.status();
		if status != reqwest::StatusCode::Created {
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

		Ok(response.json()?)
	}
}

mod deployment;

mod job;

mod pod;
