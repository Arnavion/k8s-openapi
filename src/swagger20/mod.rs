mod definitions;
pub use self::definitions::*;

mod info;
pub use self::info::*;

mod paths;
pub use self::paths::*;

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct KubernetesGroupKindVersion {
	pub group: String,
	pub kind: String,
	pub version: String,
}

#[derive(Debug)]
pub struct Spec {
	pub info: Info,
	pub definitions: ::std::collections::BTreeMap<DefinitionPath, Schema>,
	pub paths: ::std::collections::BTreeMap<Path, PathItem>,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for Spec {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		#[derive(Debug, Deserialize)]
		pub struct InnerSpec {
			swagger: String,
			info: Info,
			definitions: ::std::collections::BTreeMap<DefinitionPath, Schema>,
			paths: ::std::collections::BTreeMap<Path, PathItem>,
		}

		let result: InnerSpec = ::serde::Deserialize::deserialize(deserializer)?;

		if result.swagger != "2.0" {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&result.swagger), &"2.0"));
		}

		Ok(Spec {
			info: result.info,
			definitions: result.definitions,
			paths: result.paths,
		})
	}
}
