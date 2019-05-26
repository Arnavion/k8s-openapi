#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cognitive_complexity,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::large_enum_variant,
    clippy::single_match_else,
    clippy::type_complexity,
    clippy::use_self,
)]

//! Bindings for the Kubernetes client API, generated from the OpenAPI spec.
//!
//! Each supported version of Kubernetes is represented by a feature name (like `v1_9`). Only one such feature can be enabled at a time.
//!
//! **These docs have been generated with the `

#![cfg_attr(feature = "v1_8", doc = "v1_8")]
#![cfg_attr(feature = "v1_9", doc = "v1_9")]
#![cfg_attr(feature = "v1_10", doc = "v1_10")]
#![cfg_attr(feature = "v1_11", doc = "v1_11")]
#![cfg_attr(feature = "v1_12", doc = "v1_12")]
#![cfg_attr(feature = "v1_13", doc = "v1_13")]
#![cfg_attr(feature = "v1_14", doc = "v1_14")]

//! ` feature enabled. To see docs for one of the other supported versions, please generate the docs locally with `cargo doc --features 'v1_<>'`**
//!
//! If you're writing a library crate that supports multiple versions of Kubernetes (eg >= v1.9), it's recommended that your crate does *not*
//! enable the corresponding feature directly (eg `k8s-openapi = { features = ["v1_9"] }`). Instead, let the application crate that uses your library
//! enable the feature corresponding to the version of Kubernetes that *it* supports. This ensures that the entire crate graph can use a common set
//! of types from this crate.
//!
//! For things that differ between versions, such as the fully-qualified paths of imports, use the `k8s_*` macros to emit different code
//! depending on which feature eventually gets enabled. See the docs of the macros, and the `k8s-openapi-tests` directory in the repository,
//! for more details.
//!
//! Similarly, if your crate does not support some versions of Kubernetes (eg <= 1.10), you can put something like this at the top of your crate root:
//!
//! ```rust,ignore
//! #[macro_use] extern crate k8s_openapi;
//!
//! k8s_if_le_1_10! {
//!     compile_error!("This crate requires v1_11 or higher feature to be enabled in the k8s-openapi crate.");
//! }
//! ```
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
//!     println!("{:#?}", pod_spec);
//! }
//! ```
//!
//! ## Client API
//!
//! This example executes the [`api::core::v1::Pod::list_namespaced_pod`] API operation to list all pods inside a namespace.
//! It demonstrates the common patterns implemented by all API operation functions in this crate:
//!
//! 1. The API function has required parameters and optional parameters. All optional parameters are taken as a single struct with optional fields.
//!
//!    Specifically for the [`api::core::v1::Pod::list_namespaced_pod`] operation, the `namespace` parameter is required and taken by the function itself,
//!    while other optional parameters like `field_selector` are fields of the [`ListOptional`] struct. An instance of
//!    this struct is taken as the last parameter of `Pod::list_namespaced_pod`. This struct impls [`Default`] so that you can just pass in `Default::default()`
//!    if you don't want to specify values for any of the optional parameters.
//!
//!    All list API that take optional parameters do so using the [`ListOptional`] struct. Similarly, all watch API that take optional parameters do so using
//!    the [`WatchOptional`] struct. Other API functions with unique parameters have their own `Optional` structs, such as [`api::core::v1::DeleteNamespacedPodOptional`]
//!    for [`api::core::v1::Pod::delete_namespaced_pod`]
//!
//! 1. The function returns an [`http::Request`] value with the URL path, query string, and request body filled out according to the parameters
//!    given to the function. The function does *not* execute this request. You can execute this `http::Request` using any HTTP client library you want to use.
//!    It does not matter whether you use a synchronous client like `reqwest`, or an asynchronous client like `hyper`, or a mock client that returns bytes
//!    read from a test file.
//!
//! 1. For each API operation function, there is a corresponding response type. For `Pod::list_namespaced_pod` this is [`api::core::v1::ListNamespacedPodResponse`].
//!    This is an enum with variants for each of the possible HTTP status codes that the operation can return, and contains the data that the API server would
//!    return corresponding to that status code. For example, the list-namespaced-pod operation returns a pod list with HTTP 200 OK, so one of the variants of
//!    `ListNamespacedPodResponse` is `Ok(`[`api::core::v1::PodList`]`)`
//!
//! 1. The response types impl the [`Response`] trait, which contains a single [`Response::try_from_parts`] function. This function takes an [`http::StatusCode`]
//!    and a `&u8` byte buffer, and tries to parse the byte buffer as the response type. For example, if you executed the request and received an HTTP 200 OK response
//!    with some bytes, you could call `<ListNamespacedPodResponse as Response>::try_from_parts(status_code, buf)` and expect to get
//!    `Ok(ListNamespacedPodResponse::Ok(pod_list))` from it.
//!
//!    Once again, this design ensures that the crate is not tied to a specific HTTP client library or interface. It does not matter how you execute the HTTP request,
//!    nor whether your library is synchronous or asynchronous, since every HTTP client library gives you a way to get the HTTP response status code and the bytes
//!    of the response body.
//!
//! 1. The API operation function also returns another value next to the `http::Request`. This value is a function that takes an [`http::StatusCode`] and returns
//!    a [`ResponseBody`]`<ListNamespacedPodResponse>`. As mentioned above, `Response::try_from_parts` requires you to maintain a byte buffer for the response body.
//!    `ResponseBody` is a helper that maintains such a buffer internally. It provides an `append_slice()` function to append slices to this internal buffer,
//!    and a `parse()` function to parse the buffer as the expected type (`ListNamespacedPodResponse` in this case).
//!
//!    It is not *necessary* to use the `ResponseBody` returned by the API operation function to parse the response. The `ResponseBody::parse` function is
//!    only a wrapper around the underlying `Response::try_from_parts` function, and handles growing and shrinking its inner buffer as necessary. It also
//!    helps ensure that the response body is parsed as the *correct* type for the operation, `ListNamespacedPodResponse` in this case, and not some other type.
//!    However, you can instead use your own byte buffer instead of the `ResponseBody` value and call `ListNamespacedPodResponse::try_from_parts` yourself.
//!
//! 1. The response types are enums with variants corresponding to HTTP status codes. For example, the `ListNamespacedPodResponse::Ok` variant corresponds to the
//!    HTTP 200 response of the list-namespaced-pod API.
//! 
//!    Each response enum also has an `Other` variant, that is yielded when the response status code does not match any of the other variants.
//!    This variant has a `Result<Option<`[`serde_json::Value`]`>, `[`serde_json::Error`]`>` value.
//!
//!    If the response body is empty, this value will be `Ok(None)`.
//!
//!    If the response body is not empty, this value will be an `Ok(Some(value))` or Err(err) from attempting to parse that body as a `serde_json::Value`.
//!    If you expect the response body to be a specific JSON type such as [`apimachinery::pkg::apis::meta::v1::Status`], you can use the `serde_json::Value`
//!    as a [`serde::Deserializer`] like `let status = <Status as Deserialize>::deserialize(value)?;`. On the other hand, if you expect the response body to not be
//!    a JSON value, then ignore the `Err(err)` and parse the raw bytes of the response into the appropriate type.
//!
//! Also see the `get_single_value` and `get_multiple_values` functions in the `k8s-openapi-tests/` directory in the repository for an example of how to use
//! a synchronous client with this style of API.
//!
//! ```rust,no_run
//! // Re-export of the http crate since it's used in the public API
//! use k8s_openapi::http;
//!
//! use k8s_openapi::api::core::v1 as api;
//!
//! # struct Response;
//! # impl Response {
//! #     fn status_code(&self) -> http::StatusCode {
//! #         unimplemented!()
//! #     }
//! #     fn read_into(&self, _buf: &mut [u8]) -> std::io::Result<usize> {
//! #         unimplemented!()
//! #     }
//! # }
//! #
//! // Assume `execute` is some function that takes an `http::Request` and
//! // executes it synchronously or asynchronously to get a response. This is
//! // provided by your HTTP client library.
//! //
//! // Note that the `http::Request` values returned by API operation functions
//! // only have a URL path, query string and request body filled out. That is,
//! // they do *not* have a URL host. So the real `execute` implementation
//! // would first mutate the URL of the request to an absolute URL with
//! // the API server's authority, add authorization headers, etc before
//! // actually executing it.
//! fn execute(req: http::Request<Vec<u8>>) -> Response { unimplemented!(); }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a `http::Request` to list all the pods in the
//!     // "kube-system" namespace.
//!     let (request, response_body) =
//!         api::Pod::list_namespaced_pod("kube-system", Default::default())?;
//!
//!     // Execute the request and get a response.
//!     // If this is an asynchronous operation, you would await
//!     // or otherwise yield to the event loop here.
//!     let response = execute(request);
//!
//!     // Got a status code from executing the request.
//!     let status_code: http::StatusCode = response.status_code();
//!
//!     // Construct the `ResponseBody<ListNamespacedPodResponse>` using the
//!     // constructor returned by the API function.
//!     let mut response_body = response_body(status_code);
//!
//!     // Buffer used for each read from the HTTP response.
//!     let mut buf = Box::new([0u8; 4096]);
//!
//!     let pod_list = loop {
//!         // Read some bytes from the HTTP response into the buffer.
//!         // If this is an asynchronous operation, you would await or
//!         // yield to the event loop here.
//!         let read = response.read_into(&mut *buf)?;
//!
//!         // `buf` now contains some data read from the response. Append it
//!         // to the `ResponseBody` and try to parse it into
//!         // the response type.
//!         response_body.append_slice(&buf[..read]);
//!         let response = response_body.parse();
//!         match response {
//!             // Successful response (HTTP 200 and parsed successfully)
//!             Ok(api::ListNamespacedPodResponse::Ok(pod_list)) =>
//!                 break pod_list,
//!
//!             // Some unexpected response
//!             // (not HTTP 200, but still parsed successfully)
//!             Ok(other) => return Err(format!(
//!                 "expected Ok but got {} {:?}",
//!                 status_code, other).into()),
//!
//!             // Need more response data.
//!             // Read more bytes from the response into the `ResponseBody`
//!             Err(k8s_openapi::ResponseError::NeedMoreData) => continue,
//!
//!             // Some other error, like the response body being
//!             // malformed JSON or invalid UTF-8.
//!             Err(err) => return Err(format!(
//!                 "error: {} {:?}",
//!                 status_code, err).into()),
//!         }
//!     };
//!
//!     for pod in pod_list.items {
//!         println!("{:#?}", pod);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub use chrono;
pub use http;
pub use serde_json;

