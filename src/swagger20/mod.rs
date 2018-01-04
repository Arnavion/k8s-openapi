mod definitions;
pub use self::definitions::*;

mod info;
pub use self::info::*;

#[derive(Debug)]
pub struct Spec {
	pub info: Info,
	pub definitions: ::std::collections::BTreeMap<DefinitionPath, Schema>,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for Spec {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		#[derive(Debug, ::serde_derive::Deserialize)]
		pub struct InnerSpec {
			swagger: String,
			info: Info,
			definitions: ::std::collections::BTreeMap<DefinitionPath, Schema>,
		}

		let result: InnerSpec = ::serde::Deserialize::deserialize(deserializer)?;

		if result.swagger != "2.0" {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&result.swagger), &"2.0"));
		}

		Ok(Spec {
			info: result.info,
			definitions: result.definitions,
		})
	}
}
