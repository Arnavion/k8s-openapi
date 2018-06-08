#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(
	cyclomatic_complexity,
	unseparated_literal_suffix,
))]

extern crate backtrace;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod swagger20;

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

	run("https://raw.githubusercontent.com/kubernetes/kubernetes/v1.7.16/api/openapi-spec/swagger.json", &out_dir_base, "v1_7", &client)?;
	run("https://raw.githubusercontent.com/kubernetes/kubernetes/v1.8.13/api/openapi-spec/swagger.json", &out_dir_base, "v1_8", &client)?;
	run("https://raw.githubusercontent.com/kubernetes/kubernetes/v1.9.8/api/openapi-spec/swagger.json", &out_dir_base, "v1_9", &client)?;
	run("https://raw.githubusercontent.com/kubernetes/kubernetes/v1.10.4/api/openapi-spec/swagger.json", &out_dir_base, "v1_10", &client)?;

	Ok(())
}

fn run(input: &str, out_dir_base: &std::path::Path, mod_root: &str, client: &reqwest::Client) -> Result<(), Error> {
	use std::io::Write;

	let out_dir = out_dir_base.join(mod_root);

	// `$ref`s under these namespaces will not be emitted
	let skip_refs_under_namespaces = vec![
		// All marked deprecated and point to corresponding definitions under io.k8s.api
		vec!["io", "k8s", "kubernetes", "pkg"],
	];

	let replace_namespaces: Vec<(_, Vec<String>)> = vec![
		// Everything's under io.k8s, so strip it
		(vec!["io", "k8s"], vec![]),
	];

	let mut num_generated_structs = 0usize;
	let mut num_generated_type_aliases = 0usize;
	let mut num_skipped_refs = 0usize;

	info!(target: "", "Parsing spec file at {} ...", input);
	let spec: swagger20::Spec = {
		let mut response = client.get(input).send()?;
		let status = response.status();
		if status != reqwest::StatusCode::Ok {
			return Err(status.to_string().into());
		}
		response.json()?
	};
	info!("OK. Spec has {} definitions", spec.definitions.len());

	info!("Removing output directory {} ...", out_dir.display());
	match std::fs::remove_dir_all(&out_dir) {
		Ok(()) => trace!("OK"),
		Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => trace!("OK. Directory doesn't exist"),
		err => err?,
	}

	info!("Creating output directory {} ...", out_dir.display());
	std::fs::create_dir(&out_dir)?;
	trace!("OK");

	info!("Generating types...");

	for (definition_path, definition) in spec.definitions {
		trace!("Working on {} ...", definition_path);

		if let swagger20::SchemaKind::Ref(_) = definition.kind {
			let parts: Vec<_> = definition_path.split('.').collect();
			if skip_refs_under_namespaces.iter().any(|skip_refs_under_namespace| parts.starts_with(skip_refs_under_namespace)) {
				trace!("Skipping ref {} because its namespace is to be skipped", definition_path);
				num_skipped_refs += 1;
				continue;
			}
		}

		let (mut file, type_name) = create_file_for_type(&definition_path, &out_dir, &replace_namespaces)?;

		writeln!(file, "// Generated from definition {}", definition_path)?;
		writeln!(file)?;

		if let Some(description) = definition.description {
			for line in get_comment_text(&description) {
				writeln!(file, "{}", line)?;
			}
		}

		match definition.kind {
			swagger20::SchemaKind::Properties(properties) => {
				struct Property {
					name: swagger20::PropertyName,
					schema: swagger20::Schema,
					required: bool,
					field_name: std::borrow::Cow<'static, str>,
					field_type_name: String,
				}

				let properties = {
					use std::fmt::Write;

					let mut result = Vec::with_capacity(properties.len());

					for (name, (schema, required)) in properties {
						let field_name = get_rust_ident(&name);

						let mut field_type_name = String::new();

						if !required {
							write!(field_type_name, "Option<")?;
						}

						let type_name = get_rust_type(&schema.kind, &replace_namespaces, mod_root);

						// Fix cases of infinite recursion
						if let swagger20::SchemaKind::Ref(ref ref_path) = schema.kind {
							match (&*definition_path, &*name, &**ref_path) {
								(
									"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrArray",
									"Schema",
									"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
								) |

								(
									"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrBool",
									"Schema",
									"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
								) |

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
							required,
							field_name,
							field_type_name,
						});
					}

					result
				};

				write!(file, "#[derive(Debug")?;

				let all_properties_are_default =
					properties.iter().all(|Property { schema, required, .. }| {
						if !required {
							// Option<T>::default is None regardless of T
							return true;
						}

						if let swagger20::SchemaKind::Ref(ref ref_path) = schema.kind {
							match &**ref_path {
								// chrono::DateTime<chrono::Utc> is not Default
								"io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime" |
								"io.k8s.apimachinery.pkg.apis.meta.v1.Time" => false,
								_ => true,
							}
						}
						else {
							true
						}
					});
				if all_properties_are_default {
					write!(file, ", Default")?;
				}

				writeln!(file, ")]")?;

				writeln!(file, "pub struct {} {{", type_name)?;

				for (i, Property { schema, field_name, field_type_name, .. }) in properties.iter().enumerate() {
					if i > 0 {
						writeln!(file)?;
					}

					if let Some(ref description) = schema.description {
						for line in get_comment_text(description) {
							writeln!(file, "    {}", line)?;
						}
					}

					write!(file, "    pub {}: ", field_name)?;

					write!(file, "{}", field_type_name)?;

					writeln!(file, ",")?;
				}
				writeln!(file, "}}")?;
				writeln!(file)?;

				writeln!(file, "impl<'de> ::serde::Deserialize<'de> for {} {{", type_name)?;
				writeln!(file, "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {{")?;
				writeln!(file, "        #[allow(non_camel_case_types)]")?;
				writeln!(file, "        enum Field {{")?;
				for Property { field_name, .. } in &properties {
					writeln!(file, "            Key_{},", field_name)?;
				}
				writeln!(file, "            Other,")?;
				writeln!(file, "        }}")?;
				writeln!(file)?;
				writeln!(file, "        impl<'de> ::serde::Deserialize<'de> for Field {{")?;
				writeln!(file, "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {{")?;
				writeln!(file, "                struct Visitor;")?;
				writeln!(file)?;
				writeln!(file, "                impl<'de> ::serde::de::Visitor<'de> for Visitor {{")?;
				writeln!(file, "                    type Value = Field;")?;
				writeln!(file)?;
				writeln!(file, "                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{")?;
				writeln!(file, r#"                        write!(f, "field identifier")"#)?;
				writeln!(file, "                    }}")?;
				writeln!(file)?;
				writeln!(file, "                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {{")?;
				writeln!(file, "                        Ok(match v {{")?;
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
				writeln!(file, "        impl<'de> ::serde::de::Visitor<'de> for Visitor {{")?;
				writeln!(file, "            type Value = {};", type_name)?;
				writeln!(file)?;
				writeln!(file, "            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{")?;
				writeln!(file, r#"                write!(f, "struct {}")"#, type_name)?;
				writeln!(file, "            }}")?;
				writeln!(file)?;
				writeln!(file, "            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {{")?;
				for Property { required, field_name, field_type_name, .. } in &properties {
					if *required {
						writeln!(file, r#"                let mut value_{}: Option<{}> = None;"#, field_name, field_type_name)?;
					}
					else {
						writeln!(file, r#"                let mut value_{}: {} = None;"#, field_name, field_type_name)?;
					}
				}
				writeln!(file)?;
				writeln!(file, "                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {{")?;
				writeln!(file, "                    match key {{")?;
				for Property { required, field_name, .. } in &properties {
					if *required {
						writeln!(file, r#"                        Field::Key_{} => value_{} = Some(::serde::de::MapAccess::next_value(&mut map)?),"#, field_name, field_name)?;
					}
					else {
						writeln!(file, r#"                        Field::Key_{} => value_{} = ::serde::de::MapAccess::next_value(&mut map)?,"#, field_name, field_name)?;
					}
				}
				writeln!(file, "                        Field::Other => {{ let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; }},")?;
				writeln!(file, "                    }}")?;
				writeln!(file, "                }}")?;
				writeln!(file)?;
				writeln!(file, "                Ok({} {{", type_name)?;
				for Property { name, required, field_name, .. } in &properties {
					if *required {
						writeln!(file, r#"                    {}: value_{}.ok_or_else(|| ::serde::de::Error::missing_field("{}"))?,"#, field_name, field_name, name)?;
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
				for Property { name, .. } in &properties {
					writeln!(file, r#"                "{}","#, name)?;
				}
				writeln!(file, "            ],")?;
				writeln!(file, "            Visitor,")?;
				writeln!(file, "        )")?;
				writeln!(file, "    }}")?;
				writeln!(file, "}}")?;
				writeln!(file)?;

				writeln!(file, "impl ::serde::Serialize for {} {{", type_name)?;
				writeln!(file, "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {{")?;
				if properties.is_empty() {
					writeln!(file, "        let state = serializer.serialize_struct(")?;
				}
				else {
					writeln!(file, "        let mut state = serializer.serialize_struct(")?;
				}
				writeln!(file, r#"            "{}","#, type_name)?;
				write!(file, "            0")?;
				for Property { required, field_name, .. } in &properties {
					writeln!(file, " +")?;
					if *required {
						write!(file, "            1")?;
					}
					else {
						write!(file, "            (if self.{}.is_some() {{ 1 }} else {{ 0 }})", field_name)?;
					}
				}
				writeln!(file, ",")?;
				writeln!(file, "        )?;")?;
				for Property { name, required, field_name, .. } in &properties {
					if *required {
						writeln!(file, r#"        ::serde::ser::SerializeStruct::serialize_field(&mut state, "{}", &self.{})?;"#, name, field_name)?;
					}
					else {
						writeln!(file, "        if let Some(value) = &self.{} {{", field_name)?;
						writeln!(file, r#"            ::serde::ser::SerializeStruct::serialize_field(&mut state, "{}", value)?;"#, name)?;
						writeln!(file, "        }}")?;
					}
				}
				writeln!(file, "        ::serde::ser::SerializeStruct::end(state)")?;
				writeln!(file, "    }}")?;
				writeln!(file, "}}")?;

				num_generated_structs += 1;
			},

			swagger20::SchemaKind::Ref(_) |
			swagger20::SchemaKind::Ty(_) => {
				// TODO: Should Ty be newtypes instead of aliases?
				writeln!(file, "pub type {} = {};", type_name, get_rust_type(&definition.kind, &replace_namespaces, mod_root))?;
				num_generated_type_aliases += 1;
			},
		}

		trace!("OK");
	}

	info!("OK");
	info!("Generated {} structs", num_generated_structs);
	info!("Generated {} type aliases", num_generated_type_aliases);
	info!("Skipped generating {} type aliases", num_skipped_refs);
	info!("");

	Ok(())
}

fn create_file_for_type(
	definition_path: &swagger20::DefinitionPath,
	out_dir: &std::path::Path,
	replace_namespaces: &[(Vec<&str>, Vec<String>)],
) -> Result<(std::io::BufWriter<std::fs::File>, String), Error> {
	use std::io::Write;

	let parts = replace_namespace(definition_path.split('.'), replace_namespaces);

	let mut current = out_dir.to_owned();

	for part in &parts[0..parts.len() - 1] {
		trace!("Current directory: {}", current.display());

		let mod_name = get_rust_ident(part);

		current.push(&*mod_name);

		trace!("Checking if subdirectory {} exists...", current.display());

		if !current.is_dir() {
			trace!("    Subdirectory does not exist. Creating mod.rs with a reference to it...");

			let current_mod_rs_path = current.with_file_name("mod.rs");
			let append_newline = current_mod_rs_path.exists();
			let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current_mod_rs_path)?);
			if append_newline {
				writeln!(parent_mod_rs)?;
			}
			writeln!(parent_mod_rs, "pub mod {};", mod_name)?;

			trace!("    OK");
			trace!("    Creating subdirectory...");

			std::fs::create_dir(&current)?;
			trace!("    OK");
		}

		trace!("OK");
	}

	let type_name = parts[parts.len() - 1].to_string();

	let mod_name = get_rust_ident(&type_name);
	{
		let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current.join("mod.rs"))?);
		writeln!(parent_mod_rs)?;
		writeln!(parent_mod_rs, "mod {};", mod_name)?;
		writeln!(parent_mod_rs, "pub use self::{}::{};", mod_name, type_name)?;
	}

	let file_name = current.join(&*mod_name).with_extension("rs");

	Ok((std::io::BufWriter::new(std::fs::File::create(file_name)?), type_name))
}

fn get_comment_text<'a>(s: &'a str) -> impl Iterator<Item = std::borrow::Cow<'static, str>> + 'a {
	s.lines().map(|line|
		if line.is_empty() {
			"///".into()
		}
		else {
			let line = line.replace("[", r"\[");
			let line = line.replace("]", r"\]");
			format!("/// {}", line).into()
		})
}

fn get_fully_qualified_type_name(ref_path: &swagger20::RefPath, replace_namespaces: &[(Vec<&str>, Vec<String>)], mod_root: &str) -> String {
	use std::fmt::Write;

	let mut result = format!("::{}", mod_root);

	let parts = replace_namespace(ref_path.split('.'), replace_namespaces);

	for part in &parts[..parts.len() - 1] {
		write!(result, "::{}", get_rust_ident(part)).unwrap();
	}

	write!(result, "::{}", parts[parts.len() - 1]).unwrap();

	result
}

fn get_rust_ident(name: &str) -> std::borrow::Cow<'static, str> {
	use std::fmt::Write;

	// Fix cases of invalid rust idents
	match name {
		"$ref" => return "ref_path".into(),
		"$schema" => return "schema".into(),
		"continue" => return "continue_".into(),
		"enum" => return "enum_".into(),
		"external_ip_s" => return "external_ips".into(),
		"type" => return "type_".into(),
		_ => (),
	}

	let chars: Vec<_> = name.chars().collect();

	let mut result = String::new();

	for (i, &c) in chars.iter().enumerate() {
		if c.is_uppercase() {
			let previous = if i == 0 { None } else { Some(chars[i - 1].is_uppercase()) };
			let next = chars.get(i + 1).map(|c| c.is_uppercase());

			match (previous, next) {
				(Some(false), _) |
				(Some(true), Some(false)) => result.push('_'),
				_ => (),
			}

			write!(result, "{}", c.to_lowercase()).unwrap();
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

fn get_rust_type(schema_kind: &swagger20::SchemaKind, replace_namespaces: &[(Vec<&str>, Vec<String>)], mod_root: &str) -> std::borrow::Cow<'static, str> {
	match *schema_kind {
		swagger20::SchemaKind::Properties(_) => panic!("Nested anonymous types not supported"),

		swagger20::SchemaKind::Ref(ref ref_path) => get_fully_qualified_type_name(ref_path, replace_namespaces, mod_root).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { ref items }) => format!("Vec<{}>", get_rust_type(&items.kind, replace_namespaces, mod_root)).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => "bool".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => "i32".into(),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => "i64".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => "f64".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { ref additional_properties }) =>
			format!("::std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, mod_root)).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => "::ByteString".into(),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => "::chrono::DateTime<::chrono::Utc>".into(),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::IntOrString) }) => "::IntOrString".into(),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => "String".into(),
	}
}

fn replace_namespace<'a, I>(parts: I, replace_namespaces: &[(Vec<&str>, Vec<String>)]) -> Vec<String> where I: IntoIterator<Item = &'a str> {
	let parts: Vec<_> = parts.into_iter().collect();

	trace!("parts = {:?}, replace_namespaces = {:?}", parts, replace_namespaces);

	for &(ref from, ref to) in replace_namespaces {
		if parts.starts_with(from) {
			let mut result = to.clone();
			result.extend(parts[from.len()..].into_iter().map(ToString::to_string));
			return result;
		}
	}

	parts.into_iter().map(ToString::to_string).collect()
}
