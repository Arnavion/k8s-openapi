#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::must_use_candidate,
	clippy::similar_names,
	clippy::too_many_arguments,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unseparated_literal_suffix,
	clippy::use_self,
)]

mod fixups;
mod logger;
mod supported_version;

use k8s_openapi_codegen_common::{swagger20, CrateRooter};

struct Error(Box<dyn std::error::Error + Send + Sync>, backtrace::Backtrace);

impl<E> From<E> for Error where E: Into<Box<dyn std::error::Error + Send + Sync>> {
	fn from(value: E) -> Self {
		Error(value.into(), backtrace::Backtrace::new())
	}
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.0)?;

		let mut source = self.0.source();
		while let Some(err) = source {
			writeln!(f, "caused by: {}", err)?;
			source = err.source();
		}

		write!(f, "{:?}", self.1)?;
		Ok(())
	}
}

fn main() -> Result<(), Error> {
	{
		let logger = logger::Logger;
		log::set_boxed_logger(Box::new(logger))?;

		let mut builder = env_logger::Builder::new();
		let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
		builder.parse_filters(&rust_log);
		let logger = builder.build();
		log::set_max_level(logger.filter());
	}

	let client = std::sync::Arc::new(reqwest::blocking::Client::new());

	let out_dir_base: &std::path::Path = env!("CARGO_MANIFEST_DIR").as_ref();
	let out_dir_base = std::sync::Arc::new(out_dir_base.parent().ok_or("path does not have a parent")?.join("src"));

	let threads: Vec<_> =
		supported_version::ALL.iter()
		.map(|&supported_version| {
			let mod_root = supported_version.mod_root().to_owned();

			std::thread::Builder::new().name(mod_root.clone()).spawn({
				let out_dir_base = out_dir_base.clone();
				let client = client.clone();

				move || -> Result<(), Error> {
					{
						let mut builder = env_logger::Builder::new();
						builder.format(move |buf, record| {
							use std::io::Write;
							writeln!(buf, "[{}] {} {}:{} {}", mod_root, record.level(), record.file().unwrap_or("?"), record.line().unwrap_or(0), record.args())
						});
						let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
						builder.parse_filters(&rust_log);
						logger::register_thread_local_logger(builder.build());
					}

					run(supported_version, &out_dir_base, &client)?;

					Ok(())
				}
			}).unwrap()
		})
		.collect();

	let mut result = Ok(());
	for thread in threads {
		let thread_name = thread.thread().name().unwrap().to_owned();
		if let Err(err) = thread.join().unwrap() {
			eprintln!("[{}] {:?}", thread_name, err);
			result = Err("one or more runs failed".into());
		}
	}

	result
}

struct DefaultRooter {
}

impl CrateRooter for DefaultRooter {
	/// In our case, the crate root will always be `crate`, as we are not referencing
	/// types from other crates.
	fn root(&self, _: &Vec<&str>) -> String {
		"crate".into()
	}
}

fn run(supported_version: supported_version::SupportedVersion, out_dir_base: &std::path::Path, client: &reqwest::blocking::Client) -> Result<(), Error> {
	use std::io::Write;

	let crate_root = DefaultRooter{};
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
		log::info!("Parsing spec file at {} ...", spec_url);
		let response = client.get(spec_url).send()?;
		let status = response.status();
		if status != reqwest::StatusCode::OK {
			return Err(status.to_string().into());
		}
		response.json()?
	};

	log::info!("Applying fixups...");
	supported_version.fixup(&mut spec)?;

	let expected_num_generated_types: usize = spec.definitions.len();
	let expected_num_generated_apis: usize = spec.operations.len();

	log::info!("OK. Spec has {} definitions and {} operations", expected_num_generated_types, expected_num_generated_apis);

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

	for definition_path in spec.definitions.keys() {
		log::trace!("Working on {} ...", definition_path);

		let parent_mod_rs_file_and_mod_name = std::cell::RefCell::new(None);

		let run_result = k8s_openapi_codegen_common::run(
			&spec.definitions,
			&mut spec.operations,
			definition_path,
			swagger20::RefPathRelativeTo::Crate,
			replace_namespaces,
			&crate_root,
			"pub ",
			true,
			|parts, is_under_api_feature| {
				let mut current = out_dir.to_owned();

				for part in parts.iter().rev().skip(1).rev() {
					log::trace!("Current directory: {}", current.display());

					let mod_name = k8s_openapi_codegen_common::get_rust_ident(part);

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

				let type_name = parts.last().unwrap().to_string();

				let mod_name = k8s_openapi_codegen_common::get_rust_ident(&type_name);

				let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current.join("mod.rs"))?);
				writeln!(parent_mod_rs)?;
				if is_under_api_feature {
					writeln!(parent_mod_rs, r#"#[cfg(feature = "api")]"#)?;
				}
				writeln!(parent_mod_rs, "mod {};", mod_name)?;
				if is_under_api_feature {
					writeln!(parent_mod_rs, r#"#[cfg(feature = "api")]"#)?;
				}
				writeln!(parent_mod_rs, "pub use self::{}::{};", mod_name, type_name)?;

				let file_name = current.join(&*mod_name).with_extension("rs");
				let file = std::io::BufWriter::new(std::fs::File::create(file_name)?);

				parent_mod_rs_file_and_mod_name.replace(Some((parent_mod_rs, mod_name)));

				Ok(file)
			},
			|operation_optional_parameters_name, operation_result_name| {
				let mut parent_mod_rs_file_and_mod_name = parent_mod_rs_file_and_mod_name.borrow_mut();
				let (parent_mod_rs, mod_name) = parent_mod_rs_file_and_mod_name.as_mut().unwrap();
				match (operation_optional_parameters_name, operation_result_name) {
					(Some(operation_optional_parameters_name), Some(operation_result_name)) =>
						writeln!(
							parent_mod_rs,
							r#"#[cfg(feature = "api")] pub use self::{}::{{{}, {}}};"#,
							mod_name, operation_optional_parameters_name, operation_result_name)?,
					(Some(operation_optional_parameters_name), None) =>
						writeln!(
							parent_mod_rs,
							r#"#[cfg(feature = "api")] pub use self::{}::{};"#,
							mod_name, operation_optional_parameters_name)?,
					(None, Some(operation_result_name)) =>
						writeln!(
							parent_mod_rs,
							r#"#[cfg(feature = "api")] pub use self::{}::{};"#,
							mod_name, operation_result_name)?,
					(None, None) =>
						(),
				}
				Ok(())
			},
		)?;

		num_generated_structs += run_result.num_generated_structs;
		num_generated_type_aliases += run_result.num_generated_type_aliases;
		num_generated_apis += run_result.num_generated_apis;
	}

	// Top-level operations
	{
		let mut mod_root_file = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).open(out_dir.join("mod.rs"))?);

		spec.operations.sort_by(|o1, o2| o1.id.cmp(&o2.id));
		for operation in spec.operations {
			if let Some(swagger20::KubernetesGroupKindVersion { group, kind, version }) = operation.kubernetes_group_kind_version {
				return Err(format!(
					"Operation {} is associated with {}/{}/{} but did not get emitted with that definition",
					operation.id, group, version, kind).into());
			}

			k8s_openapi_codegen_common::write_operation(
				&mut mod_root_file,
				&operation,
				replace_namespaces,
				&crate_root,
				"pub ",
				&mut None,
				true,
			)?;

			num_generated_apis += 1;
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
