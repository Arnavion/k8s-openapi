#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cognitive_complexity,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::large_enum_variant,
    clippy::match_single_binding,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::upper_case_acronyms,
    clippy::use_self,
    rustdoc::bare_urls,
)]

//! Bindings for the Kubernetes client API, generated from the OpenAPI spec.
//!
//! Each supported version of Kubernetes is represented by a feature name (like `v1_9`). Only one such feature can be enabled at a time.
//!
//! These docs have been generated with the `

#![cfg_attr(k8s_openapi_enabled_version="1.18", doc = "v1_18")]
#![cfg_attr(k8s_openapi_enabled_version="1.19", doc = "v1_19")]
#![cfg_attr(k8s_openapi_enabled_version="1.20", doc = "v1_20")]
#![cfg_attr(k8s_openapi_enabled_version="1.21", doc = "v1_21")]
#![cfg_attr(k8s_openapi_enabled_version="1.22", doc = "v1_22")]
#![cfg_attr(k8s_openapi_enabled_version="1.23", doc = "v1_23")]
#![cfg_attr(k8s_openapi_enabled_version="1.24", doc = "v1_24")]

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
//! ## Client API
//!
#![cfg_attr(feature = "api", doc = r#"
(This requires the `api` feature to be enabled. The feature is enabled by default. See ["Crate features"](#crate-features) below for more details.)

This example executes the [`api::core::v1::Pod::list`] API operation to list all pods inside a namespace.
It demonstrates the common patterns implemented by all API operation functions in this crate:

1. The API function has required parameters and optional parameters. All optional parameters are taken as a single struct with optional fields.

   Specifically for the [`api::core::v1::Pod::list`] operation, the `namespace` parameter is required and taken by the function itself,
   while other optional parameters like `field_selector` are fields of the [`ListOptional`] struct. An instance of
   this struct is taken as the last parameter of `Pod::list`. This struct impls [`Default`] so that you can just pass in `Default::default()`
   if you don't want to specify values for any of the optional parameters.

   Some API operations have a single common type for optional parameters:

   - All create API take optional parameters using the [`CreateOptional`] struct.
   - All delete API take optional parameters using the [`DeleteOptional`] struct.
   - All list API take optional parameters using the [`ListOptional`] struct.
   - All patch API take optional parameters using the [`PatchOptional`] struct.
   - All replace API take optional parameters using the [`ReplaceOptional`] struct.
   - All watch API take optional parameters using the [`WatchOptional`] struct.
   - All delete-collection API take optional parameters using the [`DeleteOptional`] struct for delete options and the [`ListOptional`] struct for list options.

   Other API functions have their own `Optional` structs with fields corresponding to the specific parameters for those functions,
   such as [`api::core::v1::ReadPodLogOptional`] for [`api::core::v1::Pod::read_log`]

1. The function returns an [`http::Request`] value with the URL path, query string, and request body filled out according to the parameters
   given to the function. The function does *not* execute this request. You can execute this `http::Request` using any HTTP client library you want to use.
   It does not matter whether you use a synchronous client like `reqwest`, or an asynchronous client like `hyper`, or a mock client that returns bytes
   read from a test file.

1. For each API operation function, there is a corresponding response type. For `Pod::list` this is [`ListResponse`]`<`[`api::core::v1::Pod`]`>`.
   This is an enum with variants for each of the possible HTTP status codes that the operation can return, and contains the data that the API server would
   return corresponding to that status code. For example, the list-namespaced-pod operation returns a pod list with HTTP 200 OK, so one of the variants of
   that type is `Ok(`[`List`]`<`[`api::core::v1::Pod`]`>)`

1. The response types impl the [`Response`] trait, which contains a single [`Response::try_from_parts`] function. This function takes an [`http::StatusCode`]
   and a `&u8` byte buffer, and tries to parse the byte buffer as the response type. For example, if you executed the request and received an HTTP 200 OK response
   with some bytes, you could call `<ListResponse<Pod> as Response>::try_from_parts(status_code, buf)` and expect to get
   `Ok(ListResponse::<Pod>::Ok(pod_list))` from it.

   Once again, this design ensures that the crate is not tied to a specific HTTP client library or interface. It does not matter how you execute the HTTP request,
   nor whether your library is synchronous or asynchronous, since every HTTP client library gives you a way to get the HTTP response status code and the bytes
   of the response body.

1. The API operation function also returns another value next to the `http::Request`. This value is a function that takes an [`http::StatusCode`] and returns
   a [`ResponseBody`]`<ListResponse<Pod>>`. As mentioned above, `Response::try_from_parts` requires you to maintain a byte buffer for the response body.
   `ResponseBody` is a helper that maintains such a buffer internally. It provides an `append_slice()` function to append slices to this internal buffer,
   and a `parse()` function to parse the buffer as the expected type (`ListResponse<Pod>` in this case).

   It is not *necessary* to use the `ResponseBody` returned by the API operation function to parse the response. The `ResponseBody::parse` function is
   only a wrapper around the underlying `Response::try_from_parts` function, and handles growing and shrinking its inner buffer as necessary. It also
   helps ensure that the response body is parsed as the *correct* type for the operation, `ListResponse<Pod>` in this case, and not some other type.
   However, you can instead use your own byte buffer instead of the `ResponseBody` value and call `ListResponse<Pod>::try_from_parts` yourself.

1. The response types are enums with variants corresponding to HTTP status codes. For example, the `ListResponse<Pod>::Ok` variant corresponds to the
   HTTP 200 response of the list-namespaced-pod API.

   Each response enum also has an `Other` variant, that is yielded when the response status code does not match any of the other variants.
   This variant has a `Result<Option<`[`serde_json::Value`]`>, `[`serde_json::Error`]`>` value.

   If the response body is empty, this value will be `Ok(None)`.

   If the response body is not empty, this value will be an `Ok(Some(value))` or `Err(err)` from attempting to parse that body as a `serde_json::Value`.
   If you expect the response body to be a specific JSON type such as [`apimachinery::pkg::apis::meta::v1::Status`], you can use the `serde_json::Value`
   as a [`serde::Deserializer`] like `let status = <Status as Deserialize>::deserialize(value)?;`. On the other hand, if you expect the response body to not be
   a JSON value, then ignore the `Err(err)` and parse the raw bytes of the response into the appropriate type.

Also see the `get_single_value` and `get_multiple_values` functions in
[the `k8s-openapi-tests` directory in the repository](https://github.com/Arnavion/k8s-openapi/tree/master/k8s-openapi-tests/src)
for examples of how to use a synchronous client with this style of API.

```rust,no_run
// Re-export of the http crate since it's used in the public API
use k8s_openapi::http;

use k8s_openapi::api::core::v1 as api;

# struct Response;
# impl Response {
#     fn status_code(&self) -> http::StatusCode {
#         unimplemented!()
#     }
#     fn read_into(&self, _buf: &mut [u8]) -> std::io::Result<usize> {
#         unimplemented!()
#     }
# }
#
// Assume `execute` is some function that takes an `http::Request` and
// executes it synchronously or asynchronously to get a response. This is
// provided by your HTTP client library.
//
// Note that the `http::Request` values returned by API operation functions
// only have a URL path, query string and request body filled out. That is,
// they do *not* have a URL host. So the real `execute` implementation
// would first mutate the URL of the request to an absolute URL with
// the API server's authority, add authorization headers, etc before
// actually executing it.
fn execute(req: http::Request<Vec<u8>>) -> Response { unimplemented!(); }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a `http::Request` to list all the pods in the
    // "kube-system" namespace.
    let (request, response_body) =
        api::Pod::list("kube-system", Default::default())?;

    // Execute the request and get a response.
    // If this is an asynchronous operation, you would await
    // or otherwise yield to the event loop here.
    let response = execute(request);

    // Got a status code from executing the request.
    let status_code: http::StatusCode = response.status_code();

    // Construct the `ResponseBody<ListResponse<Pod>>` using the
    // constructor returned by the API function.
    let mut response_body = response_body(status_code);

    // Buffer used for each read from the HTTP response.
    let mut buf = Box::new([0_u8; 4096]);

    let pod_list = loop {
        // Read some bytes from the HTTP response into the buffer.
        // If this is an asynchronous operation, you would await or
        // yield to the event loop here.
        let read = response.read_into(&mut *buf)?;

        // `buf` now contains some data read from the response. Append it
        // to the `ResponseBody` and try to parse it into
        // the response type.
        response_body.append_slice(&buf[..read]);
        let response = response_body.parse();
        match response {
            // Successful response (HTTP 200 and parsed successfully)
            Ok(k8s_openapi::ListResponse::Ok(pod_list)) =>
                break pod_list,

            // Some unexpected response
            // (not HTTP 200, but still parsed successfully)
            Ok(other) => return Err(format!(
                "expected Ok but got {status_code} {other:?}").into()),

            // Need more response data.
            // Read more bytes from the response into the `ResponseBody`
            Err(k8s_openapi::ResponseError::NeedMoreData) => continue,

            // Some other error, like the response body being
            // malformed JSON or invalid UTF-8.
            Err(err) => return Err(format!(
                "error: {status_code} {err:?}").into()),
        }
    };

    for pod in pod_list.items {
        println!("{pod:#?}",);
    }

    Ok(())
}
```
"#)]
#![cfg_attr(not(feature = "api"), doc = r#"
The `api` feature has been disabled, so the client API is not available. See ["Crate features"](#crate-features) below for more details.
"#)]
//!
//!
//! # Crate features
//!
//! - This crate contains several `v1_*` features. Enabling one of the `v1_*` features selects which version of the Kubernetes API server this crate should target.
//!   For example, enabling the `v1_23` feature means the crate will only contain the API exposed by Kubernetes 1.23. It will not expose API
//!   that were removed in 1.23 or earlier, nor any API added in 1.24 or later.
//!
//! - The crate also contains a feature named `api`. If this feature is disabled, the library will only contain the resource types like [`api::core::v1::Pod`],
//!   and not the associated operation functions like
#![cfg_attr(feature = "api", doc = "[`api::core::v1::Pod::read`]")]
#![cfg_attr(not(feature = "api"), doc = "`api::core::v1::Pod::read`")]
//! . The `Response` and `Optional` types for the operation functions
//!   will also not be accessible.
//!
//!   This feature is enabled by default, but can be disabled if your crate does not need the operation functions to save on compile time and resources.
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
//! However, commands like `cargo check` and `cargo doc` do not build dev dependencies, so they would not enable the feature and would fail to build.
//! One way to resolve this is to add a feature to your library that enables one of the k8s-openapi `v1_*` features and is used only when running such commands.
//!
//! ```toml
//! [features]
//! __check = ["k8s-openapi/v1_50"]   # This feature is used internally for `cargo check` and `cargo doc`
//! ```
//!
//! Another way is to define the `K8S_OPENAPI_ENABLED_VERSION` env var on the command, such as `K8S_OPENAPI_ENABLED_VERSION=1.50 cargo check`.
//!
//!
//! # Conditional compilation
//!
//! As the previous section explained, library crates must not enable any version features in their `k8s-openapi` dependency. However, your library crate may
//! need to know about which version gets selected eventually.
//!
//! For example:
//!
//! 1. Your crate creates a service spec and wants to set the cluster IP. This field is only available in Kubernetes 1.20+,
//!    so you want your crate to fail to compile if a lower feature was enabled.
//!
//! 1. Your crate creates a service spec and wants to set the cluster IP, but you want it to be skipped when compiling for older versions.
//!
//! There are two ways for your crate to determine which feature of `k8s-openapi` is enabled:
//!
//! 1. The `k8s-openapi` crate exports [`k8s_if_*` macros,](#macros) which either expand to their contents or don't. See the docs of the macros for more details.
//!
//!    With these macros, the two cases above would be solved like this:
//!
//!    - ```rust,ignore
//!      // The compile_error!() is only emitted if 1.19 or lower is selected.
//!      k8s_openapi::k8s_if_le_1_19! {
//!          compile_error!("This crate requires the v1_20 (or higher) feature to be enabled on the k8s-openapi crate.");
//!      }
//!
//!      ...
//!
//!      let service_spec = k8s_openapi::api::core::v1::ServiceSpec {
//!          cluster_ips: ...,
//!          ...
//!      };
//!      ```
//!
//!    - ```rust,ignore
//!      let mut service_spec = k8s_openapi::api::core::v1::ServiceSpec {
//!          ...
//!      };
//!
//!      k8s_openapi::k8s_if_ge_1_20! {
//!          service_spec.cluster_ips = ...;
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
//!        // Thus, if the v1_20 feature was enabled, k8s_openapi_version would be 0x00_01_14_00
//!
//!        // The build script can now do arbitrary things with the information.
//!        // For example, it could define custom cfgs:
//!        if k8s_openapi_version >= 0x00_01_14_00 {
//!            println!(r#"cargo:rustc-cfg=k8s_service_spec_supports_cluster_ips"#);
//!        }
//!
//!        // or emit new source code files under OUT_DIR, or anything else a build script can do.
//!    }
//!    ```
//!
//!    With these cfgs, the two cases above would be solved like this:
//!
//!    - ```rust,ignore
//!      // The compile_error!() is only emitted if 1.19 or lower is selected.
//!      #[cfg(not(k8s_service_spec_supports_cluster_ips))]
//!      compile_error!("This crate requires the v1_20 (or higher) feature to be enabled on the k8s-openapi crate.");
//!
//!      ...
//!
//!      let service_spec = k8s_openapi::api::core::v1::ServiceSpec {
//!          cluster_ips: ...,
//!          ...
//!      };
//!      ```
//!
//!    - ```rust,ignore
//!      let service_spec = k8s_openapi::api::core::v1::ServiceSpec {
//!          #[cfg(not(k8s_service_spec_supports_cluster_ips))]
//!          cluster_ips: ...,
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
//! The [`k8s-openapi-derive` crate](https://crates.io/crates/k8s-openapi-derive) provides a custom derive for generating clientsets
//! for custom resources. See that crate's docs for more information.

pub use chrono;
#[cfg(feature = "api")]
pub use http;
#[cfg(feature = "api")]
pub use percent_encoding;
#[cfg(feature = "schemars")]
pub use schemars;
pub use serde;
pub use serde_json;
pub use serde_value;
#[cfg(feature = "api")]
pub use url;

/// A wrapper around a list of bytes.
///
/// Used in Kubernetes types whose JSON representation uses a base64-encoded string for a list of bytes.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteString(pub Vec<u8>);

impl<'de> serde::Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ByteString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "a base64-encoded string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ByteString(base64::decode_config(v, base64::STANDARD).map_err(serde::de::Error::custom)?))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl serde::Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        base64::encode_config(&self.0, base64::STANDARD).serialize(serializer)
    }
}

/// A trait applied to all Kubernetes resources.
pub trait Resource {
    /// The API version of the resource. This is a composite of [`Resource::GROUP`] and [`Resource::VERSION`] (eg `"apiextensions.k8s.io/v1beta1"`)
    /// or just the version for resources without a group (eg `"v1"`).
    ///
    /// This is the string used in the `apiVersion` field of the resource's serialized form.
    const API_VERSION: &'static str;

    /// The group of the resource, or the empty string if the resource doesn't have a group.
    const GROUP: &'static str;

    /// The kind of the resource.
    ///
    /// This is the string used in the `kind` field of the resource's serialized form.
    const KIND: &'static str;

    /// The version of the resource.
    const VERSION: &'static str;

    /// The URL path segment used to construct URLs related to this resource.
    ///
    /// For cluster- and namespaced-scoped resources, this is the plural name of the resource that is followed by the resource name.
    /// For example, [`api::core::v1::Pod`]'s value is `"pods"` and its URLs look like `.../pods/{name}`.
    ///
    /// For subresources, this is the subresource name that comes after the parent resource's name.
    /// For example, [`api::authentication::v1::TokenRequest`]'s value is `"token"`, and its URLs look like `.../serviceaccounts/{name}/token`.
    const URL_PATH_SEGMENT: &'static str;

    /// Indicates whether the resource is namespace-scoped or cluster-scoped or a subresource.
    ///
    /// If you need to restrict some generic code to resources of a specific scope, use this associated type to create a bound on the generic.
    /// For example, `fn foo<T: k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>>() { }` can only be called with cluster-scoped resources.
    type Scope: ResourceScope;
}

/// A trait applied to all Kubernetes resources that can be part of a corresponding list.
pub trait ListableResource: Resource {
    /// The kind of the list type of the resource.
    ///
    /// This is the string used in the `kind` field of the list type's serialized form.
    const LIST_KIND: &'static str;
}

/// A trait applied to all Kubernetes resources that have metadata.
pub trait Metadata: Resource {
    /// The type of the metadata object.
    type Ty;

    /// Gets a reference to the metadata of this resource value.
    fn metadata(&self) -> &<Self as Metadata>::Ty;

    /// Gets a mutable reference to the metadata of this resource value.
    fn metadata_mut(&mut self) -> &mut<Self as Metadata>::Ty;
}

/// Extracts the API version of the given resource value.
///
/// This just returns the [`Resource::API_VERSION`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn api_version<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::API_VERSION
}

/// Extracts the group of the given resource value.
///
/// This just returns the [`Resource::GROUP`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn group<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::GROUP
}

