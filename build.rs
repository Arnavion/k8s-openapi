fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    const MIN: usize = 30;
    const MAX: usize = 33;

    println!("cargo::rerun-if-env-changed=K8S_OPENAPI_ENABLED_VERSION");

    let enabled_version = {
        let mut enabled_versions =
            (MIN..=MAX).filter(|v| std::env::var(format!("CARGO_FEATURE_V1_{v}")).is_ok())
            .chain(
                std::env::var("K8S_OPENAPI_ENABLED_VERSION").ok()
                .and_then(|value| value.strip_prefix("1.").and_then(|value| value.parse::<usize>().ok())))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter();
        let v1 = enabled_versions.next().expect("\n\
            None of the v1_* features are enabled on the k8s-openapi crate.\n\
            \n\
            The k8s-openapi crate requires a feature to be enabled to indicate which version of Kubernetes it should support.\n\
            \n\
            If you're using k8s-openapi in a binary crate, enable the feature corresponding to the minimum version of API server that you want to support. \
            In case your binary crate does not directly depend on k8s-openapi, add a dependency on k8s-openapi and enable the corresponding feature in it.\n\
            \n\
            If you're using k8s-openapi in a library crate, add a dev-dependency on k8s-openapi and enable one of the features there. This way \
            the feature will be enabled when buildings tests and examples of your library, but not when building the library itself. \
            In case your library crate does not directly depend on k8s-openapi, add a dev-dependency on k8s-openapi and enable the corresponding feature in it.\n\
            \n\
            Alternatively, when running commands that do not build dev dependencies such as `cargo check` and `cargo doc`, you can set the `K8S_OPENAPI_ENABLED_VERSION` \
            environment variable, such as `K8S_OPENAPI_ENABLED_VERSION=1.50`.\n\
            \n\
            Library crates *must not* enable any features in their direct dependency on k8s-openapi, only in their dev-dependency. \
            The choice of which Kubernetes version to support should be left to the final binary crate, so only binary crates should enable a specific feature. \
            If library crates also enabled features, it could cause multiple features to be enabled simultaneously, which k8s-openapi does not support.\n\
            \n\
            If your library crate only supports a single specific version or a specific range of versions of Kubernetes, \
            please use the `k8s_if_*` version-specific macros or a build script to emit different code based on which feature gets enabled in the end.\n\
            \n\
            If you believe you *have* enabled a version feature and should not be seeing this error, check your Cargo.lock or run `cargo tree -i k8s-openapi` \
            to ensure that this version of k8s-openapi is the only one being used in your project.\n\
            \n\
            See the k8s-openapi docs for more details.
        ");
        if let Some(v2) = enabled_versions.next() {
            panic!(
                "\n\
                Both v1_{v1} and v1_{v2} features are enabled on the k8s-openapi crate. These feature indicates which version of Kubernetes the k8s-openapi crate should support. \
                Only one feature can be enabled at the same time.\n\
                \n\
                If you have enabled both of these features yourself, either via the k8s-openapi dependency in your Cargo.toml or via setting \
                the `K8S_OPENAPI_ENABLED_VERSION` env var, please remove one of them. If you are writing a library crate, do not enable any features at all. \
                Library crates *must not* enable any features on the k8s-openapi crate.\n\
                \n\
                If you have not enabled one or both of these features yourself, then one of the library crates in your dependency graph *has*. \
                Locate which library crates in your dependency graph depend on k8s-openapi and enable one or more of its features, and file a bug against them, citing this text. \
                You can search your Cargo.lock for \"k8s-openapi\" to discover these crates.\
            ");
        }
        v1
    };

    println!("cargo::metadata=version={}", 0x00_01_00_00_u32 | ((enabled_version as u32) << 8));

    {
        let mut enabled_version_possible_values = String::new();
        for v in MIN..=MAX {
            use std::fmt::Write;

            if !enabled_version_possible_values.is_empty() {
                enabled_version_possible_values.push_str(", ");
            }
            write!(enabled_version_possible_values, r#""1.{v}""#).unwrap();
        }
        println!("cargo::rustc-check-cfg=cfg(k8s_openapi_enabled_version, values({enabled_version_possible_values}))");
    }
    println!(r#"cargo::rustc-cfg=k8s_openapi_enabled_version="1.{enabled_version}""#);

    let mut f = {
        let mut out_file: std::path::PathBuf = std::env::var_os("OUT_DIR").ok_or("OUT_DIR not set")?.into();
        out_file.push("conditional_compilation_macros.rs");
        std::io::BufWriter::new(std::fs::File::create(out_file)?)
    };

    for v in MIN..=MAX {
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{v}` feature is enabled, otherwise it evaluates to nothing.")?;
        writeln!(f, "///")?;
        writeln!(f, "/// # Examples")?;
        writeln!(f, "///")?;
        writeln!(f, "/// ```rust")?;
        writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
        writeln!(f, "/// k8s_if_1_{v}! {{")?;
        writeln!(f, "///     use k8s_openapi::api::core::v1 as api;")?;
        writeln!(f, "/// }}")?;
        writeln!(f, "/// ```")?;
        if enabled_version == v {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{v} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}")?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{v} {{ ($($tt:tt)*) => {{ }}; }}")?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v1_{v}` or higher feature is enabled, otherwise it evaluates to nothing.")?;
        if enabled_version >= v {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{v} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}")?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{v} {{ ($($tt:tt)*) => {{ }}; }}")?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v1_{v}` or lower feature is enabled, otherwise it evaluates to nothing.")?;
        if enabled_version <= v {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{v} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}")?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{v} {{ ($($tt:tt)*) => {{ }}; }}")?;
        }
        writeln!(f)?;
    }

    writeln!(f, "/// A macro that emits a `match` expr with the given test expression and arms.")?;
    writeln!(f, "/// The match arms can be annotated with the other conditional compilation macros in this crate so that they're only emitted")?;
    writeln!(f, "/// if the predicate is true.")?;
    writeln!(f, "#[macro_export] macro_rules! k8s_match {{")?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ }}) => {{")?;
    writeln!(f, "        match $test {{ $($arms)* }}")?;
    writeln!(f, "    }};")?;

    for v in MIN..=MAX {
        writeln!(f)?;

        for (name_suffix, enabled) in [("", enabled_version == v), ("_ge", enabled_version >= v), ("_le", enabled_version <= v)] {
            writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ k8s_if{name_suffix}_1_{v}!($($arm:tt)*), $($rest:tt)* }}) => {{")?;
            if enabled {
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
