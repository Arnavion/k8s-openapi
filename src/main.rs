#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cyclomatic_complexity,
	clippy::default_trait_access,
	clippy::similar_names,
	clippy::too_many_arguments,
	clippy::type_complexity,
	clippy::unseparated_literal_suffix,
	clippy::use_self,
)]

mod fixups;
mod supported_version;
mod swagger20;

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

fn main() -> Result<(), Error> {
	{
		let mut builder = env_logger::Builder::new();
		builder.format(|buf, record| {
			use std::io::Write;
			writeln!(buf, "{} {}:{} {}", record.level(), record.file().unwrap_or("?"), record.line().unwrap_or(0), record.args())
		});
		let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
		builder.parse(&rust_log);
		builder.init();
	}

	let client = reqwest::Client::new();

	let out_dir_base: &std::path::Path = env!("CARGO_MANIFEST_DIR").as_ref();
	let out_dir_base = out_dir_base.join("k8s-openapi").join("src");

	for &supported_version in supported_version::ALL {
		run(supported_version, &out_dir_base, &client)?;
	}

	Ok(())
}

fn run(supported_version: supported_version::SupportedVersion, out_dir_base: &std::path::Path, client: &reqwest::Client) -> Result<(), Error> {
	use std::io::Write;

	let mod_root = supported_version.mod_root();

	let out_dir = out_dir_base.join(mod_root);

	let replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])] = &[
		// Everything's under io.k8s, so strip it
		(&["io".into(), "k8s".into()], &[]),
	];

	let mut num_generated_structs = 0usize;
	let mut num_generated_type_aliases = 0usize;
	let mut num_generated_apis = 0usize;

	let mut spec: swagger20::Spec = {
		let spec_url = supported_version.spec_url();
		log::info!(target: "", "Parsing spec file at {} ...", spec_url);
		let mut response = client.get(spec_url).send()?;
		let status = response.status();
		if status != reqwest::StatusCode::OK {
			return Err(status.to_string().into());
		}
		response.json()?
	};

	supported_version.fixup(&mut spec)?;

	let expected_num_generated_types: usize = spec.definitions.len();
	let expected_num_generated_apis: usize = spec.operations.len();

	log::info!(
		"OK. Spec has {} definitions and {} operations",
		spec.definitions.len(),
		spec.operations.len());

	let mut operations_by_gkv: std::collections::BTreeMap<_, Vec<_>> = Default::default();
	for operation in &spec.operations {
		operations_by_gkv
		.entry(operation.kubernetes_group_kind_version.as_ref())
		.or_default()
		.push(operation);
	}
	for operations in operations_by_gkv.values_mut() {
		operations.sort_by_key(|operation| &operation.id);
	}

	loop {
		log::info!("Removing output directory {} ...", out_dir.display());
		match std::fs::remove_dir_all(&out_dir) {
			Ok(()) => log::trace!("OK"),
			Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
				log::trace!("OK. Directory doesn't exist");

				log::info!("Creating output directory {} ...", out_dir.display());
				match std::fs::create_dir(&out_dir) {
					Ok(()) => {
						log::trace!("OK");
						break;
					},
					Err(err) => log::warn!("Error: {}", err),
				}
			},
			Err(err) => log::warn!("Error: {}", err),
		}
	}

	log::info!("Generating types...");

	for (definition_path, definition) in &spec.definitions {
		log::trace!("Working on {} ...", definition_path);

		with_file_for_type(&definition_path, &out_dir, &replace_namespaces, |parent_mod_rs, mut file, type_name, type_ref_path| {
			writeln!(file, "// Generated from definition {}", definition_path)?;
			writeln!(file)?;

			if let Some(description) = &definition.description {
				for line in get_comment_text(description, "") {
					writeln!(file, "///{}", line)?;
				}
			}

			let can_be_default = can_be_default(&definition.kind, &spec)?;

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

							let type_name = get_rust_type(&schema.kind, &replace_namespaces, mod_root)?;

							if name.0 == "metadata" {
								metadata_property_ty = Some((*required, type_name.clone()));
							}

							// Fix cases of infinite recursion
							if let swagger20::SchemaKind::Ref(ref ref_path) = schema.kind {
								match (&**definition_path, &**name, &**ref_path) {
									(
										"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
										"not",
										"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
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

					write!(file, "#[derive(Clone, Debug")?;

					if can_be_default {
						write!(file, ", Default")?;
					}

					writeln!(file, ", PartialEq)]")?;

					writeln!(file, "pub struct {} {{", type_name)?;

					for (i, Property { schema, field_name, field_type_name, .. }) in properties.iter().enumerate() {
						if i > 0 {
							writeln!(file)?;
						}

						if let Some(ref description) = schema.description {
							for line in get_comment_text(description, "") {
								writeln!(file, "    ///{}", line)?;
							}
						}

						write!(file, "    pub {}: ", field_name)?;

						write!(file, "{}", field_type_name)?;

						writeln!(file, ",")?;
					}
					writeln!(file, "}}")?;

					if let Some(kubernetes_group_kind_versions) = &definition.kubernetes_group_kind_versions {
						let mut kubernetes_group_kind_versions: Vec<_> = kubernetes_group_kind_versions.iter().collect();
						kubernetes_group_kind_versions.sort();
						for kubernetes_group_kind_version in kubernetes_group_kind_versions {
							if let Some(operations) = operations_by_gkv.remove(&Some(kubernetes_group_kind_version)) {
								writeln!(file)?;
								writeln!(file, "// Begin {}/{}/{}",
									kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;

								for operation in operations {
									write_operation(
										&mut file,
										operation,
										&replace_namespaces,
										mod_root,
										&mut Some((&type_name, &type_ref_path, parent_mod_rs)),
									)?;
									num_generated_apis += 1;
								}

								writeln!(file)?;
								writeln!(file, "// End {}/{}/{}",
									kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;
							}
						}
					}

					if let Some(resource_metadata) = &resource_metadata {
						writeln!(file)?;
						writeln!(file, "impl crate::Resource for {} {{", type_name)?;
						writeln!(file, "    fn api_version() -> &'static str {{")?;
						writeln!(file, r#"        "{}""#, resource_metadata.0)?;
						writeln!(file, "    }}")?;
						writeln!(file)?;
						writeln!(file, "    fn group() -> &'static str {{")?;
						writeln!(file, r#"        "{}""#, resource_metadata.1)?;
						writeln!(file, "    }}")?;
						writeln!(file)?;
						writeln!(file, "    fn kind() -> &'static str {{")?;
						writeln!(file, r#"        "{}""#, resource_metadata.2)?;
						writeln!(file, "    }}")?;
						writeln!(file)?;
						writeln!(file, "    fn version() -> &'static str {{")?;
						writeln!(file, r#"        "{}""#, resource_metadata.3)?;
						writeln!(file, "    }}")?;
						writeln!(file, "}}")?;

						if let Some((required, ty)) = metadata_property_ty {
							writeln!(file)?;
							writeln!(file, "impl crate::Metadata for {} {{", type_name)?;
							writeln!(file, "    type Ty = {};", ty)?;
							writeln!(file)?;
							writeln!(file, "    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {{")?;
							if required {
								writeln!(file, "        Some(&self.metadata)")?;
							}
							else {
								writeln!(file, "        self.metadata.as_ref()")?;
							}
							writeln!(file, "    }}")?;
							writeln!(file, "}}")?;
						}
					}

					writeln!(file)?;
					writeln!(file, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
					writeln!(file, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
					writeln!(file, "        #[allow(non_camel_case_types)]")?;
					writeln!(file, "        enum Field {{")?;
					if resource_metadata.is_some() {
						writeln!(file, "            Key_api_version,")?;
						writeln!(file, "            Key_kind,")?;
					}
					for Property { field_name, .. } in &properties {
						writeln!(file, "            Key_{},", field_name)?;
					}
					writeln!(file, "            Other,")?;
					writeln!(file, "        }}")?;
					writeln!(file)?;
					writeln!(file, "        impl<'de> serde::Deserialize<'de> for Field {{")?;
					writeln!(file, "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
					writeln!(file, "                struct Visitor;")?;
					writeln!(file)?;
					writeln!(file, "                impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
					writeln!(file, "                    type Value = Field;")?;
					writeln!(file)?;
					writeln!(file, "                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
					writeln!(file, r#"                        write!(f, "field identifier")"#)?;
					writeln!(file, "                    }}")?;
					writeln!(file)?;
					writeln!(file, "                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(file, "                        Ok(match v {{")?;
					if resource_metadata.is_some() {
						writeln!(file, r#"                            "apiVersion" => Field::Key_api_version,"#)?;
						writeln!(file, r#"                            "kind" => Field::Key_kind,"#)?;
					}
					for Property { name, field_name, .. } in &properties {
						writeln!(file, r#"                            "{}" => Field::Key_{},"#, name, field_name)?;
					}
					writeln!(file, "                            _ => Field::Other,")?;
					writeln!(file, "                        }})")?;
					writeln!(file, "                    }}")?;
					writeln!(file, "                }}")?;
					writeln!(file)?;
					writeln!(file, "                deserializer.deserialize_identifier(Visitor)")?;
					writeln!(file, "            }}")?;
					writeln!(file, "        }}")?;
					writeln!(file)?;
					writeln!(file, "        struct Visitor;")?;
					writeln!(file)?;
					writeln!(file, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
					writeln!(file, "            type Value = {};", type_name)?;
					writeln!(file)?;
					writeln!(file, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
					writeln!(file, r#"                write!(f, "struct {}")"#, type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{")?;
					for Property { required, field_name, field_type_name, .. } in &properties {
						if *required {
							writeln!(file, r#"                let mut value_{}: Option<{}> = None;"#, field_name, field_type_name)?;
						}
						else {
							writeln!(file, r#"                let mut value_{}: {} = None;"#, field_name, field_type_name)?;
						}
					}
					writeln!(file)?;
					writeln!(file, "                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{")?;
					writeln!(file, "                    match key {{")?;
					if resource_metadata.is_some() {
							writeln!(file, r#"                        Field::Key_api_version => {{"#)?;
							writeln!(file, r#"                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;"#)?;
							writeln!(file, r#"                            if value_api_version != <Self::Value as crate::Resource>::api_version() {{"#)?;
							writeln!(file, r#"                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));"#)?;
							writeln!(file, r#"                            }}"#)?;
							writeln!(file, r#"                        }},"#)?;

							writeln!(file, r#"                        Field::Key_kind => {{"#)?;
							writeln!(file, r#"                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;"#)?;
							writeln!(file, r#"                            if value_kind != <Self::Value as crate::Resource>::kind() {{"#)?;
							writeln!(file, r#"                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));"#)?;
							writeln!(file, r#"                            }}"#)?;
							writeln!(file, r#"                        }},"#)?;
					}
					for Property { required, field_name, .. } in &properties {
						if *required {
							writeln!(file, r#"                        Field::Key_{} => value_{} = Some(serde::de::MapAccess::next_value(&mut map)?),"#, field_name, field_name)?;
						}
						else {
							writeln!(file, r#"                        Field::Key_{} => value_{} = serde::de::MapAccess::next_value(&mut map)?,"#, field_name, field_name)?;
						}
					}
					writeln!(file, "                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},")?;
					writeln!(file, "                    }}")?;
					writeln!(file, "                }}")?;
					writeln!(file)?;
					writeln!(file, "                Ok({} {{", type_name)?;
					for Property { name, required, field_name, .. } in &properties {
						if *required {
							writeln!(file, r#"                    {}: value_{}.ok_or_else(|| serde::de::Error::missing_field("{}"))?,"#, field_name, field_name, name)?;
						}
						else {
							writeln!(file, "                    {}: value_{},", field_name, field_name)?;
						}
					}
					writeln!(file, "                }})")?;
					writeln!(file, "            }}")?;
					writeln!(file, "        }}")?;
					writeln!(file)?;
					writeln!(file, "        deserializer.deserialize_struct(")?;
					writeln!(file, r#"            "{}","#, type_name)?;
					writeln!(file, "            &[")?;
					if resource_metadata.is_some() {
						writeln!(file, r#"                "apiVersion","#)?;
						writeln!(file, r#"                "kind","#)?;
					}
					for Property { name, .. } in &properties {
						writeln!(file, r#"                "{}","#, name)?;
					}
					writeln!(file, "            ],")?;
					writeln!(file, "            Visitor,")?;
					writeln!(file, "        )")?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;
					writeln!(file)?;

					writeln!(file, "impl serde::Serialize for {} {{", type_name)?;
					writeln!(file, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
					if properties.is_empty() && resource_metadata.is_none() {
						writeln!(file, "        let state = serializer.serialize_struct(")?;
					}
					else {
						writeln!(file, "        let mut state = serializer.serialize_struct(")?;
					}
					writeln!(file, r#"            "{}","#, type_name)?;

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
										writeln!(file, " +")?;
									}
									else {
										wrote_first_field = true;
									}
									write!(file, "            self.{}.as_ref().map_or(0, |_| 1)", field_name)?;
								}
							}
						},

						(0, false) =>
							write!(file, "            0")?,

						(num_required_fields, have_optional_fields) => {
							write!(file, "            {}", num_required_fields)?;
							if have_optional_fields {
								for Property { required, field_name, .. } in &properties {
									if !*required {
										writeln!(file, " +")?;
										write!(file, "            self.{}.as_ref().map_or(0, |_| 1)", field_name)?;
									}
								}
							}
						},
					}
					writeln!(file, ",")?;

					writeln!(file, "        )?;")?;
					if resource_metadata.is_some() {
						writeln!(file, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;"#)?;
						writeln!(file, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;"#)?;
					}
					for Property { name, required, field_name, .. } in &properties {
						if *required {
							writeln!(file, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "{}", &self.{})?;"#, name, field_name)?;
						}
						else {
							writeln!(file, "        if let Some(value) = &self.{} {{", field_name)?;
							writeln!(file, r#"            serde::ser::SerializeStruct::serialize_field(&mut state, "{}", value)?;"#, name)?;
							writeln!(file, "        }}")?;
						}
					}
					writeln!(file, "        serde::ser::SerializeStruct::end(state)")?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;

					num_generated_structs += 1;
				},

				swagger20::SchemaKind::Ref(_) => return Err(format!("{} is a Ref", definition_path).into()),

				swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => {
					writeln!(file, "#[derive(Clone, Debug, Eq, PartialEq)]")?;
					writeln!(file, "pub enum {} {{", type_name)?;
					writeln!(file, "    Int(i32),")?;
					writeln!(file, "    String(String),")?;
					writeln!(file, "}}")?;
					writeln!(file)?;
					writeln!(file, "impl Default for {} {{", type_name)?;
					writeln!(file, "    fn default() -> Self {{")?;
					writeln!(file, "        {}::Int(0)", type_name)?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;
					writeln!(file)?;
					writeln!(file, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
					writeln!(file, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
					writeln!(file, "        struct Visitor;")?;
					writeln!(file)?;
					writeln!(file, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
					writeln!(file, "            type Value = {};", type_name)?;
					writeln!(file)?;
					writeln!(file, "            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
					writeln!(file, r#"                write!(formatter, "enum {}")"#, type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(file, "                Ok({}::Int(v))", type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(file, "                if v < i64::from(i32::min_value()) || v > i64::from(i32::max_value()) {{")?;
					writeln!(file, r#"                    return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &"a 32-bit integer"));"#)?;
					writeln!(file, "                }}")?;
					writeln!(file)?;
					writeln!(file, "                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]")?;
					writeln!(file, "                Ok({}::Int(v as i32))", type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(file, "                #[allow(clippy::cast_sign_loss)]")?;
					writeln!(file, "                {{")?;
					writeln!(file, "                    if v > i32::max_value() as u64 {{")?;
					writeln!(file, r#"                        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"));"#)?;
					writeln!(file, "                    }}")?;
					writeln!(file, "                }}")?;
					writeln!(file)?;
					writeln!(file, "                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]")?;
					writeln!(file, "                Ok({}::Int(v as i32))", type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(file, "                self.visit_string(v.to_string())")?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
					writeln!(file, "                Ok({}::String(v))", type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file, "        }}")?;
					writeln!(file)?;
					writeln!(file, "        deserializer.deserialize_any(Visitor)")?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;
					writeln!(file)?;
					writeln!(file, "impl serde::Serialize for {} {{", type_name)?;
					writeln!(file, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
					writeln!(file, "        match self {{")?;
					writeln!(file, "            {}::Int(i) => i.serialize(serializer),", type_name)?;
					writeln!(file, "            {}::String(s) => s.serialize(serializer),", type_name)?;
					writeln!(file, "        }}")?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;

					num_generated_structs += 1;
				},

				swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrArray) |
				swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrBool) |
				swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrStringArray) => {
					let json_schema_props_type_name =
						get_fully_qualified_type_name(
							&swagger20::RefPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps".to_string()),
							&replace_namespaces,
							mod_root)?;

					writeln!(file, "#[derive(Clone, Debug, PartialEq)]")?;
					writeln!(file, "pub enum {} {{", type_name)?;
					writeln!(file, "    Schema(Box<{}>),", json_schema_props_type_name)?; // Box to fix infinite recursion
					match ty {
						swagger20::Type::JSONSchemaPropsOrArray => writeln!(file, "    Schemas(Vec<{}>),", json_schema_props_type_name)?,
						swagger20::Type::JSONSchemaPropsOrBool => writeln!(file, "    Bool(bool),")?,
						swagger20::Type::JSONSchemaPropsOrStringArray => writeln!(file, "    Strings(Vec<String>),")?,
						_ => unreachable!(),
					}
					writeln!(file, "}}")?;
					writeln!(file)?;
					writeln!(file, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
					writeln!(file, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
					writeln!(file, "        struct Visitor;")?;
					writeln!(file)?;
					writeln!(file, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
					writeln!(file, "            type Value = {};", type_name)?;
					writeln!(file)?;
					writeln!(file, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
					writeln!(file, r#"                write!(f, "enum {}")"#, type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{")?;
					writeln!(file, "                Ok({}::Schema(serde::de::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))", type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;

					match ty {
						swagger20::Type::JSONSchemaPropsOrArray => {
							writeln!(file, "            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {{")?;
							writeln!(file, "                Ok({}::Schemas(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))", type_name)?;
							writeln!(file, "            }}")?;
						},

						swagger20::Type::JSONSchemaPropsOrBool => {
							writeln!(file, "            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: serde::de::Error {{")?;
							writeln!(file, "                Ok({}::Bool(v))", type_name)?;
							writeln!(file, "            }}")?;
						},

						swagger20::Type::JSONSchemaPropsOrStringArray => {
							writeln!(file, "            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {{")?;
							writeln!(file, "                Ok({}::Strings(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))", type_name)?;
							writeln!(file, "            }}")?;
						},

						_ => unreachable!(),
					}

					writeln!(file, "        }}")?;
					writeln!(file)?;
					writeln!(file, "        deserializer.deserialize_any(Visitor)")?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;
					writeln!(file)?;
					writeln!(file, "impl serde::Serialize for {} {{", type_name)?;
					writeln!(file, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
					writeln!(file, "        match self {{")?;
					writeln!(file, "            {}::Schema(value) => value.serialize(serializer),", type_name)?;

					match ty {
						swagger20::Type::JSONSchemaPropsOrArray => writeln!(file, "            {}::Schemas(value) => value.serialize(serializer),", type_name)?,
						swagger20::Type::JSONSchemaPropsOrBool => writeln!(file, "            {}::Bool(value) => value.serialize(serializer),", type_name)?,
						swagger20::Type::JSONSchemaPropsOrStringArray => writeln!(file, "            {}::Strings(value) => value.serialize(serializer),", type_name)?,
						_ => unreachable!(),
					}

					writeln!(file, "        }}")?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;

					num_generated_structs += 1;
				},

				swagger20::SchemaKind::Ty(_) => {
					write!(file, "#[derive(Clone, Debug, ")?;
					if can_be_default {
						write!(file, "Default, ")?;
					}
					writeln!(file, "PartialEq)]")?;

					writeln!(file, "pub struct {}(pub {});", type_name, get_rust_type(&definition.kind, &replace_namespaces, mod_root)?)?;
					writeln!(file)?;
					writeln!(file, "impl<'de> serde::Deserialize<'de> for {} {{", type_name)?;
					writeln!(file, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{")?;
					writeln!(file, "        struct Visitor;")?;
					writeln!(file)?;
					writeln!(file, "        impl<'de> serde::de::Visitor<'de> for Visitor {{")?;
					writeln!(file, "            type Value = {};", type_name)?;
					writeln!(file)?;
					writeln!(file, "            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
					writeln!(file, r#"                write!(f, "{}")"#, type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file)?;
					writeln!(file, "            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {{")?;
					writeln!(file, "                Ok({}(serde::Deserialize::deserialize(deserializer)?))", type_name)?;
					writeln!(file, "            }}")?;
					writeln!(file, "        }}")?;
					writeln!(file)?;
					writeln!(file, r#"        deserializer.deserialize_newtype_struct("{}", Visitor)"#, type_name)?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;
					writeln!(file)?;
					writeln!(file, "impl serde::Serialize for {} {{", type_name)?;
					writeln!(file, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{")?;
					writeln!(file, r#"        serializer.serialize_newtype_struct("{}", &self.0)"#, type_name)?;
					writeln!(file, "    }}")?;
					writeln!(file, "}}")?;

					num_generated_type_aliases += 1;
				},

				swagger20::SchemaKind::Watch => unreachable!(),
			}

			log::trace!("OK");

			Ok(())
		})?;
	}

	{
		let mut mod_root_file = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).open(out_dir.join("mod.rs"))?);

		for (kubernetes_group_kind_version, operations) in operations_by_gkv {
			for operation in operations {
				if let Some(swagger20::KubernetesGroupKindVersion { group, kind, version }) = kubernetes_group_kind_version {
					return Err(format!(
						"Operation {} is associated with {}/{}/{} but did not get emitted with that definition",
						operation.id, group, version, kind).into());
				}

				write_operation(
					&mut mod_root_file,
					operation,
					&replace_namespaces,
					mod_root,
					&mut None,
				)?;
				num_generated_apis += 1;
			}
		}
	}

	log::info!("OK");
	log::info!("Generated {} structs", num_generated_structs);
	log::info!("Generated {} type aliases", num_generated_type_aliases);
	log::info!("Generated {} API functions", num_generated_apis);

	if num_generated_structs + num_generated_type_aliases != expected_num_generated_types {
		return Err("Did not generate or skip expected number of types".into());
	}

	if num_generated_apis != expected_num_generated_apis {
		return Err("Did not generate expected number of API functions".into());
	}

	log::info!("");

	Ok(())
}

fn can_be_default(kind: &swagger20::SchemaKind, spec: &swagger20::Spec) -> Result<bool, Error> {
	match kind {
		swagger20::SchemaKind::Properties(properties) => {
			for (schema, required) in properties.values() {
				if !required {
					// Option<T>::default is None regardless of T
					continue;
				}

				if !can_be_default(&schema.kind, spec)? {
					return Ok(false);
				}
			}

			Ok(true)
		},

		swagger20::SchemaKind::Ref(ref_path) => {
			let target =
				spec.definitions.get(&swagger20::DefinitionPath(ref_path.0.clone()))
				.ok_or_else(|| format!("couldn't find target of ref path {}", ref_path))?;
			can_be_default(&target.kind, spec)
		},

		// chrono::DateTime<chrono::Utc> is not Default
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok(false),

		swagger20::SchemaKind::Ty(_) => Ok(true),

		swagger20::SchemaKind::Watch => unreachable!(),
	}
}

fn with_file_for_type<F>(
	definition_path: &swagger20::DefinitionPath,
	out_dir: &std::path::Path,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	f: F,
) -> Result<(), Error>
where
	F: FnOnce(&mut std::io::BufWriter<std::fs::File>, std::io::BufWriter<std::fs::File>, String, swagger20::RefPath) -> Result<(), Error>,
{
	use std::io::Write;

	let parts = replace_namespace(definition_path.split('.'), replace_namespaces);

	let mut current = out_dir.to_owned();

	for part in parts.iter().rev().skip(1).rev() {
		log::trace!("Current directory: {}", current.display());

		let mod_name = get_rust_ident(part);

		current.push(&*mod_name);

		log::trace!("Checking if subdirectory {} exists...", current.display());

		if !current.is_dir() {
			log::trace!("    Subdirectory does not exist. Creating mod.rs with a reference to it...");

			let current_mod_rs_path = current.with_file_name("mod.rs");
			let append_newline = current_mod_rs_path.exists();
			let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current_mod_rs_path)?);
			if append_newline {
				writeln!(parent_mod_rs)?;
			}
			writeln!(parent_mod_rs, "pub mod {};", mod_name)?;

			log::trace!("    OK");
			log::trace!("    Creating subdirectory...");

			std::fs::create_dir(&current)?;
			log::trace!("    OK");
		}

		log::trace!("OK");
	}

	let type_name = parts.last().ok_or_else(|| format!("path for {} has no parts", definition_path))?.to_string();

	let mod_name = get_rust_ident(&type_name);

	let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current.join("mod.rs"))?);
	writeln!(parent_mod_rs)?;
	writeln!(parent_mod_rs, "mod {};", mod_name)?;
	writeln!(parent_mod_rs, "pub use self::{}::{{", mod_name)?;
	writeln!(parent_mod_rs, "    {},", type_name)?;

	let file_name = current.join(&*mod_name).with_extension("rs");
	let file = std::io::BufWriter::new(std::fs::File::create(file_name)?);

	let ref_path = swagger20::RefPath(definition_path.0.to_string());

	f(&mut parent_mod_rs, file, type_name, ref_path)?;

	writeln!(parent_mod_rs, "}};")?;

	Ok(())
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
	mod_root: &str,
) -> Result<String, Error> {
	use std::fmt::Write;

	let mut result = format!("crate::{}", mod_root);

	let parts = replace_namespace(ref_path.split('.'), replace_namespaces);

	for part in parts.iter().rev().skip(1).rev() {
		write!(result, "::{}", get_rust_ident(part))?;
	}

	write!(result, "::{}", parts.last().ok_or_else(|| format!("path for {} has no parts", ref_path))?)?;

	Ok(result)
}

fn get_rust_ident(name: &str) -> std::borrow::Cow<'static, str> {
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
	mod_root: &str,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	match *schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(ref ref_path) => Ok(format!("&{}", get_fully_qualified_type_name(ref_path, replace_namespaces, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok("&serde_json::Value".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { ref items }) => Ok(format!("&[{}]", get_rust_type(&items.kind, replace_namespaces, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { ref additional_properties }) =>
			Ok(format!("&std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => Ok("&crate::ByteString".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok("&chrono::DateTime<chrono::Utc>".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("&str".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray) => Err("JSON schema types not supported".into()),

		swagger20::SchemaKind::Watch => Ok("".into()), // Value is unused since this parameter is implicit
	}
}

fn get_rust_type(
	schema_kind: &swagger20::SchemaKind,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	mod_root: &str,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	match *schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(ref ref_path) => Ok(get_fully_qualified_type_name(ref_path, replace_namespaces, mod_root)?.into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok("serde_json::Value".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { ref items }) => Ok(format!("Vec<{}>", get_rust_type(&items.kind, replace_namespaces, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { ref additional_properties }) =>
			Ok(format!("std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, mod_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => Ok("crate::ByteString".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok("chrono::DateTime<chrono::Utc>".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("String".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray) => Err("JSON schema types not supported".into()),

		swagger20::SchemaKind::Watch => unreachable!(),
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

fn write_operation(
	file: &mut std::io::BufWriter<std::fs::File>,
	operation: &swagger20::Operation,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	mod_root: &str,
	type_name_and_ref_path_and_parent_mod_rs: &mut Option<(&str, &swagger20::RefPath, &mut std::io::BufWriter<std::fs::File>)>,
) -> Result<(), Error> {
	use std::io::Write;

	writeln!(file)?;

	writeln!(file, "// Generated from operation {}", operation.id)?;

	let (operation_fn_name, operation_result_name, operation_optional_parameters_name) =
		get_operation_names(operation, type_name_and_ref_path_and_parent_mod_rs.is_some())?;

	let operation_responses: Result<Vec<_>, _> =
		operation.responses.iter()
		.map(|(&status_code, schema)| {
			let http_status_code = match status_code {
				reqwest::StatusCode::ACCEPTED => "ACCEPTED",
				reqwest::StatusCode::CREATED => "CREATED",
				reqwest::StatusCode::OK => "OK",
				reqwest::StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
				_ => return Err(format!("unrecognized status code {}", status_code)),
			};

			let variant_name = match status_code {
				reqwest::StatusCode::ACCEPTED => "Accepted",
				reqwest::StatusCode::CREATED => "Created",
				reqwest::StatusCode::OK => "Ok",
				reqwest::StatusCode::UNAUTHORIZED => "Unauthorized",
				_ => return Err(format!("unrecognized status code {}", status_code)),
			};

			let schema = schema.as_ref();

			let is_delete_ok_status = if let Some(schema) = schema {
				match &schema.kind {
					swagger20::SchemaKind::Ref(ref_path) if
						&**ref_path == "io.k8s.apimachinery.pkg.apis.meta.v1.Status" &&
						operation.method == swagger20::Method::Delete &&
						status_code == reqwest::StatusCode::OK => true,

					_ => false,
				}
			}
			else {
				false
			};

			Ok((http_status_code, variant_name, schema, is_delete_ok_status))
		})
		.collect();
	let operation_responses = operation_responses?;

	let indent = if type_name_and_ref_path_and_parent_mod_rs.is_some() { "    " } else { "" };

	writeln!(file)?;

	if let Some((type_name, _, _)) = type_name_and_ref_path_and_parent_mod_rs {
		writeln!(file, "impl {} {{", type_name)?;
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

			let parameter_type = match get_rust_borrow_type(&parameter.schema.kind, replace_namespaces, mod_root) {
				Ok(parameter_type) => parameter_type,
				Err(err) => return Err(err),
			};

			Ok((parameter_name, parameter_type, parameter))
		})
		.collect();
	let mut parameters = parameters?;
	let watch_parameter =
		if let Some(index) = parameters.iter().position(|(_, _, p)| if let swagger20::SchemaKind::Watch = p.schema.kind { true } else { false }) {
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

	let mut wrote_description = false;
	if let Some(description) = operation.description.as_ref() {
		for line in get_comment_text(description, "") {
			writeln!(file, "{}///{}", indent, line)?;
			wrote_description = true;
		}
	}

	if wrote_description {
		writeln!(file, "{}///", indent)?;
	}

	writeln!(file,
		"{}/// Use the returned [`crate::ResponseBody`]`<`[`{}`]`>` constructor, or [`{}`] directly, to parse the HTTP response.",
		indent, operation_result_name, operation_result_name)?;

	if !parameters.is_empty() {
		writeln!(file, "{}///", indent)?;
		writeln!(file, "{}/// # Arguments", indent)?;
		for (parameter_name, _, parameter) in &required_parameters {
			writeln!(file, "{}///", indent)?;
			writeln!(file, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(file, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(file, "{}///{}", indent, line)?;
				}
			}
		}
		if !optional_parameters.is_empty() {
			writeln!(file, "{}///", indent)?;
			writeln!(file, "{}/// * `optional`", indent)?;
			writeln!(file, "{}///", indent)?;
			writeln!(file, "{}///     Optional parameters. Use `Default::default()` to not pass any.", indent)?;
		}
	}

	writeln!(file, "{}pub fn {}(", indent, operation_fn_name)?;
	for (parameter_name, parameter_type, _) in &required_parameters {
		writeln!(file, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if !optional_parameters.is_empty() {
		write!(file, "{}    optional: {}", indent, operation_optional_parameters_name)?;
		if any_optional_fields_have_lifetimes {
			write!(file, "<'_>")?;
		}
		writeln!(file, ",")?;
	}
	writeln!(file, "{}) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<{}>), crate::RequestError> {{", indent, operation_result_name)?;

	let have_path_parameters = parameters.iter().any(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Path);
	let have_query_parameters = parameters.iter().any(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Query) || watch_parameter.is_some();

	if !optional_parameters.is_empty() {
		writeln!(file, "{}    let {} {{", indent, operation_optional_parameters_name)?;
		for (parameter_name, _, _) in &optional_parameters {
			writeln!(file, "{}        {},", indent, parameter_name)?;
		}

		writeln!(file, "{}    }} = optional;", indent)?;
	}

	if have_path_parameters {
		write!(file, r#"{}    let __url = format!("{}"#, indent, operation.path)?;
		if have_query_parameters {
			write!(file, "?")?;
		}
		write!(file, r#"""#)?;
		for (parameter_name, _, parameter) in &parameters {
			if parameter.location == swagger20::ParameterLocation::Path {
				write!(file, ", {} = {}", parameter_name, parameter_name)?;
			}
		}
		writeln!(file, ");")?;
	}
	else {
		write!(file, r#"{}    let __url = "{}"#, indent, operation.path)?;
		if have_query_parameters {
			write!(file, "?")?;
		}
		writeln!(file, r#"".to_string();"#)?;
	}

	if have_query_parameters {
		writeln!(file, "{}    let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);", indent)?;
		for (parameter_name, parameter_type, parameter) in parameters.iter().chain(&watch_parameter) {
			if parameter.location == swagger20::ParameterLocation::Query {
				if parameter.required {
					match parameter.schema.kind {
						swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
						swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
						swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
							writeln!(file, r#"{}    __query_pairs.append_pair("{}", &{}.to_string());"#, indent, parameter.name, parameter_name)?,

						swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
							writeln!(file, r#"{}    __query_pairs.append_pair("{}", &{});"#, indent, parameter.name, parameter_name)?,

						swagger20::SchemaKind::Watch =>
							writeln!(file, r#"{}    __query_pairs.append_pair("{}", "true");"#, indent, parameter.name)?,

						_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter_name, parameter_type).into()),
					}
				}
				else {
					writeln!(file, "{}    if let Some({}) = {} {{", indent, parameter_name, parameter_name)?;
					match parameter.schema.kind {
						swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
						swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
						swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
							writeln!(file, r#"{}        __query_pairs.append_pair("{}", &{}.to_string());"#, indent, parameter.name, parameter_name)?,

						swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
							writeln!(file, r#"{}        __query_pairs.append_pair("{}", {});"#, indent, parameter.name, parameter_name)?,

						_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter_name, parameter_type).into()),
					}
					writeln!(file, "{}    }}", indent)?;
				}
			}
		}
		writeln!(file, "{}    let __url = __query_pairs.finish();", indent)?;
	}
	writeln!(file)?;

	let method = match operation.method {
		swagger20::Method::Delete => "delete",
		swagger20::Method::Get => "get",
		swagger20::Method::Patch => "patch",
		swagger20::Method::Post => "post",
		swagger20::Method::Put => "put",
	};

	writeln!(file, "{}    let mut __request = http::Request::{}(__url);", indent, method)?;

	let body_parameter = parameters.iter().find(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Body);

	write!(file, "{}    let __body = ", indent)?;
	if let Some((parameter_name, _, parameter)) = body_parameter {
		if parameter.required {
			writeln!(file, "serde_json::to_vec(&{}).map_err(crate::RequestError::Json)?;", parameter_name)?;
		}
		else {
			writeln!(file)?;
			writeln!(file, "{}.unwrap_or(Ok(vec![]), |value| serde_json::to_vec(value).map_err(crate::RequestError::Json))?;", parameter_name)?;
		}
	}
	else {
		writeln!(file, "vec![];")?;
	}

	writeln!(file, "{}    match __request.body(__body) {{", indent)?;
	writeln!(file, "{}        Ok(body) => Ok((body, crate::ResponseBody::new)),", indent)?;
	writeln!(file, "{}        Err(err) => Err(crate::RequestError::Http(err)),", indent)?;
	writeln!(file, "{}    }}", indent)?;
	writeln!(file, "{}}}", indent)?;

	if type_name_and_ref_path_and_parent_mod_rs.is_some() {
		writeln!(file, "}}")?;
	}

	if let Some((_, _, parent_mod_rs)) = type_name_and_ref_path_and_parent_mod_rs {
		if optional_parameters.is_empty() {
			writeln!(parent_mod_rs, "    {},", operation_result_name)?;
		}
		else {
			writeln!(parent_mod_rs, "    {}, {},", operation_optional_parameters_name, operation_result_name)?;
		}
	}

	if !optional_parameters.is_empty() {
		writeln!(file)?;

		if let Some((type_name, _, _)) = type_name_and_ref_path_and_parent_mod_rs {
			writeln!(file, "/// Optional parameters of [`{}::{}`]", type_name, operation_fn_name)?;
		}
		else {
			writeln!(file, "/// Optional parameters of [`{}`]", operation_fn_name)?;
		}

		writeln!(file, "#[derive(Clone, Copy, Debug, Default)]")?;
		write!(file, "pub struct {}", operation_optional_parameters_name)?;
		if any_optional_fields_have_lifetimes {
			write!(file, "<'a>")?;
		}
		writeln!(file, " {{")?;

		for (parameter_name, parameter_type, parameter) in &optional_parameters {
			if let Some(description) = parameter.schema.description.as_ref() {
				for line in get_comment_text(description, "") {
					writeln!(file, "    ///{}", line)?;
				}
			}
			if parameter_type.starts_with('&') {
				writeln!(file, "    pub {}: Option<&'a {}>,", parameter_name, &parameter_type[1..])?;
			}
			else {
				writeln!(file, "    pub {}: Option<{}>,", parameter_name, parameter_type)?;
			}
		}

		writeln!(file, "}}")?;
	}

	writeln!(file)?;

	if let Some((type_name, _, _)) = type_name_and_ref_path_and_parent_mod_rs {
		writeln!(file, "/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}::{}`]", operation_result_name, type_name, operation_fn_name)?;
	}
	else {
		writeln!(file, "/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}`]", operation_result_name, operation_fn_name)?;
	}

	writeln!(file, "#[derive(Debug)]")?;
	writeln!(file, "pub enum {} {{", operation_result_name)?;

	for &(_, variant_name, schema, is_delete_ok_status) in &operation_responses {
		if let Some(schema) = schema {
			if is_delete_ok_status {
				// DELETE operations that return metav1.Status for HTTP 200 can also return the object itself instead.
				//
				// Ref https://github.com/kubernetes/kubernetes/issues/59501
				writeln!(file, "    {}Status({}),", variant_name, get_rust_type(&schema.kind, replace_namespaces, mod_root)?)?;
				writeln!(file, "    {}Value({}),", variant_name, get_fully_qualified_type_name(
					type_name_and_ref_path_and_parent_mod_rs.as_ref()
						.map(|(_, type_ref_path, _)| type_ref_path)
						.ok_or_else(|| "DELETE-Ok-Status that isn't associated with a type")?,
					&replace_namespaces,
					mod_root)?)?;
			}
			else {
				writeln!(file, "    {}({}),", variant_name, get_rust_type(&schema.kind, replace_namespaces, mod_root)?)?;
			}
		}
		else {
			writeln!(file, "    {},", variant_name)?;
		}
	}
	writeln!(file, "    Other,")?;
	writeln!(file, "}}")?;
	writeln!(file)?;

	writeln!(file, "impl crate::Response for {} {{", operation_result_name)?;

	let uses_buf = operation_responses.iter().any(|&(_, _, schema, _)| schema.is_some());

	if uses_buf {
		writeln!(file, "    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {{")?;
	}
	else {
		writeln!(file, "    fn try_from_parts(status_code: http::StatusCode, _: &[u8]) -> Result<(Self, usize), crate::ResponseError> {{")?;
	}

	let is_watch = match operation.kubernetes_action {
		Some(swagger20::KubernetesAction::Watch) | Some(swagger20::KubernetesAction::WatchList) => true,
		_ => false,
	};

	writeln!(file, "        match status_code {{")?;
	for &(http_status_code, variant_name, schema, is_delete_ok_status) in &operation_responses {
		write!(file, "            http::StatusCode::{} => ", http_status_code)?;
		if let Some(schema) = schema {
			writeln!(file, "{{")?;

			match &schema.kind {
				swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => {
					writeln!(file, "                if buf.is_empty() {{")?;
					writeln!(file, "                    return Err(crate::ResponseError::NeedMoreData);")?;
					writeln!(file, "                }}")?;
					writeln!(file)?;
					writeln!(file, "                let (result, len) = match std::str::from_utf8(buf) {{")?;
					writeln!(file, "                    Ok(s) => (s, buf.len()),")?;
					writeln!(file, "                    Err(err) => match (err.valid_up_to(), err.error_len()) {{")?;
					writeln!(file, "                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),")?;
					writeln!(file, "                        (0, None) => return Err(crate::ResponseError::NeedMoreData),")?;
					writeln!(file, "                        (valid_up_to, _) => (")?;
					writeln!(file, "                            unsafe {{ std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) }},")?;
					writeln!(file, "                            valid_up_to,")?;
					writeln!(file, "                        ),")?;
					writeln!(file, "                    }},")?;
					writeln!(file, "                }};")?;
					writeln!(file, "                Ok(({}::{}(result.to_string()), len))", operation_result_name, variant_name)?;
				},

				swagger20::SchemaKind::Ref(_) => if is_watch {
					writeln!(file, "                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();")?;
					writeln!(file, "                let (result, byte_offset) = match deserializer.next() {{")?;
					writeln!(file, "                    Some(Ok(value)) => (value, deserializer.byte_offset()),")?;
					writeln!(file, "                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),")?;
					writeln!(file, "                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),")?;
					writeln!(file, "                    None => return Err(crate::ResponseError::NeedMoreData),")?;
					writeln!(file, "                }};")?;
					writeln!(file, "                Ok(({}::{}(result), byte_offset))", operation_result_name, variant_name)?;
				}
				else if is_delete_ok_status {
					writeln!(file, "                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {{")?;
					writeln!(file, "                    Ok(value) => value,")?;
					writeln!(file, "                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),")?;
					writeln!(file, "                    Err(err) => return Err(crate::ResponseError::Json(err)),")?;
					writeln!(file, "                }};")?;
					writeln!(file, r#"                let is_status = match result.get("kind") {{"#)?;
					writeln!(file, r#"                    Some(serde_json::Value::String(s)) if s == "Status" => true,"#)?;
					writeln!(file, "                    _ => false,")?;
					writeln!(file, "                }};")?;
					writeln!(file, "                if is_status {{")?;
					writeln!(file, "                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));")?;
					writeln!(file, "                    let result = result.map_err(crate::ResponseError::Json)?;")?;
					writeln!(file, "                    Ok(({}::{}Status(result), buf.len()))", operation_result_name, variant_name)?;
					writeln!(file, "                }}")?;
					writeln!(file, "                else {{")?;
					writeln!(file, "                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));")?;
					writeln!(file, "                    let result = result.map_err(crate::ResponseError::Json)?;")?;
					writeln!(file, "                    Ok(({}::{}Value(result), buf.len()))", operation_result_name, variant_name)?;
					writeln!(file, "                }}")?;
				}
				else {
					writeln!(file, "                let result = match serde_json::from_slice(buf) {{")?;
					writeln!(file, "                    Ok(value) => value,")?;
					writeln!(file, "                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),")?;
					writeln!(file, "                    Err(err) => return Err(crate::ResponseError::Json(err)),")?;
					writeln!(file, "                }};")?;
					writeln!(file, "                Ok(({}::{}(result), buf.len()))", operation_result_name, variant_name)?;
				},

				other => return Err(format!("operation {} has unrecognized type for response of variant {}: {:?}", operation.id, variant_name, other).into()),
			}

			writeln!(file, "            }},")?;
		}
		else {
			writeln!(file, "Ok(({}::{}, 0)),", operation_result_name, variant_name)?;
		}
	}
	writeln!(file, "            _ => Ok(({}::Other, 0)),", operation_result_name)?;
	writeln!(file, "        }}")?;
	writeln!(file, "    }}")?;
	writeln!(file, "}}")?;

	Ok(())
}

fn get_operation_names(
	operation: &swagger20::Operation,
	strip_tag: bool,
) -> Result<(std::borrow::Cow<'static, str>, String, String), Error> {
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
	let operation_result_name = format!("{}{}Response", first_char, rest_chars);
	let operation_optional_parameters_name = format!("{}{}Optional", first_char, rest_chars);

	Ok((operation_fn_name, operation_result_name, operation_optional_parameters_name))
}
