#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::let_underscore_drop,
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

use k8s_openapi_codegen_common::swagger20;

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

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

fn main() -> Result<(), Error> {
	let Options { 
		versions: requested_versions,
		skip_client: skip_client_generation,
	} = structopt::StructOpt::from_args();

	{
		let logger = logger::Logger;
		log::set_boxed_logger(Box::new(logger))?;

		let mut builder = env_logger::Builder::new();
		let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
		builder.parse_filters(&rust_log);
		let logger = builder.build();
		log::set_max_level(logger.filter());
	}

	let client =
		std::sync::Arc::new(
			reqwest::blocking::ClientBuilder::new()
			.timeout(None)
			.build().expect("could not create reqwest client"));

	let out_dir_base: &std::path::Path = env!("CARGO_MANIFEST_DIR").as_ref();
	let out_dir_base = std::sync::Arc::new(out_dir_base.parent().ok_or("path does not have a parent")?.join("src"));

	let requested_versions =
		if requested_versions.is_empty() {
			supported_version::ALL.iter().map(|&version| RequestedVersion { version, overriden_spec_url: None }).collect()
		}
		else {
			requested_versions
		};

	let threads: Vec<_> =
		requested_versions.into_iter()
		.map(|RequestedVersion { version, overriden_spec_url }| {
			let version_name = version.name();

			std::thread::Builder::new().name(version_name.to_owned()).spawn({
				let out_dir_base = out_dir_base.clone();
				let client = client.clone();

				move || -> Result<(), Error> {
					{
						let mut builder = env_logger::Builder::new();
						builder.format(move |buf, record| {
							use std::io::Write;
							writeln!(buf, "[{}] {} {}:{} {}", version_name, record.level(), record.file().unwrap_or("?"), record.line().unwrap_or(0), record.args())
						});
						let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
						builder.parse_filters(&rust_log);
						logger::register_thread_local_logger(builder.build());
					}

					let spec_url = overriden_spec_url.as_deref().unwrap_or_else(|| version.spec_url());

					run(version, spec_url, overriden_spec_url.is_some(), skip_client_generation, &out_dir_base, &client)?;

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

#[derive(structopt::StructOpt)]
struct Options {
	/// This parameter specifies the versions of Kubernetes that the API bindings should be generated for.
	///
	/// `--generate=1.20` means "generate bindings for Kubernetes v1.20,
	/// based on that version's OpenAPI spec from the https://github.com/kubernetes/kubernetes repository".
	///
	/// `--generate=1.20:https://example.org/spec.json` means "generate binding for v1.20,
	/// based on the OpenAPI spec at https://example.org/spec.json".
	///
	/// This parameter can be specified multiple times to specify multiple versions.
	///
	/// If this parameter isn't specified, the API bindings will be generated for all supported versions,
	/// based on their OpenAPI specs from the https://github.com/kubernetes/kubernetes repository.
	#[structopt(long = "generate", value_name = "VERSION")]
	versions: Vec<RequestedVersion>,
	/// This flag allows completely skip client functions generation.
	/// It can be used for following reasons:
	/// - You use a custom spec, which contains non-builtin resources, such as
	///   CRD, and k8s-openapi-codegen fails on these resources without this flag.
	/// - You want to reduce generated code amount
	///
	/// Note however that `k8s-openapi` is generated without this flag.
	#[structopt(long = "skip-client")]
	skip_client: bool,
}

struct RequestedVersion {
	version: supported_version::SupportedVersion,
	overriden_spec_url: Option<String>,
}

impl std::str::FromStr for RequestedVersion {
	type Err = Error;

	fn from_str(spec: &str) -> Result<Self, Error> {
		let (version, overriden_spec_url) =
			spec.split_once(':')
			.map_or(
				(spec, None),
				|(version, url)| (version, Some(url.to_owned())),
			);

		let &version =
			supported_version::ALL.iter()
			.find(|v| v.name() == version)
			.ok_or_else(|| {
				let mut err = format!("unknown version {:?} requested, supported versions are ", version);
				for (i, &version) in supported_version::ALL.iter().enumerate() {
					if i > 0 {
						err.push_str(", ");
					}

					err.push('"');
					err.push_str(version.name());
					err.push('"');
				}
				err
			})?;

		Ok(RequestedVersion {
			version,
			overriden_spec_url,
		})
	}
}

fn run(
	supported_version: supported_version::SupportedVersion,
	spec_url: &str,
	spec_is_custom: bool,
	skip_client_generation: bool,
	out_dir_base: &std::path::Path,
	client: &reqwest::blocking::Client,
) -> Result<(), Error> {
	let mod_root = supported_version.mod_root();

	let out_dir = out_dir_base.join(mod_root);

	let mut num_generated_structs = 0usize;
	let mut num_generated_type_aliases = 0usize;
	let mut num_generated_apis = 0usize;

	let mut spec: swagger20::Spec = {
		log::info!("Parsing spec file at {} ...", spec_url);
		let response = client.get(spec_url).send()?;
		let status = response.status();
		if status != http::StatusCode::OK {
			return Err(status.to_string().into());
		}
		response.json()?
	};

	log::info!("Applying fixups...");
	supported_version.fixup(&mut spec, skip_client_generation)?;
	

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

		let run_state = RunState {
			out_dir: &out_dir,
			parent_mod_rs_file_and_mod_name: None,
		};

		let run_result = k8s_openapi_codegen_common::run(
			&spec.definitions,
			&mut spec.operations,
			skip_client_generation,
			definition_path,
			&MapNamespace {
				custom_spec: spec_is_custom
			},
			"pub ",
			Some("api"),
			run_state,
		)?;

		num_generated_structs += run_result.num_generated_structs;
		num_generated_type_aliases += run_result.num_generated_type_aliases;
		num_generated_apis += run_result.num_generated_apis;
	}

	// Top-level operations
	if !skip_client_generation {
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
				&MapNamespace {
					custom_spec: spec_is_custom
				},
				"pub ",
				None,
				Some("api"),
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

	if !skip_client_generation && num_generated_apis != expected_num_generated_apis {
		return Err("Did not generate expected number of API functions".into());
	}

	log::info!("");

	Ok(())
}

struct MapNamespace {
	custom_spec: bool
}

impl k8s_openapi_codegen_common::MapNamespace for MapNamespace {
	fn map_namespace<'a>(&self, path_parts: &[&'a str]) -> Option<Vec<&'a str>> {
		if let ["io", "k8s", rest @ ..] = path_parts {
			return Some(std::iter::once("crate").chain(rest.iter().copied()).collect())
		}
		if self.custom_spec {
			// custom spec can contain types from other namespaces
			Some(["crate", "special"].iter().copied().chain(path_parts.iter().copied()).collect())
		} else {
		None
		}
	}
}

struct RunState<'a> {
	out_dir: &'a std::path::Path,
	parent_mod_rs_file_and_mod_name: Option<(<Self as k8s_openapi_codegen_common::RunState>::Writer, std::borrow::Cow<'static, str>)>,
}

impl k8s_openapi_codegen_common::RunState for RunState<'_> {
	type Writer = std::io::BufWriter<std::fs::File>;

	fn make_writer(
		&mut self,
		parts: &[&str],
		type_feature: Option<&str>,
	) -> std::io::Result<Self::Writer> {
		use std::io::Write;

		let mut current = self.out_dir.to_owned();

		for part in parts.iter().skip(1).rev().skip(1).rev() {
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

		let type_name = parts.last().unwrap();

		let mod_name = k8s_openapi_codegen_common::get_rust_ident(type_name);

		let mut parent_mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open(current.join("mod.rs"))?);
		writeln!(parent_mod_rs)?;
		if let Some(type_feature) = type_feature {
			writeln!(parent_mod_rs, r#"#[cfg(feature = "{}")]"#, type_feature)?;
		}
		writeln!(parent_mod_rs, "mod {};", mod_name)?;
		if let Some(type_feature) = type_feature {
			writeln!(parent_mod_rs, r#"#[cfg(feature = "{}")]"#, type_feature)?;
		}
		writeln!(parent_mod_rs, "pub use self::{}::{};", mod_name, type_name)?;

		let file_name = current.join(&*mod_name).with_extension("rs");
		let file = std::io::BufWriter::new(std::fs::File::create(file_name)?);

		self.parent_mod_rs_file_and_mod_name = Some((parent_mod_rs, mod_name));

		Ok(file)
	}

	fn handle_operation_types(
		&mut self,
		operation_optional_parameters_name: Option<&str>,
		operation_result_name: Option<&str>,
	) -> std::io::Result<()> {
		use std::io::Write;

		let (parent_mod_rs, mod_name) = self.parent_mod_rs_file_and_mod_name.as_mut().unwrap();
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
	}

	fn finish(&mut self, _writer: Self::Writer) { }
}
