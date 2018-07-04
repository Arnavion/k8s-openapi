const MIN: usize = 7;
const MAX: usize = 11;

fn main() -> Result<(), Box<std::error::Error>> {
    use std::io::Write;

    let mut f = {
        let mut out_file: std::path::PathBuf = std::env::var_os("OUT_DIR").ok_or_else(|| "OUT_DIR not set")?.into();
        out_file.push("conditional_compilation_macros.rs");
        std::io::BufWriter::new(std::fs::File::create(out_file)?)
    };

    for v in MIN..=MAX {
        writeln!(f, r#"#[cfg(feature = "v1_{}")]"#, v)?;
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "///")?;
        writeln!(f, "/// # Examples")?;
        writeln!(f, "///")?;
        writeln!(f, "/// ```rust")?;
        writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
        writeln!(f, "/// k8s_if_1_{}! {{", v)?;
        if v == 7 {
            writeln!(f, "///     use k8s_openapi::v1_{}::kubernetes::pkg::api::v1 as api;", v)?;
        }
        else {
            writeln!(f, "///     use k8s_openapi::v1_{}::api::core::v1 as api;", v)?;
        }
        writeln!(f, "///     use k8s_openapi::v1_{}::apimachinery::pkg::apis::meta::v1 as meta;", v)?;
        writeln!(f, "/// }}")?;
        writeln!(f, "/// ```")?;
        writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        writeln!(f, r#"#[cfg(not(feature = "v1_{}"))]"#, v)?;
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "///")?;
        writeln!(f, "/// # Examples")?;
        writeln!(f, "///")?;
        writeln!(f, "/// ```rust")?;
        writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
        writeln!(f, "/// k8s_if_1_{}! {{", v)?;
        if v == 7 {
            writeln!(f, "///     use k8s_openapi::v1_{}::kubernetes::pkg::api::v1 as api;", v)?;
        }
        else {
            writeln!(f, "///     use k8s_openapi::v1_{}::api::core::v1 as api;", v)?;
        }
        writeln!(f, "///     use k8s_openapi::v1_{}::apimachinery::pkg::apis::meta::v1 as meta;", v)?;
        writeln!(f, "/// }}")?;
        writeln!(f, "/// ```")?;
        writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        writeln!(f)?;

        writeln_cfg_ge(&mut f, v, true)?;
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or higher feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        writeln_cfg_ge(&mut f, v, false)?;
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or higher feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        writeln!(f)?;

        writeln_cfg_le(&mut f, v, true)?;
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or lower feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        writeln_cfg_le(&mut f, v, false)?;
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or lower feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        writeln!(f)?;
    }

    writeln!(f, "/// A macro that emits a `match` expr with the given test expression and arms.")?;
    writeln!(f, "/// The match arms can be annotated with the other conditional compilation macros in this crate so that they're only emitted")?;
    writeln!(f, "/// if the predicate is true.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// # Examples")?;
    writeln!(f, "///")?;
    writeln!(f, "/// The `CustomResourceDefinition::create_apiextensions_v1beta1_custom_resource_definition` function returns an `HTTP 201 CREATED`")?;
    writeln!(f, "/// when it succeeds, but the codegen for v1.8 does not have a `Created` variant in the response type. So extracting the successful result from")?;
    writeln!(f, "/// the response requires matching `CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Other` for v1.8 and below")?;
    writeln!(f, "/// and `CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Created` for v1.9 and above.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// Since `CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Created` does not exist in v1.8 and below,")?;
    writeln!(f, "/// and `CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Other` would not be returned in v1.9 and above,")?;
    writeln!(f, "/// both arms need to be wrapped in conditional compilation predicates.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// ```rust,no_run")?;
    writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
    writeln!(f, "/// # use k8s_openapi::http;")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// # k8s_if_1_8! {{")?;
    writeln!(f, "/// #     use k8s_openapi::v1_8::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;")?;
    writeln!(f, "/// # }}")?;
    writeln!(f, "/// # k8s_if_1_9! {{")?;
    writeln!(f, "/// #     use k8s_openapi::v1_9::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;")?;
    writeln!(f, "/// # }}")?;
    writeln!(f, "/// # k8s_if_1_10! {{")?;
    writeln!(f, "/// #     use k8s_openapi::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;")?;
    writeln!(f, "/// # }}")?;
    writeln!(f, "/// # k8s_if_1_11! {{")?;
    writeln!(f, "/// #     use k8s_openapi::v1_11::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;")?;
    writeln!(f, "/// # }}")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// # fn main() -> Result<(), Box<std::error::Error>> {{")?;
    writeln!(f, "/// #     k8s_if_ge_1_8! {{")?;
    writeln!(f, "/// use apiextensions::CreateApiextensionsV1beta1CustomResourceDefinitionResponse;")?;
    writeln!(f, "///")?;
    writeln!(f, "/// let response: CreateApiextensionsV1beta1CustomResourceDefinitionResponse = unimplemented!();")?;
    writeln!(f, "/// let status_code: http::StatusCode = unimplemented!();")?;
    writeln!(f, "///")?;
    writeln!(f, "/// let custom_resource_definition: apiextensions::CustomResourceDefinition = k8s_match!(response, {{")?;
    writeln!(f, "///     k8s_if_le_1_8!(CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Other if status_code == ::http::StatusCode::CREATED => {{")?;
    writeln!(f, "///         // Parse response body into a CustomResourceDefinition")?;
    writeln!(f, "///         Ok(unimplemented!())")?;
    writeln!(f, "///     }}),")?;
    writeln!(f, "///     k8s_if_ge_1_9!(CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Created(custom_resource_definition) => {{")?;
    writeln!(f, "///         Ok(custom_resource_definition)")?;
    writeln!(f, "///     }}),")?;
    writeln!(f, r#"///     other => Err(format!("unexpected response {{}} {{:?}}", status_code, other)),"#)?;
    writeln!(f, "/// }})?;")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// #     }}")?;
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
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ k8s_if_1_{}!($($arm:tt)*), $($rest:tt)* }}) => {{", v)?;
        writeln!(f, "        _k8s_match_1_{}!(@inner {{ $test }} {{ $($arms)* }} {{ ($($arm)*), $($rest)* }})", v)?;
        writeln!(f, "    }};")?;
        writeln!(f)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ k8s_if_ge_1_{}!($($arm:tt)*), $($rest:tt)* }}) => {{", v)?;
        writeln!(f, "        _k8s_match_ge_1_{}!(@inner {{ $test }} {{ $($arms)* }} {{ ($($arm)*), $($rest)* }})", v)?;
        writeln!(f, "    }};")?;
        writeln!(f)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ k8s_if_le_1_{}!($($arm:tt)*), $($rest:tt)* }}) => {{", v)?;
        writeln!(f, "        _k8s_match_le_1_{}!(@inner {{ $test }} {{ $($arms)* }} {{ ($($arm)*), $($rest)* }})", v)?;
        writeln!(f, "    }};")?;
    }

    writeln!(f)?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ $next_pat:pat => $next_expr:expr, $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* $next_pat => $next_expr, }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f)?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ $next_pat:pat if $cond:expr => $next_expr:expr, $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* $next_pat if $cond => $next_expr, }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f)?;
    writeln!(f, "    ($test:expr, {{ $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f, "}}")?;

    for v in MIN..=MAX {
        writeln!(f)?;
        writeln!(f, r#"#[cfg(feature = "v1_{}")]"#, v)?;
        writeln!(f, "#[doc(hidden)] #[macro_export] macro_rules! _k8s_match_1_{} {{", v)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ ($($arm:tt)*), $($rest:tt)* }}) => {{")?;
        writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($arm)*, $($rest)* }})")?;
        writeln!(f, "    }};")?;
        writeln!(f, "}}")?;
        writeln!(f, r#"#[cfg(not(feature = "v1_{}"))]"#, v)?;
        writeln!(f, "#[doc(hidden)] #[macro_export] macro_rules! _k8s_match_1_{} {{", v)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ ($($arm:tt)*), $($rest:tt)* }}) => {{")?;
        writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($rest)* }})")?;
        writeln!(f, "    }};")?;
        writeln!(f, "}}")?;
        writeln!(f)?;
        writeln_cfg_ge(&mut f, v, true)?;
        writeln!(f, "#[doc(hidden)] #[macro_export] macro_rules! _k8s_match_ge_1_{} {{", v)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ ($($arm:tt)*), $($rest:tt)* }}) => {{")?;
        writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($arm)*, $($rest)* }})")?;
        writeln!(f, "    }};")?;
        writeln!(f, "}}")?;
        writeln!(f)?;
        writeln_cfg_ge(&mut f, v, false)?;
        writeln!(f, "#[doc(hidden)] #[macro_export] macro_rules! _k8s_match_ge_1_{} {{", v)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ ($($arm:tt)*), $($rest:tt)* }}) => {{")?;
        writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($rest)* }})")?;
        writeln!(f, "    }};")?;
        writeln!(f, "}}")?;
        writeln!(f)?;
        writeln_cfg_le(&mut f, v, true)?;
        writeln!(f, "#[doc(hidden)] #[macro_export] macro_rules! _k8s_match_le_1_{} {{", v)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ ($($arm:tt)*), $($rest:tt)* }}) => {{")?;
        writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($arm)*, $($rest)* }})")?;
        writeln!(f, "    }};")?;
        writeln!(f, "}}")?;
        writeln!(f)?;
        writeln_cfg_le(&mut f, v, false)?;
        writeln!(f, "#[doc(hidden)] #[macro_export] macro_rules! _k8s_match_le_1_{} {{", v)?;
        writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ ($($arm:tt)*), $($rest:tt)* }}) => {{")?;
        writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($rest)* }})")?;
        writeln!(f, "    }};")?;
        writeln!(f, "}}")?;
    }

    Ok(())
}

