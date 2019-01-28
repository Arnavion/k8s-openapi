const MIN: usize = 8;
const MAX: usize = 13;

fn main() -> Result<(), Box<std::error::Error>> {
    use std::io::Write;

    let version =
        if std::env::var("CARGO_FEATURE_DOX").is_ok() {
            None
        }
        else {
            let mut v1 = None;

            for v2 in MIN..=MAX {
                if std::env::var(format!("CARGO_FEATURE_V1_{}", v2)).is_ok() {
                    v1 = match v1 {
                        Some(v1) => panic!(
                            "Both v1_{} and v1_{} features are enabled on the k8s-openapi crate. Only one feature can be enabled at the same time.",
                            v1,
                            v2,
                        ),
                        None => Some(v2),
                    };
                }
            }

            Some(v1.expect("At least one v1_* feature must be enabled on the k8s-openapi crate."))
        };

    let mut f = {
        let mut out_file: std::path::PathBuf = std::env::var_os("OUT_DIR").ok_or_else(|| "OUT_DIR not set")?.into();
        out_file.push("conditional_compilation_macros.rs");
        std::io::BufWriter::new(std::fs::File::create(out_file)?)
    };

    for v in MIN..=MAX {
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "///")?;
        writeln!(f, "/// # Examples")?;
        writeln!(f, "///")?;
        writeln!(f, "/// ```rust")?;
        writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
        writeln!(f, "/// k8s_if_1_{}! {{", v)?;
        if v == 7 {
            writeln!(f, "///     use k8s_openapi::kubernetes::pkg::api::v1 as api;")?;
        }
        else {
            writeln!(f, "///     use k8s_openapi::api::core::v1 as api;")?;
        }
        writeln!(f, "/// }}")?;
        writeln!(f, "/// ```")?;
        if version == Some(v) {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or higher feature is enabled, otherwise it evaluates to nothing.", v)?;
        if cfg_ge(v, version) {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or lower feature is enabled, otherwise it evaluates to nothing.", v)?;
        if cfg_le(v, version) {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        }
        writeln!(f)?;
    }

    writeln!(f, "/// A macro that emits a `match` expr with the given test expression and arms.")?;
    writeln!(f, "/// The match arms can be annotated with the other conditional compilation macros in this crate so that they're only emitted")?;
    writeln!(f, "/// if the predicate is true.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// # Examples")?;
    writeln!(f, "///")?;
    writeln!(f, "/// The `CustomResourceDefinition::create_custom_resource_definition` function returns an `HTTP 201 CREATED`")?;
    writeln!(f, "/// when it succeeds, but the codegen for v1.8 does not have a `Created` variant in the response type. So extracting the successful result from")?;
    writeln!(f, "/// the response requires matching `CreateCustomResourceDefinitionResponse::Other` for v1.8")?;
    writeln!(f, "/// and `CreateCustomResourceDefinitionResponse::Created` for v1.9 and above.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// Since `CreateCustomResourceDefinitionResponse::Created` does not exist in v1.8,")?;
    writeln!(f, "/// and `CreateCustomResourceDefinitionResponse::Other` would not be returned in v1.9 and above,")?;
    writeln!(f, "/// both arms need to be wrapped in conditional compilation predicates.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// ```rust,no_run")?;
    writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
    writeln!(f, "/// # use k8s_openapi::http;")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::{{")?;
    writeln!(f, "///     CreateCustomResourceDefinitionResponse,")?;
    writeln!(f, "///     CustomResourceDefinition,")?;
    writeln!(f, "/// }};")?;
    writeln!(f, "///")?;
    writeln!(f, "/// # fn main() -> Result<(), Box<std::error::Error>> {{")?;
    writeln!(f, "/// let response: CreateCustomResourceDefinitionResponse = unimplemented!();")?;
    writeln!(f, "/// let status_code: http::StatusCode = unimplemented!();")?;
    writeln!(f, "///")?;
    writeln!(f, "/// let custom_resource_definition: CustomResourceDefinition = k8s_match!(response, {{")?;
    writeln!(f, "///     k8s_if_1_8!(CreateCustomResourceDefinitionResponse::Other if status_code == http::StatusCode::CREATED => {{")?;
    writeln!(f, "///         // Parse response body into a CustomResourceDefinition")?;
    writeln!(f, "///         Ok(unimplemented!())")?;
    writeln!(f, "///     }}),")?;
    writeln!(f, "///     k8s_if_ge_1_9!(CreateCustomResourceDefinitionResponse::Created(custom_resource_definition) => {{")?;
    writeln!(f, "///         Ok(custom_resource_definition)")?;
    writeln!(f, "///     }}),")?;
    writeln!(f, r#"///     other => Err(format!("unexpected response {{}} {{:?}}", status_code, other)),"#)?;
    writeln!(f, "/// }})?;")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// #     Ok(())")?;
    writeln!(f, "/// # }}")?;
    writeln!(f, "/// ```")?;
    writeln!(f, "#[macro_export] macro_rules! k8s_match {{")?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ }}) => {{")?;
    writeln!(f, "        match $test {{ $($arms)* }}")?;
    writeln!(f, "    }};")?;

    for v in MIN..=MAX {
        writeln!(f)?;

        for (name, enabled) in &[("", version == Some(v)), ("_ge", cfg_ge(v, version)), ("_le", cfg_le(v, version))] {
            writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ k8s_if{}_1_{}!($($arm:tt)*), $($rest:tt)* }}) => {{", name, v)?;
            if *enabled {
                writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($arm)*, $($rest)* }})")?;
            }
            else {
                writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($rest)* }})")?;
            }
            writeln!(f, "    }};")?;
        }
    }

    writeln!(f)?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ $next_pat:pat $(if $cond:expr)? => $next_expr:expr, $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* $next_pat $(if $cond)? => $next_expr, }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f)?;
    writeln!(f, "    ($test:expr, {{ $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn cfg_ge(v: usize, version: Option<usize>) -> bool {
    if let Some(version) = version {
        v <= version
    }
    else {
        false
    }
}

fn cfg_le(v: usize, version: Option<usize>) -> bool {
    if let Some(version) = version {
        v >= version
    }
    else {
        false
    }
}