/// A wrapper around a list of bytes.
///
/// Used in Kubernetes types whose JSON representation uses a base64-encoded string for a list of bytes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    /// The API version of the resource. This is a composite of [`Resource::group`] and [`Resource::version`] (eg `"apiextensions.k8s.io/v1beta1"`)
    /// or just the version for resources without a group (eg `"v1"`).
    ///
    /// This is the string used in the `apiVersion` field of the resource's serialized form.
    fn api_version() -> &'static str where Self: Sized;

    /// The group of the resource, or the empty string if the resource doesn't have a group.
    fn group() -> &'static str where Self: Sized;

    /// The kind of the resource.
    ///
    /// This is the string used in the `kind` field of the resource's serialized form.
    fn kind() -> &'static str where Self: Sized;

    /// The version of the resource.
    fn version() -> &'static str where Self: Sized;
}

/// A trait applied to all Kubernetes resources that have metadata.
pub trait Metadata: Resource {
    /// The type of the metadata object.
    type Ty;

    /// Gets the metadata of this resource value.
    fn metadata(&self) -> Option<&<Self as Metadata>::Ty>;
}

/// Extracts the API version of the given resource value.
///
/// This just forwards to the value's impl of [`Resource::api_version`] but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn api_version<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::api_version()
}

/// Extracts the group of the given resource value.
///
/// This just forwards to the value's impl of [`Resource::group`] but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn group<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::group()
}