/// Extracts the kind of the given resource value.
///
/// This just returns the [`Resource::KIND`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn kind<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::KIND
}

/// Extracts the version of the given resource value.
///
/// This just returns the [`Resource::VERSION`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn version<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::VERSION
}

/// The scope of a [`Resource`].
pub trait ResourceScope {}

/// Indicates that a [`Resource`] is cluster-scoped.
pub struct ClusterResourceScope {}
impl ResourceScope for ClusterResourceScope {}

/// Indicates that a [`Resource`] is namespace-scoped.
pub struct NamespaceResourceScope {}
impl ResourceScope for NamespaceResourceScope {}

/// Indicates that a [`Resource`] is neither cluster-scoped nor namespace-scoped.
pub struct SubResourceScope {}
impl ResourceScope for SubResourceScope {}

mod deep_merge;
pub use self::deep_merge::DeepMerge;

/// The type of errors returned by the Kubernetes API functions that prepare the HTTP request.
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum RequestError {
    /// An error from preparing the HTTP request.
    Http(http::Error),

    /// An error while serializing a value into the JSON body of the HTTP request.
    Json(serde_json::Error),
}

#[cfg(feature = "api")]
impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Http(err) => write!(f, "{err}"),
            RequestError::Json(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "api")]
impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::Http(err) => Some(err),
            RequestError::Json(err) => Some(err),
        }
    }
}