fn writeln_cfg_ge<W>(mut f: W, v: usize, yes: bool) -> std::io::Result<()> where W: std::io::Write {
    if yes {
        write!(f, "#[cfg(any(")?;
    }
    else {
        write!(f, "#[cfg(not(any(")?;
    }

    for (i, v_ge) in (v..=MAX).enumerate() {
        if i == 0 {
            write!(f, r#"feature = "v1_{}""#, v_ge)?;
        }
        else {
            write!(f, r#", feature = "v1_{}""#, v_ge)?;
        }
    }

    if yes {
        writeln!(f, "))]")?;
    }
    else {
        writeln!(f, ")))]")?;
    }

    Ok(())
}

fn writeln_cfg_le<W>(mut f: W, v: usize, yes: bool) -> std::io::Result<()> where W: std::io::Write {
    if yes {
        write!(f, "#[cfg(any(")?;
    }
    else {
        write!(f, "#[cfg(not(any(")?;
    }

    for (i, v_le) in (MIN..=v).enumerate() {
        if i == 0 {
            write!(f, r#"feature = "v1_{}""#, v_le)?;
        }
        else {
            write!(f, r#", feature = "v1_{}""#, v_le)?;
        }
    }

    if yes {
        writeln!(f, "))]")?;
    }
    else {
        writeln!(f, ")))]")?;
    }

    Ok(())
}
