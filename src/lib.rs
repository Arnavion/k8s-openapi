#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_lazy_continuation,
    clippy::doc_markdown,
    clippy::doc_overindented_list_items,
    clippy::large_enum_variant,
    clippy::match_single_binding,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::type_complexity,
    rustdoc::bare_urls,
)]

//! Bindings for the Kubernetes client API, generated from the OpenAPI spec.
//!
//! Each supported version of Kubernetes is represented by a feature name (like `v1_9`). Only one such feature can be enabled at a time.
//!
//! These docs have been generated with the `

#![cfg_attr(k8s_openapi_enabled_version="1.30", doc = "v1_30")]
#![cfg_attr(k8s_openapi_enabled_version="1.31", doc = "v1_31")]
#![cfg_attr(k8s_openapi_enabled_version="1.32", doc = "v1_32")]
#![cfg_attr(k8s_openapi_enabled_version="1.33", doc = "v1_33")]

//! ` feature enabled. To see docs for one of the other supported versions, please generate the docs locally with `cargo doc --features 'v1_<>'`
//!
//!
//! # Examples
//!
//! ## Resources
//!
//! This example creates an instance of [`api::core::v1::PodSpec`] with no other properties set, and pretty-prints it.
//!
//! ```rust
//! use k8s_openapi::api::core::v1 as api;
//!
//! fn main() {
//!     let pod_spec: api::PodSpec = Default::default();
//!     println!("{pod_spec:#?}");
//! }
//! ```
//!
//!
//! # Crate features
//!
//! This crate contains several `v1_*` features. Enabling one of the `v1_*` features selects which version of the Kubernetes API server this crate should target.
//! For example, enabling the `v1_50` feature means the crate will only contain the API exposed by Kubernetes 1.50. It will not expose API
//! that were removed in 1.50 or earlier, nor any API added in 1.51 or later.
//!
//! One and only one of the `v1_*` features must be enabled at the same time, otherwise the crate will not compile. This ensures that all crates in the crate graph
//! use the same types. If it was possible for one library crate to use `api::core::v1::Pod` corresponding to v1.50 and another to use the type
//! corresponding to v1.51, an application would not be able to use the same `Pod` value with both.
//!
//! Thus, it is recommended that only application crates must enable one of the `v1_*` features, corresponding to the version of Kubernetes
//! that the application wants to support.
//!
//! ```toml
//! # For application crates
//!
//! [dependencies]
//! k8s-openapi = { version = "...", features = ["v1_50"] }
//! ```
//!
//! If you're writing a library crate, your crate *must not* enable any features of `k8s-openapi` directly. The choice of which feature to enable
//! must be left to any application crates that use your library. This ensures that all `k8s-openapi`-using dependencies in that application crate's dependency graph
//! use the same set of `k8s-openapi` types and are interoperable.
//!
//! If your library crate has tests or examples, you should also add a dev-dependency on `k8s-openapi` in addition to the direct dependency,
//! and enable a version feature only for that dev-dependency.
//!
//! ```toml
//! # For library crates
//!
//! [dependencies]
//! k8s-openapi = "..."
//!
//! [dev-dependencies]
//! k8s-openapi = { version = "...", features = ["v1_50"] }
//! ```
//!
//! However, commands like `cargo check` and `cargo doc` do not build dev dependencies, so they will not enable the feature and will fail to build. There are two ways
//! you can resolve this:
//!
//! 1. Add a feature to your library that enables one of the k8s-openapi `v1_*` features, and then remember to enable this feature when running such commands.
//!
//!    ```toml
//!    [features]
//!    __check = ["k8s-openapi/v1_50"]
//!    ```
//!
//!    ```sh
//!    $ cargo check --features __check
//!    ```
//!
//! 1. Define the `K8S_OPENAPI_ENABLED_VERSION` env var when running such commands:
//!
//!    ```sh
//!    $ K8S_OPENAPI_ENABLED_VERSION=1.50 cargo check
//!    ```
//!
//!
//! # Conditional compilation
//!
//! As the previous section explained, library crates must not enable any version features in their `k8s-openapi` dependency. However, your library crate may
//! need to know about which version gets selected eventually.
//!
//! For example:
//!
//! 1. Your crate creates a `PodSecurityContext` and wants to set the `supplemental_groups_policy` field. This field is only available in Kubernetes 1.31+,
//!    so you want your crate to fail to compile if a lower feature was enabled.
//!
//! 1. Your crate creates a `PodSecurityContext` and wants to set the `supplemental_groups_policy` field, but it's okay to not set it when compiling for older versions.
//!
//! There are two ways for your crate to determine which feature of `k8s-openapi` is enabled:
//!
//! 1. The `k8s-openapi` crate exports [`k8s_if_*` macros,](#macros) which either expand to their contents or don't. See the docs of the macros for more details.
//!
//!    With these macros, the two cases above would be solved like this:
//!
//!    - ```rust,ignore
//!      // The compile_error!() is only emitted if 1.30 or lower is selected.
//!      k8s_openapi::k8s_if_le_1_30! {
//!          compile_error!("This crate requires the v1_31 (or higher) feature to be enabled on the k8s-openapi crate.");
//!      }
//!
//!      ...
//!
//!      let pod_security_context = k8s_openapi::api::core::v1::PodSecurityContext {
//!          supplemental_groups_policy: ...,
//!          ...
//!      };
//!      ```
//!
//!    - ```rust,ignore
//!      let mut pod_security_context = k8s_openapi::api::core::v1::PodSecurityContext {
//!          ...
//!      };
//!
//!      k8s_openapi::k8s_if_ge_1_31! {
//!          pod_security_context.supplemental_groups_policy = ...;
//!      }
//!      ```
//!
//! 1. The `k8s-openapi` crate emits the selected version number as metadata that your crate can read in a build script
//!    from the `DEP_K8S_OPENAPI_*_VERSION` env var.
//!
//!    ```rust,no_run
//!    // Your crate's build.rs
//!
//!    fn main() {
//!        let k8s_openapi_version: u32 =
//!            std::env::vars_os()
//!            .find_map(|(key, value)| {
//!                let key = key.into_string().ok()?;
//!                if key.starts_with("DEP_K8S_OPENAPI_") && key.ends_with("_VERSION") {
//!                    let value = value.into_string().ok()?;
//!                    Some(value)
//!                }
//!                else {
//!                    None
//!                }
//!            }).expect("DEP_K8S_OPENAPI_*_VERSION must have been set by k8s-openapi")
//!            .parse().expect("DEP_K8S_OPENAPI_*_VERSION is malformed");
//!
//!        // k8s_openapi_version has the format 0x00_MM_NN_00.
//!        //
//!        // - MM is the major version.
//!        // - NN is the minor version.
//!        //
//!        // Thus, if the v1_31 feature was enabled, k8s_openapi_version would be 0x00_01_1F_00
//!
//!        // The build script can now do arbitrary things with the information.
//!        // For example, it could define custom cfgs:
//!        if k8s_openapi_version >= 0x00_01_1F_00 {
//!            println!(r#"cargo::rustc-cfg=k8s_pod_security_context_supports_supplemental_groups_policy"#);
//!        }
//!
//!        // ... or emit new source code files under OUT_DIR, or anything else a build script can do.
//!    }
//!    ```
//!
//!    With this cfg, the two cases above would be solved like this:
//!
//!    - ```rust,ignore
//!      // The compile_error!() is only emitted if 1.30 or lower is selected.
//!      #[cfg(not(k8s_pod_security_context_supports_supplemental_groups_policy))]
//!      compile_error!("This crate requires the v1_31 (or higher) feature to be enabled on the k8s-openapi crate.");
//!
//!      ...
//!
//!      let pod_security_context = k8s_openapi::api::core::v1::PodSecurityContext {
//!          supplemental_groups_policy: ...,
//!          ...
//!      };
//!      ```
//!
//!    - ```rust,ignore
//!      let pod_security_context = k8s_openapi::api::core::v1::PodSecurityContext {
//!          #[cfg(not(k8s_pod_security_context_supports_supplemental_groups_policy))]
//!          supplemental_groups_policy: ...,
//!          ...
//!      };
//!      ```
//!
//! Note that both approaches require your crate to have a direct dependency on the `k8s-openapi` crate. Neither approach is available if your crate
//! only has a transitive dependency on the `k8s-openapi` crate.
//!
//! The macros approach is easier to use since it doesn't require a build script.
//!
//! The build script method lets you emit arbitrary cfgs, emit arbitrary source code, and generally gives you more options, at the cost of needing a build script.
//! `cfg()`s can be used in places where macros cannot, such as how the second example above shows it being used on a single field in a struct literal.
//!
//!
//! # Custom resource definitions
//!
//! The [`k8s-openapi-derive` crate](https://crates.io/crates/k8s-openapi-derive) provides a custom derive for generating types
//! for custom resources. See that crate's docs for more information.