/// A trait implemented by all response types corresponding to Kubernetes API functions.
#[cfg(feature = "api")]
pub trait Response: Sized {
    /// Tries to parse the response from the given status code and response body.
    ///
    /// If an instance of `Self` can be successfully parsed from the given byte buffer, the instance is returned,
    /// along with the number of bytes used up from the buffer. Remove those bytes from the buffer before calling
    /// this function again.
    ///
    /// If the buffer does not contain enough bytes to be able to parse an instance of `Self`, the function returns
    /// `Err(ResponseError::NeedMoreData)`. Append more bytes into the buffer, then call this function again.
    ///
    /// Also see the [`ResponseBody`] type.
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ResponseError>;
}

/// This struct provides an easy way to parse a byte buffer into a Kubernetes API function's response.
///
/// All API function responses implement the [`Response`] trait, and are constructed by calling their [`Response::try_from_parts`] function.
/// If this function returns `Err(ResponseError::NeedMoreData)`, that means more bytes need to be appended to the function. Alternatively,
/// if the function returns `Ok((value, num_bytes_read))`, then `num_bytes_read` bytes need to be popped off from the front of the buffer.
///
/// The `ResponseBody` struct contains an internal dynamic buffer, and provides `append_slice` and `parse` functions to help with this.
/// `append_slice` appends the slice you give it to its internal buffer, and `parse` uses the [`Response::try_from_parts`] function to parse
/// the response out of the buffer, and truncates it accordingly.
///
/// You do not *have* to use this type to parse the response, say if you want to manage your own byte buffers. You can use
/// `<T as Response>::try_from_parts` directly instead.
#[cfg(feature = "api")]
pub struct ResponseBody<T> {
    /// The HTTP status code of the response.
    pub status_code: http::StatusCode,

