#![feature(catch_expr, conservative_impl_trait, proc_macro)]

#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(
	cyclomatic_complexity,
	missing_docs_in_private_items,
))]

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod swagger20;

fn main() {
	let mut args = std::env::args_os().skip(1);
	let input: std::path::PathBuf = args.next().expect("expected input file parameter").into();
	let out_dir: std::path::PathBuf = args.next().expect("expected output directory parameter").into();

	let result: Result<(), Box<std::error::Error>> = do catch {
		use std::io::Write;

		let spec: swagger20::Spec = {
			let file = std::io::BufReader::new(std::fs::File::open(input)?);
			serde_json::from_reader(file)?
		};

		std::fs::remove_dir_all(&out_dir).or_else(|err|
			if err.kind() == std::io::ErrorKind::NotFound {
				Ok(())
			}
			else {
				Err(err)
			})?;

		std::fs::create_dir(&out_dir)?;

		for (definition_path, definition) in spec.definitions {
			let (mut file, type_name) = create_file_for_type(&definition_path, &out_dir)?;

			writeln!(file, "// Generated from definition {}", definition_path)?;
			writeln!(file)?;

			if let Some(description) = definition.description {
				for line in get_comment_text(&description) {
					writeln!(file, "{}", line)?;
				}
			}

			match definition.kind {
				swagger20::SchemaKind::Properties(properties) => {
					writeln!(file, "#[derive(Debug, Default, Deserialize, Serialize)]")?;
					writeln!(file, "pub struct {} {{", type_name)?;
					for (i, (name, (property, required))) in properties.into_iter().enumerate() {
						if i > 0 {
							writeln!(file)?;
						}

						if let Some(description) = property.description {
							for line in get_comment_text(&description) {
								writeln!(file, "    {}", line)?;
							}
						}

						let field_name = get_rust_ident(&name);
						if field_name != *name {
							writeln!(file, r#"    #[serde(rename = "{}")]"#, name)?;
						}

						if !required {
							writeln!(file, r#"    #[serde(skip_serializing_if = "Option::is_none")]"#)?;
						}

						write!(file, "    pub {}: ", field_name)?;

						if !required {
							write!(file, "Option<")?;
						}

						let field_type_name = get_rust_type(&property.kind);

						// Fix cases of infinite recursion
						let field_type_name = match (&*definition_path, &*field_name, &*field_type_name) {
							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrArray",
								"schema",
								"::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps",
							) |

							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrBool",
								"schema",
								"::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps",
							) |

							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
								"not",
								"::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps",
							) => format!("Box<{}>", field_type_name).into(),

							_ => field_type_name,
						};

						write!(file, "{}", field_type_name)?;

						if !required {
							write!(file, ">")?;
						}

						writeln!(file, ",")?;
					}
					writeln!(file, "}}")?;
				},

				swagger20::SchemaKind::Ref(_) |
				swagger20::SchemaKind::Ty(_) => {
					writeln!(file, "pub type {} = {};", type_name, get_rust_type(&definition.kind))?;
				},
			}
		}

		Ok(())
	};

	#[cfg_attr(feature = "cargo-clippy", allow(result_unwrap_used))]
	result.unwrap();
}

fn create_file_for_type(definition_path: &swagger20::DefinitionPath, out_dir: &std::path::Path) -> std::io::Result<(std::io::BufWriter<std::fs::File>, String)> {
	use std::io::Write;

	let parts: Vec<_> = definition_path.split('.').collect();

	let mut current = out_dir.to_owned();
	for part in &parts[0..parts.len() - 1] {
		let mod_name = get_rust_ident(part);

		current.push(&*mod_name);

		if !current.is_dir() {
			let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current.with_file_name("mod.rs"))?);
			writeln!(parent_mod_rs)?;
			writeln!(parent_mod_rs, "pub mod {};", mod_name)?;

			std::fs::create_dir(&current)?;
		}
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
			format!("/// {}", line).into()
		})
}

fn get_fully_qualified_type_name(ref_path: &swagger20::RefPath) -> String {
	use std::fmt::Write;

	let mut result = String::new();

	let parts: Vec<_> = ref_path.split('.').collect();

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

fn get_rust_type(schema_kind: &swagger20::SchemaKind) -> std::borrow::Cow<'static, str> {
	#[cfg_attr(feature = "cargo-clippy", allow(unneeded_field_pattern))]
	match *schema_kind {
		swagger20::SchemaKind::Properties(_) => panic!("Nested anonymous types not supported"),

		swagger20::SchemaKind::Ref(ref ref_path) => get_fully_qualified_type_name(ref_path).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { ref items }) => format!("Vec<{}>", get_rust_type(&items.kind)).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => "bool".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => "i32".into(),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => "i64".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => "f64".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { ref additional_properties }) => format!("::std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind)).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: _ }) => "String".into(),
	}
}