#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc as std;
#[cfg(feature = "std")]
extern crate std;

pub use chrono;
#[cfg(feature = "schemars")]
pub use schemars;
pub use serde;
pub use serde_json;


#[path = "byte_string.rs"]
mod _byte_string;
pub use _byte_string::ByteString;

#[path = "deep_merge.rs"]
mod _deep_merge;
pub use self::_deep_merge::{DeepMerge, strategies as merge_strategies};

#[path = "resource.rs"]
mod _resource;
pub use _resource::{
    Resource,
    ResourceScope, ClusterResourceScope, NamespaceResourceScope, SubResourceScope,
    ListableResource,
    Metadata,
    api_version, group, kind, version,
};

#[cfg(k8s_openapi_enabled_version="1.30")] mod v1_30;
#[cfg(k8s_openapi_enabled_version="1.30")] pub use self::v1_30::*;

#[cfg(k8s_openapi_enabled_version="1.31")] mod v1_31;
#[cfg(k8s_openapi_enabled_version="1.31")] pub use self::v1_31::*;

#[cfg(k8s_openapi_enabled_version="1.32")] mod v1_32;
#[cfg(k8s_openapi_enabled_version="1.32")] pub use self::v1_32::*;

#[cfg(k8s_openapi_enabled_version="1.33")] mod v1_33;
#[cfg(k8s_openapi_enabled_version="1.33")] pub use self::v1_33::*;

include!(concat!(env!("OUT_DIR"), "/conditional_compilation_macros.rs"));