    buf: bytes::BytesMut,

    _response: std::marker::PhantomData<fn() -> T>,
}

#[cfg(feature = "api")]
impl<T> ResponseBody<T> where T: Response {
    /// Construct a value for a response that has the specified HTTP status code.
    pub fn new(status_code: http::StatusCode) -> Self {
        ResponseBody {
            status_code,
            buf: Default::default(),
            _response: Default::default(),
        }
    }

    /// Append a slice of data from the HTTP response to this buffer.
    pub fn append_slice(&mut self, buf: &[u8]) {
        self.buf.extend_from_slice(buf);
    }

    /// Try to parse all the data buffered so far into a response type.
    pub fn parse(&mut self) -> Result<T, ResponseError> {
        match T::try_from_parts(self.status_code, &*self.buf) {
            Ok((result, read)) => {
                self.advance(read);
                Ok(result)
            },

            Err(err) => Err(err),
        }
    }

    /// Drop the first `cnt` bytes of this buffer.
    ///
    /// This is useful for skipping over malformed bytes, such as invalid utf-8 sequences when parsing streaming `String` responses
    /// like from [`api::core::v1::Pod::read_log`].
    ///
    /// # Panics
    ///
    /// This function panics if `cnt` is greater than the length of the internal buffer.
    pub fn advance(&mut self, cnt: usize) {
        bytes::Buf::advance(&mut self.buf, cnt);
    }
}

