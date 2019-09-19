#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::similar_names,
	clippy::too_many_arguments,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unseparated_literal_suffix,
	clippy::use_self,
)]

//! This crate contains common code for the [`k8s-openapi` code generator](https://github.com/Arnavion/k8s-openapi/tree/master/k8s-openapi-codegen)
//! and the [`k8s-openapi-derive`](https://crates.io/crates/k8s-openapi-derive) custom derive crate.
//!
//! WARNING: This crate is not meant to be used directly by end users and does not have a stable API.

#[doc(hidden)]
pub mod swagger20;

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct RunResult {
	pub num_generated_structs: usize,
	pub num_generated_type_aliases: usize,
	pub num_generated_apis: usize,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Error(Box<dyn std::error::Error + Send + Sync>);

impl From<&'_ str> for Error {
	fn from(err: &'_ str) -> Self {
		Error(err.into())
	}
}

impl From<String> for Error {
	fn from(err: String) -> Self {
		Error(err.into())
	}
}

impl From<std::fmt::Error> for Error {
	fn from(err: std::fmt::Error) -> Self {
		Error(err.into())
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Error(err.into())
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::error::Error for Error {
}

#[doc(hidden)]
pub fn run<W>(
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	operations: &mut Vec<swagger20::Operation>,
	definition_path: &swagger20::DefinitionPath,
	ref_path_relative_to: swagger20::RefPathRelativeTo,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	mod_root: Option<&str>,
	vis: &str,
	out: impl FnOnce(&[std::borrow::Cow<'_, str>]) -> std::io::Result<W>,
	mut imports: impl FnMut(Option<String>, Option<String>) -> std::io::Result<()>,
) -> Result<RunResult, Error> where W: std::io::Write {
	let definition = definitions.get(definition_path).ok_or_else(|| format!("definition for {} does not exist in spec", definition_path))?;

	let mut run_result = RunResult {
		num_generated_structs: 0,
		num_generated_type_aliases: 0,
		num_generated_apis: 0,
	};

	let parts = replace_namespace(definition_path.split('.'), replace_namespaces);
	let mut out = out(&parts)?;

	let type_name = parts.last().ok_or_else(|| format!("path for {} has no parts", definition_path))?.to_string();

	let type_ref_path = swagger20::RefPath {
		path: definition_path.0.clone(),
		relative_to: ref_path_relative_to,
		can_be_default: None,
	};

	writeln!(out, "// Generated from definition {}", definition_path)?;
	writeln!(out)?;

	if let Some(description) = &definition.description {
		for line in get_comment_text(description, "") {
			writeln!(out, "///{}", line)?;
		}
	}

	let can_be_default = can_be_default(&definition.kind, definitions)?;

	match &definition.kind {
		swagger20::SchemaKind::Properties(properties) => {
			struct Property<'a> {
				name: &'a swagger20::PropertyName,
				schema: &'a swagger20::Schema,
				required: bool,
				field_name: std::borrow::Cow<'static, str>,
				field_type_name: String,
			}

			let (properties, resource_metadata, metadata_property_ty) = {
				use std::fmt::Write;

				let mut result = Vec::with_capacity(properties.len());

				let single_group_version_kind =
					definition.kubernetes_group_kind_versions.as_ref()
					.and_then(|group_version_kinds| {
						if group_version_kinds.len() == 1 {
							Some(&group_version_kinds[0])
						}
						else {
							None
						}
					});
				let mut has_api_version = false;
				let mut has_kind = false;
				let mut metadata_property_ty = None;

				for (name, (schema, required)) in properties {
					if name.0 == "apiVersion" && single_group_version_kind.is_some() {
						has_api_version = true;
						continue;
					}

					if name.0 == "kind" && single_group_version_kind.is_some() {
						has_kind = true;
						continue;
					}

					let field_name = get_rust_ident(&name);

					let mut field_type_name = String::new();

					if !required {
						write!(field_type_name, "Option<")?;
					}

					let type_name = get_rust_type(&schema.kind, &replace_namespaces, crate_root, mod_root)?;

					if name.0 == "metadata" {
						metadata_property_ty = Some((*required, type_name.clone()));
					}

					// Fix cases of infinite recursion
					if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &schema.kind {
						match (&**definition_path, &**name, &**path) {
							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
								"not",
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
							) |
							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps",
								"not",
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps",
							) => write!(field_type_name, "Box<{}>", type_name)?,

							_ => write!(field_type_name, "{}", type_name)?,
						}
					}
					else {
						write!(field_type_name, "{}", type_name)?;
					};

					if !required {
						write!(field_type_name, ">")?;
					}

					result.push(Property {
						name,
						schema,
						required: *required,
						field_name,
						field_type_name,
					});
				}

				let resource_metadata = match (has_api_version, has_kind) {
					(true, true) => {
						let single_group_version_kind = single_group_version_kind.unwrap();
						if single_group_version_kind.group == "" {
							Some((
								std::borrow::Cow::Borrowed(&*single_group_version_kind.version),
								"",
								&*single_group_version_kind.kind,
								&*single_group_version_kind.version,
							))
						}
						else {
							Some((
								std::borrow::Cow::Owned(format!("{}/{}", single_group_version_kind.group, single_group_version_kind.version)),
								&*single_group_version_kind.group,
								&*single_group_version_kind.kind,
								&*single_group_version_kind.version,
							))
						}
					},
					(false, false) => None,
					(true, false) => return Err(format!("{} has an apiVersion property but not a kind property", definition_path).into()),
					(false, true) => return Err(format!("{} has a kind property but not an apiVersion property", definition_path).into()),
				};

				(result, resource_metadata, metadata_property_ty)
			};

			write!(out, "#[derive(Clone, Debug")?;

			if can_be_default {
				write!(out, ", Default")?;
			}

			writeln!(out, ", PartialEq)]")?;

			writeln!(out, "{}struct {} {{", vis, type_name)?;

			for (i, Property { schema, field_name, field_type_name, .. }) in properties.iter().enumerate() {
				if i > 0 {
					writeln!(out)?;
				}

				if let Some(description) = &schema.description {
					for line in get_comment_text(description, "") {
						writeln!(out, "    ///{}", line)?;
					}
				}

				write!(out, "    {}{}: ", vis, field_name)?;

				write!(out, "{}", field_type_name)?;

				writeln!(out, ",")?;
			}
			writeln!(out, "}}")?;

			if let Some(kubernetes_group_kind_versions) = &definition.kubernetes_group_kind_versions {
				let mut kubernetes_group_kind_versions: Vec<_> = kubernetes_group_kind_versions.iter().collect();
				kubernetes_group_kind_versions.sort();

				let mut operations_by_gkv: std::collections::BTreeMap<_, Vec<_>> = Default::default();
				for operation in std::mem::replace(operations, vec![]) {
					operations_by_gkv
						.entry(operation.kubernetes_group_kind_version.clone())
						.or_default()
						.push(operation);
				}

				for kubernetes_group_kind_version in kubernetes_group_kind_versions {
					if let Some(mut operations) = operations_by_gkv.remove(&Some(kubernetes_group_kind_version.clone())) {
						operations.sort_by(|o1, o2| o1.id.cmp(&o2.id));

						writeln!(out)?;
						writeln!(out, "// Begin {}/{}/{}",
							kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;

						for operation in operations {
							let (operation_optional_parameters_name, operation_result_name) =
								write_operation(
									&mut out,
									&operation,
									&replace_namespaces,
									crate_root,
									mod_root,
									vis,
									&mut Some((&type_name, &type_ref_path)))?;
							imports(operation_optional_parameters_name, operation_result_name)?;
							run_result.num_generated_apis += 1;
						}

						writeln!(out)?;
						writeln!(out, "// End {}/{}/{}",
							kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;
					}
				}

				*operations = operations_by_gkv.into_iter().flat_map(|(_, operations)| operations).collect();
			}

			if let Some(resource_metadata) = &resource_metadata {
				writeln!(out)?;
				writeln!(out, "impl {}::Resource for {} {{", crate_root, type_name)?;
				writeln!(out, "    fn api_version() -> &'static str {{")?;
				writeln!(out, r#"        "{}""#, resource_metadata.0)?;
				writeln!(out, "    }}")?;
				writeln!(out)?;
				writeln!(out, "    fn group() -> &'static str {{")?;
				writeln!(out, r#"        "{}""#, resource_metadata.1)?;
				writeln!(out, "    }}")?;
				writeln!(out)?;
				writeln!(out, "    fn kind() -> &'static str {{")?;
				writeln!(out, r#"        "{}""#, resource_metadata.2)?;
				writeln!(out, "    }}")?;
				writeln!(out)?;
				writeln!(out, "    fn version() -> &'static str {{")?;
				writeln!(out, r#"        "{}""#, resource_metadata.3)?;
				writeln!(out, "    }}")?;
				writeln!(out, "}}")?;

				if let Some((required, ty)) = metadata_property_ty {
					writeln!(out)?;
					writeln!(out, "impl {}::Metadata for {} {{", crate_root, type_name)?;
					writeln!(out, "    type Ty = {};", ty)?;
					writeln!(out)?;
					writeln!(out, "    fn metadata(&self) -> Option<&<Self as {}::Metadata>::Ty> {{", crate_root)?;
					if required {
						writeln!(out, "        Some(&self.metadata)")?;
					}
					else {
						writeln!(out, "        self.metadata.as_ref()")?;
					}
					writeln!(out, "    }}")?;
					writeln!(out, "}}")?;
				}
			}

			writeln!(out)?;
			writeln!(out, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
			writeln!(out, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "        #[allow(non_camel_case_types)]")?;
			writeln!(out, "        enum Field {{")?;
			if resource_metadata.is_some() {
				writeln!(out, "            Key_api_version,")?;
				writeln!(out, "            Key_kind,")?;
			}
			for Property { field_name, .. } in &properties {
				writeln!(out, "            Key_{},", field_name)?;
			}
			writeln!(out, "            Other,")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::Deserialize<'de> for Field {{")?;
			writeln!(out, "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "                struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "                impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "                    type Value = Field;")?;
			writeln!(out)?;
			writeln!(out, "                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                        write!(f, "field identifier")"#)?;
			writeln!(out, "                    }}")?;
			writeln!(out)?;
			writeln!(out, "                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                        Ok(match v {{")?;
			if resource_metadata.is_some() {
				writeln!(out, r#"                            "apiVersion" => Field::Key_api_version,"#)?;
				writeln!(out, r#"                            "kind" => Field::Key_kind,"#)?;
			}
			for Property { name, field_name, .. } in &properties {
				writeln!(out, r#"                            "{}" => Field::Key_{},"#, name, field_name)?;
			}
			writeln!(out, "                            _ => Field::Other,")?;
			writeln!(out, "                        }})")?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, "                deserializer.deserialize_identifier(Visitor)")?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "            type Value = {};", type_name)?;
			writeln!(out)?;
			writeln!(out, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                write!(f, "struct {}")"#, type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{")?;
			for Property { required, field_name, field_type_name, .. } in &properties {
				if *required {
					writeln!(out, r#"                let mut value_{}: Option<{}> = None;"#, field_name, field_type_name)?;
				}
				else {
					writeln!(out, r#"                let mut value_{}: {} = None;"#, field_name, field_type_name)?;
				}
			}
			writeln!(out)?;
			writeln!(out, "                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{")?;
			writeln!(out, "                    match key {{")?;
			if resource_metadata.is_some() {
					writeln!(out, r#"                        Field::Key_api_version => {{"#)?;
					writeln!(out, r#"                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;"#)?;
					writeln!(out, r#"                            if value_api_version != <Self::Value as {}::Resource>::api_version() {{"#, crate_root)?;
					writeln!(out, r#"                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as {}::Resource>::api_version()));"#, crate_root)?;
					writeln!(out, r#"                            }}"#)?;
					writeln!(out, r#"                        }},"#)?;

					writeln!(out, r#"                        Field::Key_kind => {{"#)?;
					writeln!(out, r#"                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;"#)?;
					writeln!(out, r#"                            if value_kind != <Self::Value as {}::Resource>::kind() {{"#, crate_root)?;
					writeln!(out, r#"                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as {}::Resource>::kind()));"#, crate_root)?;
					writeln!(out, r#"                            }}"#)?;
					writeln!(out, r#"                        }},"#)?;
			}
			for Property { required, field_name, .. } in &properties {
				if *required {
					writeln!(out, r#"                        Field::Key_{} => value_{} = Some(serde::de::MapAccess::next_value(&mut map)?),"#, field_name, field_name)?;
				}
				else {
					writeln!(out, r#"                        Field::Key_{} => value_{} = serde::de::MapAccess::next_value(&mut map)?,"#, field_name, field_name)?;
				}
			}
			writeln!(out, "                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},")?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, "                Ok({} {{", type_name)?;
			for Property { name, required, field_name, .. } in &properties {
				if *required {
					writeln!(out, r#"                    {}: value_{}.ok_or_else(|| serde::de::Error::missing_field("{}"))?,"#, field_name, field_name, name)?;
				}
				else {
					writeln!(out, "                    {}: value_{},", field_name, field_name)?;
				}
			}
			writeln!(out, "                }})")?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        deserializer.deserialize_struct(")?;
			writeln!(out, r#"            "{}","#, type_name)?;
			writeln!(out, "            &[")?;
			if resource_metadata.is_some() {
				writeln!(out, r#"                "apiVersion","#)?;
				writeln!(out, r#"                "kind","#)?;
			}
			for Property { name, .. } in &properties {
				writeln!(out, r#"                "{}","#, name)?;
			}
			writeln!(out, "            ],")?;
			writeln!(out, "            Visitor,")?;
			writeln!(out, "        )")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
			writeln!(out)?;

			writeln!(out, "impl serde::Serialize for {} {{", type_name)?;
			writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
			if properties.is_empty() && resource_metadata.is_none() {
				writeln!(out, "        let state = serializer.serialize_struct(")?;
			}
			else {
				writeln!(out, "        let mut state = serializer.serialize_struct(")?;
			}
			writeln!(out, r#"            "{}","#, type_name)?;

			let num_required_fields =
				resource_metadata.as_ref().map_or(0, |_| 2) +
				properties.iter().filter(|Property { required, .. }| *required).count();
			let have_optional_fields = properties.iter().any(|Property { required, .. }| !*required);
			match (num_required_fields, have_optional_fields) {
				(0, true) => {
					let mut wrote_first_field = false;
					for Property { required, field_name, .. } in &properties {
						if !*required {
							if wrote_first_field {
								writeln!(out, " +")?;
							}
							else {
								wrote_first_field = true;
							}
							write!(out, "            self.{}.as_ref().map_or(0, |_| 1)", field_name)?;
						}
					}
				},

				(0, false) =>
					write!(out, "            0")?,

				(num_required_fields, have_optional_fields) => {
					write!(out, "            {}", num_required_fields)?;
					if have_optional_fields {
						for Property { required, field_name, .. } in &properties {
							if !*required {
								writeln!(out, " +")?;
								write!(out, "            self.{}.as_ref().map_or(0, |_| 1)", field_name)?;
							}
						}
					}
				},
			}
			writeln!(out, ",")?;

			writeln!(out, "        )?;")?;
			if resource_metadata.is_some() {
				writeln!(out, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as {}::Resource>::api_version())?;"#, crate_root)?;
				writeln!(out, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as {}::Resource>::kind())?;"#, crate_root)?;
			}
			for Property { name, required, field_name, .. } in &properties {
				if *required {
					writeln!(out, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "{}", &self.{})?;"#, name, field_name)?;
				}
				else {
					writeln!(out, "        if let Some(value) = &self.{} {{", field_name)?;
					writeln!(out, r#"            serde::ser::SerializeStruct::serialize_field(&mut state, "{}", value)?;"#, name)?;
					writeln!(out, "        }}")?;
				}
			}
			writeln!(out, "        serde::ser::SerializeStruct::end(state)")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ref(_) => return Err(format!("{} is a Ref", definition_path).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => {
			writeln!(out, "#[derive(Clone, Debug, Eq, PartialEq)]")?;
			writeln!(out, "{}enum {} {{", vis, type_name)?;
			writeln!(out, "    Int(i32),")?;
			writeln!(out, "    String(String),")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl Default for {} {{", type_name)?;
			writeln!(out, "    fn default() -> Self {{")?;
			writeln!(out, "        {}::Int(0)", type_name)?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
			writeln!(out, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "        struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "            type Value = {};", type_name)?;
			writeln!(out)?;
			writeln!(out, "            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                write!(formatter, "enum {}")"#, type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                Ok({}::Int(v))", type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                if v < i64::from(i32::min_value()) || v > i64::from(i32::max_value()) {{")?;
			writeln!(out, r#"                    return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &"a 32-bit integer"));"#)?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, "                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]")?;
			writeln!(out, "                Ok({}::Int(v as i32))", type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                #[allow(clippy::cast_sign_loss)]")?;
			writeln!(out, "                {{")?;
			writeln!(out, "                    if v > i32::max_value() as u64 {{")?;
			writeln!(out, r#"                        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"));"#)?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, "                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]")?;
			writeln!(out, "                Ok({}::Int(v as i32))", type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                self.visit_string(v.to_owned())")?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                Ok({}::String(v))", type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        deserializer.deserialize_any(Visitor)")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl serde::Serialize for {} {{", type_name)?;
			writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
			writeln!(out, "        match self {{")?;
			writeln!(out, "            {}::Int(i) => i.serialize(serializer),", type_name)?;
			writeln!(out, "            {}::String(s) => s.serialize(serializer),", type_name)?;
			writeln!(out, "        }}")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrArray) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrBool) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrStringArray) => {
			let json_schema_props_type_name =
				get_fully_qualified_type_name(
					&swagger20::RefPath {
						path: "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps".to_owned(),
						relative_to: swagger20::RefPathRelativeTo::Crate,
						can_be_default: None,
					},
					&replace_namespaces,
					crate_root,
					mod_root)?;

			writeln!(out, "#[derive(Clone, Debug, PartialEq)]")?;
			writeln!(out, "{}enum {} {{", vis, type_name)?;
			writeln!(out, "    Schema(Box<{}>),", json_schema_props_type_name)?; // Box to fix infinite recursion
			match ty {
				swagger20::Type::JSONSchemaPropsOrArray => writeln!(out, "    Schemas(Vec<{}>),", json_schema_props_type_name)?,
				swagger20::Type::JSONSchemaPropsOrBool => writeln!(out, "    Bool(bool),")?,
				swagger20::Type::JSONSchemaPropsOrStringArray => writeln!(out, "    Strings(Vec<String>),")?,
				_ => unreachable!(),
			}
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
			writeln!(out, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "        struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "            type Value = {};", type_name)?;
			writeln!(out)?;
			writeln!(out, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                write!(f, "enum {}")"#, type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{")?;
			writeln!(out, "                Ok({}::Schema(serde::de::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))", type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;

			match ty {
				swagger20::Type::JSONSchemaPropsOrArray => {
					writeln!(out, "            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {{")?;
					writeln!(out, "                Ok({}::Schemas(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))", type_name)?;
					writeln!(out, "            }}")?;
				},

				swagger20::Type::JSONSchemaPropsOrBool => {
					writeln!(out, "            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(out, "                Ok({}::Bool(v))", type_name)?;
					writeln!(out, "            }}")?;
				},

				swagger20::Type::JSONSchemaPropsOrStringArray => {
					writeln!(out, "            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {{")?;
					writeln!(out, "                Ok({}::Strings(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))", type_name)?;
					writeln!(out, "            }}")?;
				},

				_ => unreachable!(),
			}

			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        deserializer.deserialize_any(Visitor)")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl serde::Serialize for {} {{", type_name)?;
			writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
			writeln!(out, "        match self {{")?;
			writeln!(out, "            {}::Schema(value) => value.serialize(serializer),", type_name)?;

			match ty {
				swagger20::Type::JSONSchemaPropsOrArray => writeln!(out, "            {}::Schemas(value) => value.serialize(serializer),", type_name)?,
				swagger20::Type::JSONSchemaPropsOrBool => writeln!(out, "            {}::Bool(value) => value.serialize(serializer),", type_name)?,
				swagger20::Type::JSONSchemaPropsOrStringArray => writeln!(out, "            {}::Strings(value) => value.serialize(serializer),", type_name)?,
				_ => unreachable!(),
			}

			writeln!(out, "        }}")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => {
			writeln!(out, "#[derive(Clone, Debug, PartialEq)]")?;
			writeln!(out, "{}enum {} {{", vis, type_name)?;
			writeln!(out, "    Json(Vec<serde_json::Value>),")?;
			writeln!(out, "    Merge(serde_json::Value),")?;
			writeln!(out, "    StrategicMerge(serde_json::Value),")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl serde::Serialize for {} {{", type_name)?;
			writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
			writeln!(out, "        match self {{")?;
			writeln!(out, r#"            {}::Json(patch) => serializer.serialize_newtype_struct("{}", patch),"#, type_name, type_name)?;
			writeln!(out, r#"            {}::Merge(patch) |"#, type_name)?;
			writeln!(out, r#"            {}::StrategicMerge(patch) => serializer.serialize_newtype_struct("{}", patch),"#, type_name, type_name)?;
			writeln!(out, "        }}")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(raw_extension_ref_path)) => {
			let has_bookmark_event_type = {
				let watch_optional_schema =
					definitions.get(&swagger20::DefinitionPath("io.k8s.WatchOptional".to_owned()))
					.ok_or("could not find io.k8s.WatchOptional")?;
				let watch_optional_properties =
					if let swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(properties)) = &watch_optional_schema.kind {
						properties
					}
					else {
						return Err("io.k8s.WatchOptional has unexpected schema kind".into());
					};
				watch_optional_properties.contains_key(&swagger20::PropertyName("allowWatchBookmarks".to_owned()))
			};

			writeln!(out, "#[derive(Clone, Debug, PartialEq)]")?;
			writeln!(out, "{}enum {}<T> {{", vis, type_name)?;
			writeln!(out, "    Added(T),")?;
			writeln!(out, "    Deleted(T),")?;
			writeln!(out, "    Modified(T),")?;
			if has_bookmark_event_type {
				writeln!(out, "    Bookmark(T),")?;
			}
			writeln!(out, "    ErrorStatus({}),", get_rust_type(
				&swagger20::SchemaKind::Ref(swagger20::RefPath {
					path: "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned(),
					relative_to: swagger20::RefPathRelativeTo::Crate,
					can_be_default: None,
				}),
				&replace_namespaces,
				crate_root,
				mod_root,
			)?)?;
			writeln!(out, "    ErrorOther({}),", get_rust_type(&swagger20::SchemaKind::Ref(raw_extension_ref_path.clone()), &replace_namespaces, crate_root, mod_root)?)?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl<'de, T> serde::Deserialize<'de> for {}<T> where T: serde::Deserialize<'de> {{", type_name)?;
			writeln!(out, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "        #[allow(non_camel_case_types)]")?;
			writeln!(out, "        enum Field {{")?;
			writeln!(out, "            Key_type,")?;
			writeln!(out, "            Key_object,")?;
			writeln!(out, "            Other,")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::Deserialize<'de> for Field {{")?;
			writeln!(out, "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "                struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "                impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "                    type Value = Field;")?;
			writeln!(out)?;
			writeln!(out, "                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                        write!(f, "field identifier")"#)?;
			writeln!(out, "                    }}")?;
			writeln!(out)?;
			writeln!(out, "                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                        Ok(match v {{")?;
			writeln!(out, r#"                            "type" => Field::Key_type,"#)?;
			writeln!(out, r#"                            "object" => Field::Key_object,"#)?;
			writeln!(out, "                            _ => Field::Other,")?;
			writeln!(out, "                        }})")?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, "                deserializer.deserialize_identifier(Visitor)")?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        enum WatchEventType {{")?;
			writeln!(out, "            Added,")?;
			writeln!(out, "            Deleted,")?;
			writeln!(out, "            Modified,")?;
			if has_bookmark_event_type {
				writeln!(out, "            Bookmark,")?;
			}
			writeln!(out, "            Error,")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::Deserialize<'de> for WatchEventType {{")?;
			writeln!(out, "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "                struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "                impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "                    type Value = WatchEventType;")?;
			writeln!(out)?;
			writeln!(out, "                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                        write!(f, "field identifier")"#)?;
			writeln!(out, "                    }}")?;
			writeln!(out)?;
			writeln!(out, "                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
			writeln!(out, "                        Ok(match v {{")?;
			writeln!(out, r#"                            "ADDED" => WatchEventType::Added,"#)?;
			writeln!(out, r#"                            "DELETED" => WatchEventType::Deleted,"#)?;
			writeln!(out, r#"                            "MODIFIED" => WatchEventType::Modified,"#)?;
			if has_bookmark_event_type {
				writeln!(out, r#"                            "BOOKMARK" => WatchEventType::Bookmark,"#)?;
			}
			writeln!(out, r#"                            "ERROR" => WatchEventType::Error,"#)?;
			writeln!(out, "                            _ => return Err(serde::de::Error::unknown_variant(")?;
			writeln!(out, "                                v,")?;
			if has_bookmark_event_type {
				writeln!(out, r#"                                &["ADDED", "DELETED", "MODIFIED", "BOOKMARK", "ERROR"],"#)?;
			}
			else {
				writeln!(out, r#"                                &["ADDED", "DELETED", "MODIFIED", "ERROR"],"#)?;
			}
			writeln!(out, "                            )),")?;
			writeln!(out, "                        }})")?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, "                deserializer.deserialize_identifier(Visitor)")?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        struct Visitor<T>(std::marker::PhantomData<T>);")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> where T: serde::Deserialize<'de> {{")?;
			writeln!(out, "            type Value = {}<T>;", type_name)?;
			writeln!(out)?;
			writeln!(out, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                write!(f, "struct {}")"#, type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{")?;
			writeln!(out, "                let mut value_type: Option<WatchEventType> = None;")?;
			writeln!(out, "                let mut value_object: Option<serde_value::Value> = None;")?;
			writeln!(out)?;
			writeln!(out, "                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{")?;
			writeln!(out, "                    match key {{")?;
			writeln!(out, "                        Field::Key_type => value_type = serde::de::MapAccess::next_value(&mut map)?,")?;
			writeln!(out, "                        Field::Key_object => value_object = serde::de::MapAccess::next_value(&mut map)?,")?;
			writeln!(out, "                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},")?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                }}")?;
			writeln!(out)?;
			writeln!(out, r#"                let value_type = value_type.ok_or_else(|| serde::de::Error::missing_field("type"))?;"#)?;
			writeln!(out, r#"                let value_object = value_object.ok_or_else(|| serde::de::Error::missing_field("object"))?;"#)?;
			writeln!(out)?;
			writeln!(out, "                Ok(match value_type {{")?;
			writeln!(out, "                    WatchEventType::Added => {{")?;
			writeln!(out, "                        let value_object = serde_value::ValueDeserializer::new(value_object);")?;
			writeln!(out, "                        {}::Added(serde::Deserialize::deserialize(value_object)?)", type_name)?;
			writeln!(out, "                    }},")?;
			writeln!(out, "                    WatchEventType::Deleted => {{")?;
			writeln!(out, "                        let value_object = serde_value::ValueDeserializer::new(value_object);")?;
			writeln!(out, "                        {}::Deleted(serde::Deserialize::deserialize(value_object)?)", type_name)?;
			writeln!(out, "                    }},")?;
			writeln!(out, "                    WatchEventType::Modified => {{")?;
			writeln!(out, "                        let value_object = serde_value::ValueDeserializer::new(value_object);")?;
			writeln!(out, "                        {}::Modified(serde::Deserialize::deserialize(value_object)?)", type_name)?;
			writeln!(out, "                    }},")?;
			if has_bookmark_event_type {
				writeln!(out, "                    WatchEventType::Bookmark => {{")?;
				writeln!(out, "                        let value_object = serde_value::ValueDeserializer::new(value_object);")?;
				writeln!(out, "                        {}::Bookmark(serde::Deserialize::deserialize(value_object)?)", type_name)?;
				writeln!(out, "                    }},")?;
			}
			writeln!(out)?;
			writeln!(out, "                    WatchEventType::Error => {{")?;
			writeln!(out, "                        let is_status =")?;
			writeln!(out, "                            if let serde_value::Value::Map(map) = &value_object {{")?;
			writeln!(out, r#"                                match map.get(&serde_value::Value::String("kind".to_owned())) {{"#)?;
			writeln!(out, r#"                                    Some(serde_value::Value::String(s)) if s == "Status" => true,"#)?;
			writeln!(out, "                                    _ => false,")?;
			writeln!(out, "                                }}")?;
			writeln!(out, "                            }}")?;
			writeln!(out, "                            else {{")?;
			writeln!(out, "                                false")?;
			writeln!(out, "                            }};")?;
			writeln!(out, "                        let value_object = serde_value::ValueDeserializer::new(value_object);")?;
			writeln!(out, "                        if is_status {{")?;
			writeln!(out, "                            {}::ErrorStatus(serde::Deserialize::deserialize(value_object)?)", type_name)?;
			writeln!(out, "                        }}")?;
			writeln!(out, "                        else {{")?;
			writeln!(out, "                            {}::ErrorOther(serde::Deserialize::deserialize(value_object)?)", type_name)?;
			writeln!(out, "                        }}")?;
			writeln!(out, "                    }},")?;
			writeln!(out, "                }})")?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        deserializer.deserialize_struct(")?;
			writeln!(out, r#"            "{}","#, type_name)?;
			writeln!(out, "            &[")?;
			writeln!(out, r#"                "type","#)?;
			writeln!(out, r#"                "object","#)?;
			writeln!(out, "            ],")?;
			writeln!(out, "            Visitor(Default::default()),")?;
			writeln!(out, "        )")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl<T> serde::Serialize for {}<T> where T: serde::Serialize {{", type_name)?;
			writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
			writeln!(out, "        let mut state = serializer.serialize_struct(")?;
			writeln!(out, r#"            "{}","#, type_name)?;
			writeln!(out, "            2,")?;
			writeln!(out, "        )?;")?;
			writeln!(out, "        match self {{")?;
			writeln!(out, "            {}::Added(object) => {{", type_name)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ADDED")?;"#)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
			writeln!(out, "            }},")?;
			writeln!(out, "            {}::Deleted(object) => {{", type_name)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "DELETED")?;"#)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
			writeln!(out, "            }},")?;
			writeln!(out, "            {}::Modified(object) => {{", type_name)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "MODIFIED")?;"#)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
			writeln!(out, "            }},")?;
			if has_bookmark_event_type {
				writeln!(out, "            {}::Bookmark(object) => {{", type_name)?;
				writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "BOOKMARK")?;"#)?;
				writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
				writeln!(out, "            }},")?;
			}
			writeln!(out, "            {}::ErrorStatus(object) => {{", type_name)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;"#)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
			writeln!(out, "            }},")?;
			writeln!(out, "            {}::ErrorOther(object) => {{", type_name)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;"#)?;
			writeln!(out, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
			writeln!(out, "            }},")?;
			writeln!(out, "        }}")?;
			writeln!(out, "        serde::ser::SerializeStruct::end(state)")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(ty @ swagger20::Type::DeleteOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::ListOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::PatchOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::WatchOptional(_)) => {
			struct Property<'a> {
				name: &'a str,
				schema: &'a swagger20::Schema,
				field_name: std::borrow::Cow<'static, str>,
				field_type_name: String,
			}

			let properties = match ty {
				swagger20::Type::DeleteOptional(properties) |
				swagger20::Type::ListOptional(properties) |
				swagger20::Type::PatchOptional(properties) |
				swagger20::Type::WatchOptional(properties) => properties,
				_ => unreachable!(),
			};

			let properties = {
				let mut result = Vec::with_capacity(properties.len());

				for (name, schema) in properties {
					let field_name = get_rust_ident(name);

					let type_name = get_rust_borrow_type(&schema.kind, &replace_namespaces, crate_root, mod_root)?;

					let field_type_name =
						if type_name.starts_with('&') {
							format!("Option<&'a {}>", &type_name[1..])
						}
						else {
							format!("Option<{}>", type_name)
						};

					result.push(Property {
						name,
						schema,
						field_name,
						field_type_name,
					});
				}

				result
			};

			writeln!(out, "#[derive(Clone, Copy, Debug, Default, PartialEq)]")?;
			writeln!(out, "{}struct {}<'a> {{", vis, type_name)?;

			for (i, Property { schema, field_name, field_type_name, .. }) in properties.iter().enumerate() {
				if i > 0 {
					writeln!(out)?;
				}

				if let Some(description) = &schema.description {
					for line in get_comment_text(description, "") {
						writeln!(out, "    ///{}", line)?;
					}
				}

				writeln!(out, "    {}{}: {},", vis, field_name, field_type_name)?;
			}
			writeln!(out, "}}")?;

			writeln!(out)?;

			match ty {
				swagger20::Type::DeleteOptional(_) => {
					writeln!(out, "impl serde::Serialize for {}<'_> {{", type_name)?;
					writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
					writeln!(out, "        let mut state = serializer.serialize_struct(")?;
					writeln!(out, r#"            "{}","#, type_name)?;

					for (i, Property { field_name, .. }) in properties.iter().enumerate() {
						if i > 0 {
							writeln!(out, " +")?;
						}
						write!(out, "            self.{}.as_ref().map_or(0, |_| 1)", field_name)?;
					}
					writeln!(out, ",")?;

					writeln!(out, "        )?;")?;

					for Property { name, field_name, .. } in &properties {
						writeln!(out, "        if let Some(value) = &self.{} {{", field_name)?;
						writeln!(out, r#"            serde::ser::SerializeStruct::serialize_field(&mut state, "{}", value)?;"#, name)?;
						writeln!(out, "        }}")?;
					}
					writeln!(out, "        serde::ser::SerializeStruct::end(state)")?;
					writeln!(out, "    }}")?;
					writeln!(out, "}}")?;
				},

				swagger20::Type::ListOptional(_) |
				swagger20::Type::PatchOptional(_) |
				swagger20::Type::WatchOptional(_) => {
					writeln!(out, "impl {}<'_> {{", type_name)?;
					writeln!(out, "    #[doc(hidden)]")?;
					writeln!(out, "    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]")?;
					writeln!(out, "    ///")?;
					writeln!(out, "    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.")?;
					writeln!(out, "    {}fn __serialize<T>(", vis)?;
					writeln!(out, "        self,")?;
					writeln!(out, "        __query_pairs: &mut crate::url::form_urlencoded::Serializer<T>,")?;
					writeln!(out, "    ) where T: crate::url::form_urlencoded::Target {{")?;
					for Property { name, schema, field_name, .. } in properties {
						writeln!(out, "        if let Some({}) = self.{} {{", field_name, field_name)?;
						match schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
							swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
							swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
								writeln!(out, r#"            __query_pairs.append_pair("{}", &{}.to_string());"#, name, field_name)?,

							swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
								writeln!(out, r#"            __query_pairs.append_pair("{}", {});"#, name, field_name)?,

							_ => unreachable!(),
						}
						writeln!(out, "        }}")?;
					}
					if let swagger20::Type::WatchOptional(_) = ty {
						writeln!(out, r#"        __query_pairs.append_pair("watch", "true");"#)?;
					}
					writeln!(out, "    }}")?;
					writeln!(out, "}}")?;
				},

				_ => unreachable!(),
			}

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(_) => {
			write!(out, "#[derive(Clone, Debug, ")?;
			if can_be_default {
				write!(out, "Default, ")?;
			}
			writeln!(out, "PartialEq)]")?;

			writeln!(out, "{}struct {}({}{});", vis, type_name, vis, get_rust_type(&definition.kind, &replace_namespaces, crate_root, mod_root)?)?;
			writeln!(out)?;
			writeln!(out, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
			writeln!(out, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "        struct Visitor;")?;
			writeln!(out)?;
			writeln!(out, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
			writeln!(out, "            type Value = {};", type_name)?;
			writeln!(out)?;
			writeln!(out, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
			writeln!(out, r#"                write!(f, "{}")"#, type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {{")?;
			writeln!(out, "                Ok({}(serde::Deserialize::deserialize(deserializer)?))", type_name)?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, r#"        deserializer.deserialize_newtype_struct("{}", Visitor)"#, type_name)?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
			writeln!(out)?;
			writeln!(out, "impl serde::Serialize for {} {{", type_name)?;
			writeln!(out, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
			writeln!(out, r#"        serializer.serialize_newtype_struct("{}", &self.0)"#, type_name)?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;

			run_result.num_generated_type_aliases += 1;
		},
	}

	Ok(run_result)
}

fn can_be_default(
	kind: &swagger20::SchemaKind,
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
) -> Result<bool, Error> {
	#[allow(clippy::match_same_arms)]
	match kind {
		swagger20::SchemaKind::Properties(properties) => {
			for (schema, required) in properties.values() {
				if !required {
					// Option<T>::default is None regardless of T
					continue;
				}

				if !can_be_default(&schema.kind, definitions)? {
					return Ok(false);
				}
			}

			Ok(true)
		},

		swagger20::SchemaKind::Ref(swagger20::RefPath { can_be_default: Some(can_be_default), .. }) => Ok(*can_be_default),

		swagger20::SchemaKind::Ref(swagger20::RefPath { relative_to: swagger20::RefPathRelativeTo::Scope, .. }) => Ok(false),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) => {
			let target =
				definitions.get(&swagger20::DefinitionPath(path.to_owned()))
				.ok_or_else(|| format!("couldn't find target of ref path {}", path))?;
			can_be_default(&target.kind, definitions)
		},

		// chrono::DateTime<chrono::Utc> is not Default
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok(false),

		swagger20::SchemaKind::Ty(_) => Ok(true),
	}
}

fn get_comment_text<'a>(s: &'a str, indent: &'a str) -> impl Iterator<Item = std::borrow::Cow<'static, str>> + 'a {
	s.lines().map(move |line|
		if line.is_empty() {
			"".into()
		}
		else {
			let line =
				line
				.replace(r"\", r"\\")
				.replace("[", r"\[")
				.replace("]", r"\]")
				.replace("<", r"\<")
				.replace(">", r"\>");
			format!("{} {}", indent, line).into()
		})
}

fn get_fully_qualified_type_name(
	ref_path: &swagger20::RefPath,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	mod_root: Option<&str>,
) -> Result<String, Error> {
	use std::fmt::Write;

	match ref_path.relative_to {
		swagger20::RefPathRelativeTo::Crate => {
			let mut result =
				if let Some(mod_root) = mod_root {
					format!("{}::{}", crate_root, mod_root)
				}
				else {
					crate_root.to_owned()
				};

			let parts = replace_namespace(ref_path.path.split('.'), replace_namespaces);

			for part in parts.iter().rev().skip(1).rev() {
				write!(result, "::{}", get_rust_ident(part))?;
			}

			write!(result, "::{}", parts.last().ok_or_else(|| format!("path for {} has no parts", ref_path.path))?)?;

			Ok(result)
		},

		swagger20::RefPathRelativeTo::Scope => {
			let last_part = ref_path.path.split('.').last().ok_or_else(|| format!("path for {} has no parts", ref_path.path))?;
			Ok(last_part.to_owned())
		},
	}
}

#[doc(hidden)]
pub fn get_rust_ident(name: &str) -> std::borrow::Cow<'static, str> {
	// Fix cases of invalid rust idents
	match name {
		"$ref" => return "ref_path".into(),
		"$schema" => return "schema".into(),
		"continue" => return "continue_".into(),
		"enum" => return "enum_".into(),
		"type" => return "type_".into(),
		_ => (),
	}

	// Some cases of "ABc" should be converted to "abc" instead of "a_bc".
	// Eg "JSONSchemas" => "json_schemas", but "externalIPs" => "external_ips" instead of "external_i_ps".
	// Mostly happens with plurals of abbreviations.
	match name {
		"externalIPs" => return "external_ips".into(),
		"nonResourceURLs" => return "non_resource_urls".into(),
		"podCIDRs" => return "pod_cidrs".into(),
		"podIPs" => return "pod_ips".into(),
		"serverAddressByClientCIDRs" => return "server_address_by_client_cidrs".into(),
		"targetWWNs" => return "target_wwns".into(),
		_ => (),
	}

	let mut result = String::new();

	let chars =
		name.chars()
		.zip(std::iter::once(None).chain(name.chars().map(|c| Some(c.is_uppercase()))))
		.zip(name.chars().skip(1).map(|c| Some(c.is_uppercase())).chain(std::iter::once(None)));

	for ((c, previous), next) in chars {
		if c.is_uppercase() {
			match (previous, next) {
				(Some(false), _) |
				(Some(true), Some(false)) => result.push('_'),
				_ => (),
			}

			result.extend(c.to_lowercase());
		}
		else {
			result.push(match c {
				'-' => '_',
				c => c,
			});
		}
	}

	result.into()
}

fn get_rust_borrow_type(
	schema_kind: &swagger20::SchemaKind,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	mod_root: Option<&str>,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	match schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.DeleteOptional" =>
			if let Some(mod_root) = mod_root {
				Ok(format!("{}::{}::DeleteOptional<'_>", crate_root, mod_root).into())
			}
			else {
				Ok(format!("{}::DeleteOptional<'_>", crate_root).into())
			},

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.ListOptional" =>
			if let Some(mod_root) = mod_root {
				Ok(format!("{}::{}::ListOptional<'_>", crate_root, mod_root).into())
			}
			else {
				Ok(format!("{}::ListOptional<'_>", crate_root).into())
			},

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.PatchOptional" =>
			if let Some(mod_root) = mod_root {
				Ok(format!("{}::{}::PatchOptional<'_>", crate_root, mod_root).into())
			}
			else {
				Ok(format!("{}::PatchOptional<'_>", crate_root).into())
			},

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.WatchOptional" =>
			if let Some(mod_root) = mod_root {
				Ok(format!("{}::{}::WatchOptional<'_>", crate_root, mod_root).into())
			}
			else {
				Ok(format!("{}::WatchOptional<'_>", crate_root).into())
			},

		swagger20::SchemaKind::Ref(ref_path) =>
			Ok(format!("&{}", get_fully_qualified_type_name(ref_path, replace_namespaces, crate_root, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok("&serde_json::Value".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
			Ok(format!("&[{}]", get_rust_type(&items.kind, replace_namespaces, crate_root, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
			Ok(format!("&std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, crate_root, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => Ok(format!("&{}::ByteString", crate_root).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok("&chrono::DateTime<chrono::Utc>".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("&str".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray) => Err("JSON schema types not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteOptional(_)) => Err("DeleteOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListOptional(_)) => Err("ListOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::PatchOptional(_)) => Err("PatchOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(_)) => Err("WatchOptional type not supported".into()),
	}
}

fn get_rust_type(
	schema_kind: &swagger20::SchemaKind,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	mod_root: Option<&str>,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	match schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(ref_path) =>
			Ok(get_fully_qualified_type_name(ref_path, replace_namespaces, crate_root, mod_root)?.into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok("serde_json::Value".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
			Ok(format!("Vec<{}>", get_rust_type(&items.kind, replace_namespaces, crate_root, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
			Ok(format!("std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, crate_root, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => Ok(format!("{}::ByteString", crate_root).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok("chrono::DateTime<chrono::Utc>".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("String".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray) => Err("JSON schema types not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteOptional(_)) => Err("DeleteOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListOptional(_)) => Err("ListOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::PatchOptional(_)) => Err("PatchOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(_)) => Err("WatchOptional type not supported".into()),
	}
}

fn replace_namespace<'a, I>(
	parts: I,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
) -> Vec<std::borrow::Cow<'a, str>> where I: IntoIterator<Item = &'a str> {
	let parts: Vec<_> = parts.into_iter().map(Into::into).collect();

	log::trace!("parts = {:?}, replace_namespaces = {:?}", parts, replace_namespaces);

	for (from, to) in replace_namespaces {
		if parts.starts_with(from) {
			let mut result = to.to_vec();
			result.extend(parts.into_iter().skip(from.len()));
			return result;
		}
	}

	parts
}

#[doc(hidden)]
pub fn write_operation(
	out: &mut impl std::io::Write,
	operation: &swagger20::Operation,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	mod_root: Option<&str>,
	vis: &str,
	type_name_and_ref_path: &mut Option<(&str, &swagger20::RefPath)>,
) -> Result<(Option<String>, Option<String>), Error> {
	writeln!(out)?;

	writeln!(out, "// Generated from operation {}", operation.id)?;

	let (operation_fn_name, operation_result_name, operation_optional_parameters_name) =
		get_operation_names(operation, type_name_and_ref_path.is_some())?;

	let operation_responses: Result<Vec<_>, _> =
		operation.responses.iter()
		.map(|(&status_code, schema)| {
			let http_status_code = match status_code {
				http::StatusCode::ACCEPTED => "ACCEPTED",
				http::StatusCode::CREATED => "CREATED",
				http::StatusCode::OK => "OK",
				http::StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
				http::StatusCode::UNPROCESSABLE_ENTITY => "UNPROCESSABLE_ENTITY",
				_ => return Err(format!("unrecognized status code {}", status_code)),
			};

			let variant_name = match status_code {
				http::StatusCode::ACCEPTED => "Accepted",
				http::StatusCode::CREATED => "Created",
				http::StatusCode::OK => "Ok",
				http::StatusCode::UNAUTHORIZED => "Unauthorized",
				http::StatusCode::UNPROCESSABLE_ENTITY => "UnprocessableEntity",
				_ => return Err(format!("unrecognized status code {}", status_code)),
			};

			let is_delete_ok_status = match &schema.kind {
				swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if
					path == "io.k8s.apimachinery.pkg.apis.meta.v1.Status" &&
					operation.method == swagger20::Method::Delete &&
					status_code == http::StatusCode::OK => true,

				_ => false,
			};

			Ok((http_status_code, variant_name, schema, is_delete_ok_status))
		})
		.collect();
	let operation_responses = operation_responses?;

	let indent = if type_name_and_ref_path.is_some() { "    " } else { "" };

	writeln!(out)?;

	if let Some((type_name, _)) = type_name_and_ref_path {
		writeln!(out, "impl {} {{", type_name)?;
	}

	let parameters: Vec<_> = operation.parameters.iter().map(std::ops::Deref::deref).collect();
	let mut previous_parameters: std::collections::HashSet<_> = Default::default();
	let parameters: Result<Vec<_>, Error> =
		parameters.into_iter()
		.map(|parameter| {
			let mut parameter_name = get_rust_ident(&parameter.name);
			while previous_parameters.contains(&parameter_name) {
				parameter_name = format!("{}_", parameter_name).into();
			}
			previous_parameters.insert(parameter_name.clone());

			let parameter_type = match get_rust_borrow_type(&parameter.schema.kind, replace_namespaces, crate_root, mod_root) {
				Ok(parameter_type) => parameter_type,
				Err(err) => return Err(err),
			};

			Ok((parameter_name, parameter_type, parameter))
		})
		.collect();
	let mut parameters = parameters?;

	let delete_optional_parameter =
		if let Some(index) =
			parameters.iter()
			.position(|(_, _, parameter)|
				if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &parameter.schema.kind {
					path == "io.k8s.DeleteOptional"
				}
				else {
					false
				})
		{
			Some(parameters.swap_remove(index))
		}
		else {
			None
		};

	let list_or_patch_or_watch_optional_parameter =
		if let Some(index) =
			parameters.iter()
			.position(|(_, _, parameter)|
				if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &parameter.schema.kind {
					path == "io.k8s.ListOptional" || path == "io.k8s.PatchOptional" || path == "io.k8s.WatchOptional"
				}
				else {
					false
				})
		{
			Some(parameters.swap_remove(index))
		}
		else {
			None
		};

	parameters.sort_by(|(_, _, parameter1), (_, _, parameter2)| {
		(match (parameter1.location, parameter2.location) {
			(location1, location2) if location1 == location2 => std::cmp::Ordering::Equal,
			(swagger20::ParameterLocation::Path, _) |
			(swagger20::ParameterLocation::Body, swagger20::ParameterLocation::Query) => std::cmp::Ordering::Less,
			_ => std::cmp::Ordering::Greater,
		})
		.then_with(|| parameter1.name.cmp(&parameter2.name))
	});
	let parameters = parameters;
	let (required_parameters, optional_parameters): (Vec<_>, Vec<_>) = parameters.iter().partition(|(_, _, parameter)| parameter.required);
	let any_optional_fields_have_lifetimes = optional_parameters.iter().any(|(_, parameter_type, _)| parameter_type.starts_with('&'));

	let mut need_empty_line = false;

	if let Some(description) = operation.description.as_ref() {
		for line in get_comment_text(description, "") {
			writeln!(out, "{}///{}", indent, line)?;
			need_empty_line = true;
		}
	}
	if let Some(operation_result_name) = &operation_result_name {
		if need_empty_line {
			writeln!(out, "{}///", indent)?;
		}

		writeln!(out,
			"{}/// Use the returned [`{}::ResponseBody`]`<`[`{}`]`>` constructor, or [`{}`] directly, to parse the HTTP response.",
			indent, crate_root, operation_result_name, operation_result_name)?;
		need_empty_line = true;
	}

	if !parameters.is_empty() || delete_optional_parameter.is_some() || list_or_patch_or_watch_optional_parameter.is_some() {
		if need_empty_line {
			writeln!(out, "{}///", indent)?;
		}

		writeln!(out, "{}/// # Arguments", indent)?;
		for (parameter_name, _, parameter) in &required_parameters {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(out, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(out, "{}///{}", indent, line)?;
				}
			}
		}
		if let Some((parameter_name, _, parameter)) = &delete_optional_parameter {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(out, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(out, "{}///{}", indent, line)?;
				}
			}
		}
		if let Some((parameter_name, _, parameter)) = &list_or_patch_or_watch_optional_parameter {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(out, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(out, "{}///{}", indent, line)?;
				}
			}
		}
		if !optional_parameters.is_empty() {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `optional`", indent)?;
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}///     Optional parameters. Use `Default::default()` to not pass any.", indent)?;
		}
	}

	if mod_root.is_some() {
		writeln!(out, r#"{}#[cfg(feature = "api")]"#, indent)?;
	}

	writeln!(out, "{}{}fn {}(", indent, vis, operation_fn_name)?;
	for (parameter_name, parameter_type, _) in &required_parameters {
		writeln!(out, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if let Some((parameter_name, parameter_type, _)) = &delete_optional_parameter {
		writeln!(out, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if let Some((parameter_name, parameter_type, _)) = &list_or_patch_or_watch_optional_parameter {
		writeln!(out, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if !optional_parameters.is_empty() {
		write!(out, "{}    optional: {}", indent, operation_optional_parameters_name)?;
		if any_optional_fields_have_lifetimes {
			write!(out, "<'_>")?;
		}
		writeln!(out, ",")?;
	}
	if let Some(operation_result_name) = &operation_result_name {
		writeln!(out,
			"{}) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> {}::ResponseBody<{}>), {}::RequestError> {{",
			indent, crate_root, operation_result_name, crate_root)?;
	}
	else {
		writeln!(out, "{}) -> Result<http::Request<Vec<u8>>, {}::RequestError> {{", indent, crate_root)?;
	}

	let have_path_parameters = parameters.iter().any(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Path);
	let have_query_parameters =
		parameters.iter().any(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Query) ||
		list_or_patch_or_watch_optional_parameter.is_some();

	if !optional_parameters.is_empty() {
		writeln!(out, "{}    let {} {{", indent, operation_optional_parameters_name)?;
		for (parameter_name, _, _) in &optional_parameters {
			writeln!(out, "{}        {},", indent, parameter_name)?;
		}

		writeln!(out, "{}    }} = optional;", indent)?;
	}

	if have_path_parameters {
		write!(out, r#"{}    let __url = format!("{}"#, indent, operation.path)?;
		if have_query_parameters {
			write!(out, "?")?;
		}
		write!(out, r#"""#)?;
		if !parameters.is_empty() {
			writeln!(out, ",")?;
			for (parameter_name, parameter_type, parameter) in &parameters {
				if parameter.location == swagger20::ParameterLocation::Path {
					match parameter.schema.kind {
						swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => (),
						_ => return Err(format!("parameter {} is in the path but is a {:?}", parameter_name, parameter_type).into()),
					}

					writeln!(
						out,
						"{}        {} = {}::url::percent_encoding::percent_encode({}.as_bytes(), {}::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),",
						indent,
						parameter_name,
						crate_root,
						parameter_name,
						crate_root)?;
				}
			}
			write!(out, "{}    ", indent)?;
		}
		writeln!(out, ");")?;
	}
	else {
		write!(out, r#"{}    let __url = "{}"#, indent, operation.path)?;
		if have_query_parameters {
			write!(out, "?")?;
		}
		writeln!(out, r#"".to_owned();"#)?;
	}

	if have_query_parameters {
		writeln!(out, "{}    let mut __query_pairs = {}::url::form_urlencoded::Serializer::new(__url);", indent, crate_root)?;
		if let Some((parameter_name, _, _)) = &list_or_patch_or_watch_optional_parameter {
			writeln!(out, "{}    {}.__serialize(&mut __query_pairs);", indent, parameter_name)?;
		}
		else {
			for (parameter_name, parameter_type, parameter) in &parameters {
				if parameter.location == swagger20::ParameterLocation::Query {
					if parameter.required {
						match parameter.schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
							swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
							swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
								writeln!(out, r#"{}    __query_pairs.append_pair("{}", &{}.to_string());"#, indent, parameter.name, parameter_name)?,

							swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
								writeln!(out, r#"{}    __query_pairs.append_pair("{}", &{});"#, indent, parameter.name, parameter_name)?,

							_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
						}
					}
					else {
						writeln!(out, "{}    if let Some({}) = {} {{", indent, parameter_name, parameter_name)?;
						match parameter.schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
							swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
							swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
								writeln!(out, r#"{}        __query_pairs.append_pair("{}", &{}.to_string());"#, indent, parameter.name, parameter_name)?,

							swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
								writeln!(out, r#"{}        __query_pairs.append_pair("{}", {});"#, indent, parameter.name, parameter_name)?,

							_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
						}
						writeln!(out, "{}    }}", indent)?;
					}
				}
			}
		}
		writeln!(out, "{}    let __url = __query_pairs.finish();", indent)?;
	}
	writeln!(out)?;

	let method = match operation.method {
		swagger20::Method::Delete => "delete",
		swagger20::Method::Get => "get",
		swagger20::Method::Patch => "patch",
		swagger20::Method::Post => "post",
		swagger20::Method::Put => "put",
	};

	writeln!(out, "{}    let mut __request = http::Request::{}(__url);", indent, method)?;

	let body_parameter =
		delete_optional_parameter.as_ref()
		.or_else(|| parameters.iter().find(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Body));

	write!(out, "{}    let __body = ", indent)?;
	if let Some((parameter_name, parameter_type, parameter)) = body_parameter {
		if parameter.required {
			if parameter_type.starts_with('&') {
				writeln!(out, "serde_json::to_vec({}).map_err({}::RequestError::Json)?;", parameter_name, crate_root)?;
			}
			else {
				writeln!(out, "serde_json::to_vec(&{}).map_err({}::RequestError::Json)?;", parameter_name, crate_root)?;
			}
		}
		else {
			writeln!(out)?;
			writeln!(out, "{}.unwrap_or(Ok(vec![]), |value| serde_json::to_vec(value).map_err({}::RequestError::Json))?;", parameter_name, crate_root)?;
		}

		let is_patch =
			if let swagger20::SchemaKind::Ref(ref_path) = &parameter.schema.kind {
				ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.Patch"
			}
			else {
				false
			};
		if is_patch {
			let patch_type = get_rust_type(&parameter.schema.kind, replace_namespaces, crate_root, mod_root)?;
			writeln!(out, "{}    __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match {} {{", indent, parameter_name)?;
			writeln!(out, r#"{}        {}::Json(_) => "application/json-patch+json","#, indent, patch_type)?;
			writeln!(out, r#"{}        {}::Merge(_) => "application/merge-patch+json","#, indent, patch_type)?;
			writeln!(out, r#"{}        {}::StrategicMerge(_) => "application/strategic-merge-patch+json","#, indent, patch_type)?;
			writeln!(out, "{}    }}));", indent)?;
		}
		else {
			writeln!(out, r#"{}    __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));"#, indent)?;
		}
	}
	else {
		writeln!(out, "vec![];")?;
	}

	if operation_result_name.is_some() {
		writeln!(out, "{}    match __request.body(__body) {{", indent)?;
		writeln!(out, "{}        Ok(request) => Ok((request, {}::ResponseBody::new)),", indent, crate_root)?;
		writeln!(out, "{}        Err(err) => Err({}::RequestError::Http(err)),", indent, crate_root)?;
		writeln!(out, "{}    }}", indent)?;
	}
	else {
		writeln!(out, "{}    __request.body(__body).map_err({}::RequestError::Http)", indent, crate_root)?;
	}
	writeln!(out, "{}}}", indent)?;

	if type_name_and_ref_path.is_some() {
		writeln!(out, "}}")?;
	}

	if !optional_parameters.is_empty() {
		writeln!(out)?;

		if let Some((type_name, _)) = type_name_and_ref_path {
			writeln!(out, "/// Optional parameters of [`{}::{}`]", type_name, operation_fn_name)?;
		}
		else {
			writeln!(out, "/// Optional parameters of [`{}`]", operation_fn_name)?;
		}

		if mod_root.is_some() {
			writeln!(out, r#"#[cfg(feature = "api")]"#)?;
		}
		writeln!(out, "#[derive(Clone, Copy, Debug, Default)]")?;
		write!(out, "{}struct {}", vis, operation_optional_parameters_name)?;
		if any_optional_fields_have_lifetimes {
			write!(out, "<'a>")?;
		}
		writeln!(out, " {{")?;

		for (parameter_name, parameter_type, parameter) in &optional_parameters {
			if let Some(description) = parameter.schema.description.as_ref() {
				for line in get_comment_text(description, "") {
					writeln!(out, "    ///{}", line)?;
				}
			}
			if parameter_type.starts_with('&') {
				writeln!(out, "    {}{}: Option<&'a {}>,", vis, parameter_name, &parameter_type[1..])?;
			}
			else {
				writeln!(out, "    {}{}: Option<{}>,", vis, parameter_name, parameter_type)?;
			}
		}

		writeln!(out, "}}")?;
	}

	if let Some(operation_result_name) = &operation_result_name {
		writeln!(out)?;

		if let Some((type_name, _)) = type_name_and_ref_path {
			writeln!(out,
				"/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}::{}`]",
				operation_result_name, type_name, operation_fn_name)?;
		}
		else {
			writeln!(out,
				"/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}`]",
				operation_result_name, operation_fn_name)?;
		}

		if mod_root.is_some() {
			writeln!(out, r#"#[cfg(feature = "api")]"#)?;
		}
		writeln!(out, "#[derive(Debug)]")?;
		writeln!(out, "{}enum {} {{", vis, operation_result_name)?;

		for &(_, variant_name, schema, is_delete_ok_status) in &operation_responses {
			if is_delete_ok_status {
				// Delete and delete-collection operations that return metav1.Status for HTTP 200 can also return the object itself instead.
				//
				// In case of delete-collection operations, this is the list object corresponding to the associated type.
				//
				// Ref https://github.com/kubernetes/kubernetes/issues/59501

				writeln!(out, "    {}Status({}),", variant_name, get_rust_type(&schema.kind, replace_namespaces, crate_root, mod_root)?)?;

				let associated_type =
					get_fully_qualified_type_name(
						type_name_and_ref_path.as_ref()
							.map(|(_, type_ref_path)| type_ref_path)
							.ok_or_else(|| "DELETE-Ok-Status that isn't associated with a type")?,
						&replace_namespaces,
						crate_root,
						mod_root)?;
				if operation.kubernetes_action == Some(swagger20::KubernetesAction::DeleteCollection) {
					writeln!(out, "    {}Value({}List),", variant_name, associated_type)?;
				}
				else {
					writeln!(out, "    {}Value({}),", variant_name, associated_type)?;
				}
			}
			else {
				match &schema.kind {
					crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }) if path == "io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent" =>
						writeln!(
							out,
							"    {}({}<{}>),",
							variant_name,
							get_rust_type(&schema.kind, replace_namespaces, crate_root, mod_root)?,
							type_name_and_ref_path.as_ref()
								.map(|(type_name, _)| type_name)
								.ok_or_else(|| "WatchEvent operation that isn't associated with a type")?)?,

					_ => writeln!(out, "    {}({}),", variant_name, get_rust_type(&schema.kind, replace_namespaces, crate_root, mod_root)?)?,
				}
			}
		}

		writeln!(out, "    Other(Result<Option<serde_json::Value>, serde_json::Error>),")?;
		writeln!(out, "}}")?;
		writeln!(out)?;

		if mod_root.is_some() {
			writeln!(out, r#"#[cfg(feature = "api")]"#)?;
		}
		writeln!(out, "impl {}::Response for {} {{", crate_root, operation_result_name)?;
		writeln!(out, "    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {}::ResponseError> {{", crate_root)?;

		let is_watch = match operation.kubernetes_action {
			Some(swagger20::KubernetesAction::Watch) | Some(swagger20::KubernetesAction::WatchList) => true,
			_ => false,
		};

		writeln!(out, "        match status_code {{")?;
		for &(http_status_code, variant_name, schema, is_delete_ok_status) in &operation_responses {
			writeln!(out, "            http::StatusCode::{} => {{", http_status_code)?;

			match &schema.kind {
				swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => {
					writeln!(out, "                if buf.is_empty() {{")?;
					writeln!(out, "                    return Err({}::ResponseError::NeedMoreData);", crate_root)?;
					writeln!(out, "                }}")?;
					writeln!(out)?;
					writeln!(out, "                let (result, len) = match std::str::from_utf8(buf) {{")?;
					writeln!(out, "                    Ok(s) => (s, buf.len()),")?;
					writeln!(out, "                    Err(err) => match (err.valid_up_to(), err.error_len()) {{")?;
					writeln!(out, "                        (0, Some(_)) => return Err({}::ResponseError::Utf8(err)),", crate_root)?;
					writeln!(out, "                        (0, None) => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
					writeln!(out, "                        (valid_up_to, _) => (")?;
					writeln!(out, "                            unsafe {{ std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) }},")?;
					writeln!(out, "                            valid_up_to,")?;
					writeln!(out, "                        ),")?;
					writeln!(out, "                    }},")?;
					writeln!(out, "                }};")?;
					writeln!(out, "                Ok(({}::{}(result.to_owned()), len))", operation_result_name, variant_name)?;
				},

				swagger20::SchemaKind::Ref(_) => if is_watch {
					writeln!(out, "                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();")?;
					writeln!(out, "                let (result, byte_offset) = match deserializer.next() {{")?;
					writeln!(out, "                    Some(Ok(value)) => (value, deserializer.byte_offset()),")?;
					writeln!(out, "                    Some(Err(ref err)) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
					writeln!(out, "                    Some(Err(err)) => return Err({}::ResponseError::Json(err)),", crate_root)?;
					writeln!(out, "                    None => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
					writeln!(out, "                }};")?;
					writeln!(out, "                Ok(({}::{}(result), byte_offset))", operation_result_name, variant_name)?;
				}
				else if is_delete_ok_status {
					writeln!(out, "                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {{")?;
					writeln!(out, "                    Ok(value) => value,")?;
					writeln!(out, "                    Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
					writeln!(out, "                    Err(err) => return Err({}::ResponseError::Json(err)),", crate_root)?;
					writeln!(out, "                }};")?;
					writeln!(out, r#"                let is_status = match result.get("kind") {{"#)?;
					writeln!(out, r#"                    Some(serde_json::Value::String(s)) if s == "Status" => true,"#)?;
					writeln!(out, "                    _ => false,")?;
					writeln!(out, "                }};")?;
					writeln!(out, "                if is_status {{")?;
					writeln!(out, "                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));")?;
					writeln!(out, "                    let result = result.map_err({}::ResponseError::Json)?;", crate_root)?;
					writeln!(out, "                    Ok(({}::{}Status(result), buf.len()))", operation_result_name, variant_name)?;
					writeln!(out, "                }}")?;
					writeln!(out, "                else {{")?;
					writeln!(out, "                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));")?;
					writeln!(out, "                    let result = result.map_err({}::ResponseError::Json)?;", crate_root)?;
					writeln!(out, "                    Ok(({}::{}Value(result), buf.len()))", operation_result_name, variant_name)?;
					writeln!(out, "                }}")?;
				}
				else {
					writeln!(out, "                let result = match serde_json::from_slice(buf) {{")?;
					writeln!(out, "                    Ok(value) => value,")?;
					writeln!(out, "                    Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
					writeln!(out, "                    Err(err) => return Err({}::ResponseError::Json(err)),", crate_root)?;
					writeln!(out, "                }};")?;
					writeln!(out, "                Ok(({}::{}(result), buf.len()))", operation_result_name, variant_name)?;
				},

				other => return Err(format!("operation {} has unrecognized type for response of variant {}: {:?}", operation.id, variant_name, other).into()),
			}

			writeln!(out, "            }},")?;
		}
		writeln!(out, "            _ => {{")?;
		writeln!(out, "                let (result, read) =")?;
		writeln!(out, "                    if buf.is_empty() {{")?;
		writeln!(out, "                        (Ok(None), 0)")?;
		writeln!(out, "                    }}")?;
		writeln!(out, "                    else {{")?;
		writeln!(out, "                        match serde_json::from_slice(buf) {{")?;
		writeln!(out, "                            Ok(value) => (Ok(Some(value)), buf.len()),")?;
		writeln!(out, "                            Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
		writeln!(out, "                            Err(err) => (Err(err), 0),")?;
		writeln!(out, "                        }}")?;
		writeln!(out, "                    }};")?;
		writeln!(out, "                Ok(({}::Other(result), read))", operation_result_name)?;
		writeln!(out, "            }},")?;
		writeln!(out, "        }}")?;
		writeln!(out, "    }}")?;
		writeln!(out, "}}")?;
	}

	let mut result = (None, operation_result_name);
	if type_name_and_ref_path.is_some() && !optional_parameters.is_empty() {
		result.0 = Some(operation_optional_parameters_name);
	}
	Ok(result)
}

fn get_operation_names(
	operation: &swagger20::Operation,
	strip_tag: bool,
) -> Result<(std::borrow::Cow<'static, str>, Option<String>, String), Error> {
	let operation_id =
		if strip_tag {
			// For functions associatd with types (eg `Pod::list_core_v1_namespaced_pod`), the API version contained in the operation name
			// is already obvious from the type's path (`core::v1::Pod`), so it can be stripped (`list_namespaced_pod`).
			let tag: String =
				operation.tag.split('_')
				.map(|part| {
					let mut chars = part.chars();
					if let Some(first_char) = chars.next() {
						let rest_chars = chars.as_str();
						std::borrow::Cow::Owned(format!("{}{}", first_char.to_uppercase(), rest_chars))
					}
					else {
						std::borrow::Cow::Borrowed("")
					}
				})
				.collect();

			std::borrow::Cow::Owned(operation.id.replace(&tag, ""))
		}
		else {
			// Functions not associated with types (eg `::get_core_v1_api_resources`) get emitted at the mod root,
			// so their ID should be used as-is.
			std::borrow::Cow::Borrowed(&*operation.id)
		};

	let operation_fn_name = get_rust_ident(&operation_id);

	let mut chars = operation_id.chars();
	let first_char = chars.next().ok_or_else(|| format!("operation has empty ID: {:?}", operation))?.to_uppercase();
	let rest_chars = chars.as_str();
	let operation_result_name =
		if operation.kubernetes_action == Some(swagger20::KubernetesAction::Connect) {
			None
		}
		else {
			Some(format!("{}{}Response", first_char, rest_chars))
		};
	let operation_optional_parameters_name = format!("{}{}Optional", first_char, rest_chars);

	Ok((operation_fn_name, operation_result_name, operation_optional_parameters_name))
}
