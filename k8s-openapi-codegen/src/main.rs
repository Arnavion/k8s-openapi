#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::let_unit_value,
    clippy::too_many_lines,
)]

mod fixups;
mod logger;
mod supported_version;

use futures_util::TryStreamExt;
use k8s_openapi_codegen_common::swagger20;

struct Error(Box<dyn std::error::Error + Send + Sync>);

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)?;

        let mut source = self.0.source();
        while let Some(err) = source {
            writeln!(f, "caused by: {err}")?;
            source = err.source();
        }

        Ok(())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

macro_rules! impl_from_for_error {
    ($($ty:ty ,)*) => {
        $(
            impl From<$ty> for Error {
                fn from(err: $ty) -> Self {
                    Error(err.into())
                }
            }
        )*
    };
}

impl_from_for_error! {
    &'_ str,
    String,
    std::fmt::Error,
    std::io::Error,
    k8s_openapi_codegen_common::Error,
    log::SetLoggerError,
    reqwest::Error,
    serde_json::Error,
    tokio::task::JoinError,
    url::ParseError,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let Options { versions: requested_versions } = clap::Parser::parse();

    {
        let logger = logger::Logger;
        log::set_boxed_logger(Box::new(logger))?;

        let mut builder = env_logger::Builder::new();
        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        builder.parse_filters(&rust_log);
        let logger = builder.build();
        log::set_max_level(logger.filter());
    }

    let client = std::sync::Arc::new(reqwest::Client::new());

    let out_dir_base: &std::path::Path = env!("CARGO_MANIFEST_DIR").as_ref();
    let out_dir_base = std::sync::Arc::new(out_dir_base.parent().ok_or("path does not have a parent")?.join("src"));

    let requested_versions =
        if requested_versions.is_empty() {
            supported_version::ALL.iter().map(|&version| RequestedVersion { version, overriden_spec_url: None }).collect()
        }
        else {
            requested_versions
        };

    let tasks: futures_util::stream::FuturesUnordered<_> =
        requested_versions.into_iter()
        .map(|RequestedVersion { version, overriden_spec_url }| {
            let out_dir_base = out_dir_base.clone();
            let client = client.clone();

            async move {
                let task_local_logger = logger::make_local_logger(version.name());
                let spec_url = overriden_spec_url.as_deref().unwrap_or_else(|| version.spec_url());
                logger::TASK_LOCAL_LOGGER.scope(task_local_logger, async {
                    match run(version, spec_url, &out_dir_base, &client).await {
                        Ok(()) => Ok(()),
                        Err(err) => {
                            log::error!("Error: {err}");
                            Err(err)
                        },
                    }
                }).await?;
                Ok::<_, Error>(())
            }
        })
        .collect();
    tasks.try_for_each(|()| std::future::ready(Ok(()))).await?;

    Ok(())
}

#[derive(clap::Parser)]
struct Options {
    /// This parameter specifies the versions of Kubernetes that the API bindings should be generated for.
    ///
    /// `--generate=1.27` means "generate bindings for Kubernetes v1.27,
    /// using that version's OpenAPI spec from the https://github.com/kubernetes/kubernetes repository".
    ///
    /// `--generate=1.27:https://example.org/swagger.json` means "generate bindings for v1.27
    /// using the OpenAPI spec at the URL https://example.org/swagger.json".
    ///
    /// `--generate=1.27:file:///path/to/swagger.json` means "generate binding for v1.27,
    /// using the OpenAPI spec in the file /path/to/swagger.json".
    ///
    /// This parameter can be specified multiple times to generate bindings for multiple versions.
    ///
    /// If this parameter isn't specified, bindings will be generated for all supported versions,
    /// using their respective OpenAPI specs from the https://github.com/kubernetes/kubernetes repository.
    #[clap(long = "generate", value_name = "VERSION")]
    versions: Vec<RequestedVersion>,
}

#[derive(Clone)]
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
                let mut err = format!("unknown version {version:?} requested, supported versions are ");
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