/// Extracts the kind of the given resource value.
///
/// This just forwards to the value's impl of [`Resource::kind`] but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn kind<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::kind()
}

/// Extracts the version of the given resource value.
///
/// This just forwards to the value's impl of [`Resource::version`] but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn version<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::version()
}

/// The type of errors returned by the Kubernetes API functions that prepare the HTTP request.
#[derive(Debug)]
pub enum RequestError {
    /// An error from preparing the HTTP request.
    Http(http::Error),

    /// An error while serializing a value into the JSON body of the HTTP request.
    Json(serde_json::Error),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Http(err) => write!(f, "{}", err),
            RequestError::Json(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for RequestError {
    fn description(&self) -> &str {
        match self {
            RequestError::Http(err) => err.description(),
            RequestError::Json(err) => err.description(),
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::Http(err) => Some(err),
            RequestError::Json(err) => Some(err),
        }
    }
}

/// A trait implemented by all response types corresponding to Kubernetes API functions.
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
pub struct ResponseBody<T> {
    /// The HTTP status code of the response.
    pub status_code: http::StatusCode,

    buf: bytes::BytesMut,

    _response: std::marker::PhantomData<fn() -> T>,
}

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
                self.buf.advance(read);
                Ok(result)
            },

            Err(err) => Err(err),
        }
    }

    /// Drop the first `cnt` bytes of this buffer.
    ///
    /// This is useful for skipping over malformed bytes, such as invalid utf-8 sequences when parsing streaming `String` responses
    /// like from [`api::core::v1::Pod::read_namespaced_pod_log`].
    ///
    /// # Panics
    ///
    /// This function panics if `cnt > self.len()`
    pub fn advance(&mut self, cnt: usize) {
        self.buf.advance(cnt)
    }
}

