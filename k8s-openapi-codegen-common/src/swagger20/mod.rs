mod definitions;
pub use self::definitions::*;

mod info;
pub use self::info::*;

mod paths;
pub use self::paths::*;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct KubernetesGroupKindVersion {
	pub group: String,
	pub kind: String,
	pub version: String,
}

impl std::cmp::Ord for KubernetesGroupKindVersion {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.group.cmp(&other.group)
		.then_with(|| self.version.cmp(&other.version))
		.then_with(|| self.kind.cmp(&other.kind))
	}
}

impl std::cmp::PartialOrd for KubernetesGroupKindVersion {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug)]
pub struct Spec {
	pub info: Info,
	pub definitions: std::collections::BTreeMap<DefinitionPath, Schema>,
	pub operations: Vec<Operation>,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for Spec {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		#[derive(Debug, serde::Deserialize)]
		pub struct InnerSpec {
			swagger: String,
			info: Info,
			definitions: std::collections::BTreeMap<DefinitionPath, Schema>,
			paths: std::collections::BTreeMap<Path, InnerPathItem>,
		}

		#[derive(Debug, serde::Deserialize)]
		struct InnerPathItem {
			delete: Option<InnerOperation>,
			get: Option<InnerOperation>,
			patch: Option<InnerOperation>,
			post: Option<InnerOperation>,
			put: Option<InnerOperation>,
			#[serde(default)]
			parameters: Vec<std::sync::Arc<Parameter>>,
		}

		#[derive(Debug, serde::Deserialize)]
		struct InnerOperation {
			description: Option<String>,
			#[serde(rename = "operationId")]
			id: String,
			#[serde(rename = "x-kubernetes-action")]
			kubernetes_action: Option<KubernetesAction>,
			#[serde(rename = "x-kubernetes-group-version-kind")]
			kubernetes_group_kind_version: Option<KubernetesGroupKindVersion>,
			#[serde(default)]
			parameters: Vec<std::sync::Arc<Parameter>>,
			responses: std::collections::BTreeMap<String, InnerResponse>,
			tags: Option<(String,)>,
		}

		#[derive(Debug, serde::Deserialize)]
		struct InnerResponse {
			schema: Option<Schema>,
		}

		fn parse_operation<'de, D>(
			value: InnerOperation,
			method: Method,
			path: Path,
			mut parameters: Vec<std::sync::Arc<Parameter>>,
		) -> Result<Operation, D::Error> where D: serde::Deserializer<'de> {
			let responses: Result<_, _> =
				value.responses.into_iter()
				.filter_map(|(status_code_str, response)| {
					let status_code = match status_code_str.parse() {
						Ok(status_code) => status_code,
						Err(_) => return Some(Err(serde::de::Error::invalid_value(
							serde::de::Unexpected::Str(&status_code_str),
							&"string representation of an HTTP status code"))),
					};
					let schema = response.schema?;
					Some(Ok((status_code, schema)))
				})
				.collect();

			for parameter in value.parameters {
				if let Some(p) = parameters.iter_mut().find(|p| p.name == parameter.name) {
					*p = parameter;
				}
				else {
					parameters.push(parameter);
				}
			}

			if method == Method::Get {
				for parameter in &parameters {
					if parameter.location == ParameterLocation::Body {
						return Err(serde::de::Error::custom(format!("Operation {} has method GET but has a body parameter {}", value.id, parameter.name)));
					}
				}
			}

			Ok(Operation {
				description: value.description,
				id: value.id,
				path,
				kubernetes_action: value.kubernetes_action,
				kubernetes_group_kind_version: value.kubernetes_group_kind_version,
				method,
				parameters,
				responses: OperationResponses::Map(responses?),
				tag: value.tags.map(|t|t.0),
			})
		}

		let result: InnerSpec = serde::Deserialize::deserialize(deserializer)?;

		if result.swagger != "2.0" {
			return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&result.swagger), &"2.0"));
		}

		let mut operations = vec![];

		for (path, path_item) in result.paths {
			if let Some(delete) = path_item.delete {
				operations.push(parse_operation::<D>(delete, Method::Delete, path.clone(), path_item.parameters.clone())?);
			}

			if let Some(get) = path_item.get {
				operations.push(parse_operation::<D>(get, Method::Get, path.clone(), path_item.parameters.clone())?);
			}

			if let Some(patch) = path_item.patch {
				operations.push(parse_operation::<D>(patch, Method::Patch, path.clone(), path_item.parameters.clone())?);
			}

			if let Some(post) = path_item.post {
				operations.push(parse_operation::<D>(post, Method::Post, path.clone(), path_item.parameters.clone())?);
			}

			if let Some(put) = path_item.put {
				operations.push(parse_operation::<D>(put, Method::Put, path, path_item.parameters)?);
			}
		}

		let mut operation_ids: std::collections::BTreeSet<_> = Default::default();
		for operation in &operations {
			assert!(operation_ids.insert(&operation.id));
		}

		Ok(Spec {
			info: result.info,
			definitions: result.definitions,
			operations,
		})
	}
}