async fn run(
    supported_version: supported_version::SupportedVersion,
    spec_url: &str,
    out_dir_base: &std::path::Path,
    client: &reqwest::Client,
) -> Result<(), Error> {
    let mod_root = supported_version.mod_root();

    let out_dir = out_dir_base.join(mod_root);

    let mut num_generated_structs = 0_usize;
    let mut num_generated_type_aliases = 0_usize;

    let mut spec: swagger20::Spec = {
        log::info!("Parsing spec file at {spec_url} ...");
        let spec_url: url::Url = spec_url.parse()?;
        if spec_url.scheme() == "file" {
            let spec_path = spec_url.to_file_path().map_err(|()| "not a file path")?;
            let spec_file = std::fs::File::open(spec_path)?;
            let spec_file = std::io::BufReader::new(spec_file);
            serde::Deserialize::deserialize(&mut serde_json::Deserializer::from_reader(spec_file))?
        }
        else {
            let response = client.get(spec_url).send().await?;
            let status = response.status();
            if status != http::StatusCode::OK {
                return Err(status.to_string().into());
            }
            response.json().await?
        }
    };

    let () = tokio::task::spawn_blocking(move || -> Result<(), Error> {
        {
            let thread_local_logger = logger::make_local_logger(supported_version.name());
            logger::register_thread_local_logger(thread_local_logger);
        }

        log::info!("Applying fixups...");
        supported_version.fixup(&mut spec)?;

        let expected_num_generated_types: usize = spec.definitions.len();

        log::info!("OK. Spec has {expected_num_generated_types} definitions.");

        loop {
            log::info!("Removing output directory {} ...", out_dir.display());
            match std::fs::remove_dir_all(&out_dir) {
                Ok(()) => log::trace!("OK"),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    log::trace!("OK. Directory doesn't exist");

                    log::info!("Creating output directory {} ...", out_dir.display());
                    match std::fs::create_dir(&out_dir) {
                        Ok(()) => {
                            log::trace!("OK");
                            break;
                        },
                        Err(err) => log::warn!("Error: {err}"),
                    }
                },
                Err(err) => log::warn!("Error: {err}"),
            }
        }

        log::info!("Generating types...");

        for definition_path in spec.definitions.keys() {
            log::trace!("Working on {definition_path} ...");

            let run_state = RunState {
                out_dir: &out_dir,
                parent_mod_rs_file_and_mod_name: None,
            };

            let run_result = k8s_openapi_codegen_common::run(
                &spec.definitions,
                &mut spec.operations,
                definition_path,
                &MapNamespace,
                "pub ",
                k8s_openapi_codegen_common::GenerateSchema::Yes { feature: Some("schemars") },
                run_state,
            )?;

            num_generated_structs += run_result.num_generated_structs;
            num_generated_type_aliases += run_result.num_generated_type_aliases;
        }

        log::info!("OK");
        log::info!("Generated {num_generated_structs} structs");
        log::info!("Generated {num_generated_type_aliases} type aliases");

        if num_generated_structs + num_generated_type_aliases != expected_num_generated_types {
            return Err("Did not generate or skip expected number of types".into());
        }

        log::info!("");

        Ok(())
    }).await??;

    Ok(())
}

struct MapNamespace;

impl k8s_openapi_codegen_common::MapNamespace for MapNamespace {
    fn map_namespace<'a>(&self, path_parts: &[&'a str]) -> Option<Vec<&'a str>> {
        match path_parts {
            ["io", "k8s", rest @ ..] => Some(std::iter::once("crate").chain(rest.iter().copied()).collect()),
            _ => None,
        }
    }
}

struct RunState<'a> {
    out_dir: &'a std::path::Path,
    parent_mod_rs_file_and_mod_name: Option<(<Self as k8s_openapi_codegen_common::RunState>::Writer, std::borrow::Cow<'static, str>)>,
}

impl k8s_openapi_codegen_common::RunState for RunState<'_> {
    type Writer = std::io::BufWriter<std::fs::File>;

    fn make_writer(&mut self, parts: &[&str]) -> std::io::Result<Self::Writer> {
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
                writeln!(parent_mod_rs, "pub mod {mod_name};")?;

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
        writeln!(parent_mod_rs, "mod {mod_name};")?;
        writeln!(parent_mod_rs, "pub use self::{mod_name}::{type_name};")?;

        let file_name = current.join(&*mod_name).with_extension("rs");
        let file = std::io::BufWriter::new(std::fs::File::create(file_name)?);

        self.parent_mod_rs_file_and_mod_name = Some((parent_mod_rs, mod_name));

        Ok(file)
    }

    fn finish(&mut self, _writer: Self::Writer) { }
}
