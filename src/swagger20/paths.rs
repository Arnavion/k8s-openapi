#[derive(Clone, Copy, Debug)]
pub enum KubernetesAction {
	Connect,
	Delete,
	DeleteCollection,
	Get,
	List,
	Patch,
	Post,
	Proxy,
	Put,
	Watch,
	WatchList,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for KubernetesAction {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> ::serde::de::Visitor<'de> for Visitor {
			type Value = KubernetesAction;

			fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				write!(f, "x-kubernetes-action")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
				Ok(match v {
					"connect" => KubernetesAction::Connect,
					"delete" => KubernetesAction::Delete,
					"deletecollection" => KubernetesAction::DeleteCollection,
					"get" => KubernetesAction::Get,
					"list" => KubernetesAction::List,
					"patch" => KubernetesAction::Patch,
					"post" => KubernetesAction::Post,
					"proxy" => KubernetesAction::Proxy,
					"put" => KubernetesAction::Put,
					"watch" => KubernetesAction::Watch,
					"watchlist" => KubernetesAction::WatchList,
					v => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(v), &self)),
				})
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
	Delete,
	Get,
	Patch,
	Post,
	Put,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for Method {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> ::serde::de::Visitor<'de> for Visitor {
			type Value = Method;

			fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				write!(f, "string representation of HTTP method")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
				Ok(match v {
					"delete" => Method::Delete,
					"Get" => Method::Get,
					"Patch" => Method::Patch,
					"Post" => Method::Post,
					"Put" => Method::Put,
					v => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(v), &self)),
				})
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

#[derive(Debug)]
pub struct Operation {
	pub description: Option<String>,
	pub method: Method,
	pub id: String,
	pub kubernetes_action: Option<KubernetesAction>,
	pub kubernetes_group_kind_version: Option<super::KubernetesGroupKindVersion>,
	pub parameters: Vec<Parameter>,
	pub responses: ::std::collections::BTreeMap<::reqwest::StatusCode, Option<super::Schema>>,
}

#[derive(Debug)]
pub struct Parameter {
	pub location: ParameterLocation,
	pub name: String,
	pub required: bool,
	pub schema: super::Schema,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for Parameter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		#[derive(Debug, Deserialize)]
		struct InnerParameter {
			description: Option<String>,
			#[serde(rename = "in")]
			location: String,
			name: String,
			required: Option<bool>,
			#[serde(rename = "type")]
			ty: Option<String>,
			schema: Option<super::Schema>,
		}

		let value: InnerParameter = ::serde::Deserialize::deserialize(deserializer)?;

		let (location, schema) = match (&*value.location, value.ty, value.schema) {
			("body", None, Some(schema)) => (
				ParameterLocation::Body,
				super::Schema {
					description: value.description,
					..schema
				},
			),

			("path", Some(ty), None) => (
				ParameterLocation::Path,
				super::Schema {
					description: value.description,
					kind: super::SchemaKind::Ty(super::Type::parse::<D>(&ty, None, None, None)?),
					kubernetes_group_kind_versions: None,
				},
			),

			("query", Some(ty), None) => (
				ParameterLocation::Query,
				super::Schema {
					description: value.description,
					kind: super::SchemaKind::Ty(super::Type::parse::<D>(&ty, None, None, None)?),
					kubernetes_group_kind_versions: None,
				},
			),

			_ => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&*value.location), &"a parameter location")),
		};

		let required = match (value.required, location) {
			(_, ParameterLocation::Path) => true,
			(Some(required), _) => required,
			(None, _) => false,
		};

		Ok(Parameter {
			location,
			name: value.name,
			required,
			schema,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterLocation {
	Body,
	Path,
	Query,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct Path(pub String);

impl ::std::ops::Deref for Path {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl ::std::fmt::Display for Path {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		self.0.fmt(f)
	}
}

#[derive(Debug)]
pub struct PathItem {
	pub operations: Vec<Operation>,
	pub parameters: Vec<Parameter>,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for PathItem {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		#[derive(Debug, Deserialize)]
		struct InnerPathItem {
			delete: Option<InnerOperation>,
			get: Option<InnerOperation>,
			patch: Option<InnerOperation>,
			post: Option<InnerOperation>,
			put: Option<InnerOperation>,
			#[serde(default)]
			parameters: Vec<Parameter>,
		}

		#[derive(Debug, Deserialize)]
		struct InnerOperation {
			description: Option<String>,
			#[serde(rename = "operationId")]
			id: String,
			#[serde(rename = "x-kubernetes-action")]
			kubernetes_action: Option<KubernetesAction>,
			#[serde(rename = "x-kubernetes-group-version-kind")]
			kubernetes_group_kind_version: Option<super::KubernetesGroupKindVersion>,
			#[serde(default)]
			parameters: Vec<Parameter>,
			responses: ::std::collections::BTreeMap<String, InnerResponse>,
		}

		#[derive(Debug, Deserialize)]
		struct InnerResponse {
			schema: Option<super::Schema>,
		}

		fn parse_operation<'de, D>(value: InnerOperation, method: Method) -> Result<Operation, D::Error> where D: ::serde::Deserializer<'de> {
			let responses: Result<_, _> =
				value.responses.into_iter()
				.map(|(status_code_str, response)| {
					let status_code = status_code_str.parse().map_err(|_|
						::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&status_code_str), &"string representation of an HTTP status code"))?;
					Ok((status_code, response.schema))
				})
				.collect();

			Ok(Operation {
				description: value.description,
				id: value.id,
				kubernetes_action: value.kubernetes_action,
				kubernetes_group_kind_version: value.kubernetes_group_kind_version,
				method,
				parameters: value.parameters,
				responses: responses?,
			})
		}

		let value: InnerPathItem = ::serde::Deserialize::deserialize(deserializer)?;

		let mut operations = vec![];

		if let Some(delete) = value.delete {
			operations.push(parse_operation::<D>(delete, Method::Delete)?);
		}

		if let Some(get) = value.get {
			operations.push(parse_operation::<D>(get, Method::Get)?);
		}

		if let Some(patch) = value.patch {
			operations.push(parse_operation::<D>(patch, Method::Patch)?);
		}

		if let Some(post) = value.post {
			operations.push(parse_operation::<D>(post, Method::Post)?);
		}

		if let Some(put) = value.put {
			operations.push(parse_operation::<D>(put, Method::Put)?);
		}

		Ok(PathItem {
			operations,
			parameters: value.parameters,
		})
	}
}