impl<T> std::ops::Deref for ResponseBody<T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.buf
    }
}

/// The type of errors from parsing an HTTP response as one of the Kubernetes API functions' response types.
#[derive(Debug)]
pub enum ResponseError {
    /// An error from deserializing the HTTP response, indicating more data is needed to complete deserialization.
    NeedMoreData,

    /// An error while deserializing the HTTP response as a JSON value, indicating the response is malformed.
    Json(serde_json::Error),

    /// An error while deserializing the HTTP response as a string, indicating that the response data is not UTF-8.
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::NeedMoreData => write!(f, "need more response data"),
            ResponseError::Json(err) => write!(f, "{}", err),
            ResponseError::Utf8(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ResponseError {
    fn description(&self) -> &str {
        match self {
            ResponseError::NeedMoreData => "need more response data",
            ResponseError::Json(err) => err.description(),
            ResponseError::Utf8(err) => err.description(),
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResponseError::NeedMoreData => None,
            ResponseError::Json(err) => Some(err),
            ResponseError::Utf8(err) => Some(err),
        }
    }
}

macro_rules! mods {
    ($($name:ident $name_str:literal)*) => {
        $(
            #[cfg(feature = $name_str)] mod $name;
            #[cfg(feature = $name_str)] pub use self::$name::*;
        )*
    };
}

mods! {
    v1_8 "v1_8"
    v1_9 "v1_9"
    v1_10 "v1_10"
    v1_11 "v1_11"
    v1_12 "v1_12"
    v1_13 "v1_13"
    v1_14 "v1_14"
}

include!(concat!(env!("OUT_DIR"), "/conditional_compilation_macros.rs"));