#[cfg(feature = "api")]
impl<T> std::ops::Deref for ResponseBody<T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.buf
    }
}

/// The type of errors from parsing an HTTP response as one of the Kubernetes API functions' response types.
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ResponseError {
    /// An error from deserializing the HTTP response, indicating more data is needed to complete deserialization.
    NeedMoreData,

    /// An error while deserializing the HTTP response as a JSON value, indicating the response is malformed.
    Json(serde_json::Error),

    /// An error while deserializing the HTTP response as a string, indicating that the response data is not UTF-8.
    Utf8(std::str::Utf8Error),
}

#[cfg(feature = "api")]
impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::NeedMoreData => f.write_str("need more response data"),
            ResponseError::Json(err) => write!(f, "{err}"),
            ResponseError::Utf8(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "api")]
impl std::error::Error for ResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResponseError::NeedMoreData => None,
            ResponseError::Json(err) => Some(err),
            ResponseError::Utf8(err) => Some(err),
        }
    }
}

/// Extensions to the percent-encoding crate
#[cfg(feature = "api")]
pub mod percent_encoding2 {
    /// Ref <https://url.spec.whatwg.org/#path-percent-encode-set>
    pub const PATH_SEGMENT_ENCODE_SET: &percent_encoding::AsciiSet =
        &percent_encoding::CONTROLS
        .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`') // fragment percent-encode set
        .add(b'#').add(b'?').add(b'{').add(b'}'); // path percent-encode set
}

#[cfg(k8s_openapi_enabled_version="1.18")] mod v1_18;
#[cfg(k8s_openapi_enabled_version="1.18")] pub use self::v1_18::*;

#[cfg(k8s_openapi_enabled_version="1.19")] mod v1_19;
#[cfg(k8s_openapi_enabled_version="1.19")] pub use self::v1_19::*;

#[cfg(k8s_openapi_enabled_version="1.20")] mod v1_20;
#[cfg(k8s_openapi_enabled_version="1.20")] pub use self::v1_20::*;

#[cfg(k8s_openapi_enabled_version="1.21")] mod v1_21;
#[cfg(k8s_openapi_enabled_version="1.21")] pub use self::v1_21::*;

#[cfg(k8s_openapi_enabled_version="1.22")] mod v1_22;
#[cfg(k8s_openapi_enabled_version="1.22")] pub use self::v1_22::*;

#[cfg(k8s_openapi_enabled_version="1.23")] mod v1_23;
#[cfg(k8s_openapi_enabled_version="1.23")] pub use self::v1_23::*;

#[cfg(k8s_openapi_enabled_version="1.24")] mod v1_24;
#[cfg(k8s_openapi_enabled_version="1.24")] pub use self::v1_24::*;

include!(concat!(env!("OUT_DIR"), "/conditional_compilation_macros.rs"));
