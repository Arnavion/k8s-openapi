#![feature(catch_expr, conservative_impl_trait, proc_macro)]

#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(
	cyclomatic_complexity,
	missing_docs_in_private_items,
	shadow_reuse,
	shadow_same,
	unseparated_literal_suffix,
	use_debug,
))]

extern crate backtrace;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod swagger20;

struct Error(Box<std::error::Error>, backtrace::Backtrace);

impl<E> From<E> for Error where Box<std::error::Error>: From<E> {
	fn from(value: E) -> Self {
		Error(value.into(), backtrace::Backtrace::new())
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "error: {}", self.0)?;
		#[cfg_attr(feature = "cargo-clippy", allow(use_debug))]
		write!(f, "{:?}", self.1)?;
		Ok(())
	}
}

type Result<T> = std::result::Result<T, Error>;

fn main() {
	{
		use std::io::Write;

		let mut builder = env_logger::Builder::new();
		builder.format(|buf, record| writeln!(buf, "{} {}:{} {}", record.level(), record.file().unwrap_or("?"), record.line().unwrap_or(0), record.args()));
		let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
		builder.parse(&rust_log);
		builder.init();
	}

	let result: Result<()> = do catch {
		use std::io::Write;

		let mut args = std::env::args_os().skip(1);
		let input: std::path::PathBuf = args.next().ok_or("expected input file parameter")?.into();
		let out_dir: std::path::PathBuf = args.next().ok_or("expected output directory parameter")?.into();

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

		info!(target: "", "Parsing spec file at {} ...", input.display());
		let spec: swagger20::Spec = {
			let file = std::io::BufReader::new(std::fs::File::open(input)?);
			serde_json::from_reader(file)?
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

						let field_type_name = get_rust_type(&property.kind, &replace_namespaces);

						// Fix cases of infinite recursion
						let field_type_name = if let swagger20::SchemaKind::Ref(ref ref_path) = property.kind {
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
								) => format!("Box<{}>", field_type_name).into(),

								_ => field_type_name,
							}
						}
						else {
							field_type_name
						};

						write!(file, "{}", field_type_name)?;

						if !required {
							write!(file, ">")?;
						}

						writeln!(file, ",")?;
					}
					writeln!(file, "}}")?;

					num_generated_structs += 1;
				},

				swagger20::SchemaKind::Ref(_) |
				swagger20::SchemaKind::Ty(_) => {
					writeln!(file, "pub type {} = {};", type_name, get_rust_type(&definition.kind, &replace_namespaces))?;
					num_generated_type_aliases += 1;
				},
			}

			trace!("OK");
		}

		info!("OK");
		info!("Generated {} structs", num_generated_structs);
		info!("Generated {} type aliases", num_generated_type_aliases);
		info!("Skipped generating {} type aliases", num_skipped_refs);

		{
			info!("Fixing crate root");

			let mut old_crate_root = out_dir.clone();
			old_crate_root.push("mod.rs");

			let mut old_crate_root_contents = vec![];
			std::io::Read::read_to_end(&mut std::fs::File::open(&old_crate_root)?, &mut old_crate_root_contents)?;
			std::fs::remove_file(old_crate_root)?;

			let mut crate_root = out_dir.clone();
			crate_root.push("lib.rs");

			let mut file = std::io::BufWriter::new(std::fs::File::create(crate_root)?);
			writeln!(file, "#[macro_use] extern crate serde_derive;")?;
			writeln!(file)?;
			file.write_all(&old_crate_root_contents)?;

			info!("OK");
		}

		Ok(())
	};

	#[cfg_attr(feature = "cargo-clippy", allow(print_stdout))]
	{
		if let Err(err) = result {
			println!("{}", err);
			std::process::exit(1);
		}
	}
}

fn create_file_for_type(
	definition_path: &swagger20::DefinitionPath,
	out_dir: &std::path::Path,
	replace_namespaces: &[(Vec<&str>, Vec<String>)],
) -> Result<(std::io::BufWriter<std::fs::File>, String)> {
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
			format!("/// {}", line).into()
		})
}

fn get_fully_qualified_type_name(ref_path: &swagger20::RefPath, replace_namespaces: &[(Vec<&str>, Vec<String>)]) -> String {
	use std::fmt::Write;

	let mut result = String::new();

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

fn get_rust_type(schema_kind: &swagger20::SchemaKind, replace_namespaces: &[(Vec<&str>, Vec<String>)]) -> std::borrow::Cow<'static, str> {
	#[cfg_attr(feature = "cargo-clippy", allow(unneeded_field_pattern))]
	match *schema_kind {
		swagger20::SchemaKind::Properties(_) => panic!("Nested anonymous types not supported"),

		swagger20::SchemaKind::Ref(ref ref_path) => get_fully_qualified_type_name(ref_path, replace_namespaces).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { ref items }) => format!("Vec<{}>", get_rust_type(&items.kind, replace_namespaces)).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => "bool".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => "i32".into(),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => "i64".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => "f64".into(),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { ref additional_properties }) => format!("::std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces)).into(),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: _ }) => "String".into(),
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
