//! Bindings for the Kubernetes client API, generated from the OpenAPI spec.
//!
//! Each supported version of Kubernetes is represented by one top-level module (like `::v1_9`) and is enabled by a crate feature of the same name (like `v1_9`).
//!
//!
//! # Examples
//!
//! ## Resources
//!
//! ```rust
//! #[macro_use] extern crate k8s_openapi;
//!
//! k8s_if_1_7! {
//!     use k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
//! }
//! k8s_if_1_8! {
//!     use k8s_openapi::v1_8::api::core::v1 as api;
//! }
//! k8s_if_1_9! {
//!     use k8s_openapi::v1_9::api::core::v1 as api;
//! }
//! k8s_if_1_10! {
//!     use k8s_openapi::v1_10::api::core::v1 as api;
//! }
//! k8s_if_1_11! {
//!     use k8s_openapi::v1_11::api::core::v1 as api;
//! }
//! k8s_if_1_12! {
//!     use k8s_openapi::v1_12::api::core::v1 as api;
//! }
//!
//! fn main() {
//!     let pod_spec: api::PodSpec = Default::default();
//!     println!("{:#?}", pod_spec);
//! }
//! ```
//!
//! ## Client API
//!
//! ```rust,no_run
//! #[macro_use] extern crate k8s_openapi;
//!
//! // Re-export of the http crate since it's used in the public API
//! use k8s_openapi::http;
//!
//! k8s_if_1_7! {
//!     use k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
//! }
//! k8s_if_1_8! {
//!     use k8s_openapi::v1_8::api::core::v1 as api;
//! }
//! k8s_if_1_9! {
//!     use k8s_openapi::v1_9::api::core::v1 as api;
//! }
//! k8s_if_1_10! {
//!     use k8s_openapi::v1_10::api::core::v1 as api;
//! }
//! k8s_if_1_11! {
//!     use k8s_openapi::v1_11::api::core::v1 as api;
//! }
//! k8s_if_1_12! {
//!     use k8s_openapi::v1_12::api::core::v1 as api;
//! }
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
//! // `execute` is some function that takes an `http::Request` and executes it
//! // synchronously or asynchronously to get a response.
//! // Among other things, it will need to change the URL of the request to an
//! // absolute URL with the API server's authority.
//! fn execute(req: http::Request<Vec<u8>>) -> Response { unimplemented!(); }
//!
//! fn main() -> Result<(), Box<std::error::Error>> {
//!     // Create a `http::Request` to list all the pods in the
//!     // "kube-system" namespace.
//!     k8s_if_le_1_7! {
//!         let request = api::Pod::list_core_v1_namespaced_pod(
//!             "kube-system",
//!             None, None, None, None, None, None, None)?;
//!     }
//!     k8s_if_ge_1_8! {
//!         let request = api::Pod::list_core_v1_namespaced_pod(
//!             "kube-system",
//!             None, None, None, None, None, None, None, None, None)?;
//!     }
//!
//!     // Execute the request and get a response.
//!     // If this is an asynchronous operation, you would await
//!     // or otherwise defer here.
//!     let response = execute(request);
//! 
//!     // Got a status code from executing the request.
//!     let status_code: http::StatusCode = response.status_code();
//!
//!     // Construct a `ResponseBody` to accumulate the bytes received from
//!     // the HTTP response.
//!     //
//!     // It is not *necessary* to use this type. It's only a helper to
//!     // provide a convenient byte buffer that can be written to at the end
//!     // and consumed from the front.
//!     //
//!     // You can instead use any buffer type that can be converted to
//!     // a `&[u8]`.
//!     let mut response_body = k8s_openapi::ResponseBody::new(status_code);
//!
//!     // Buffer used for each read from the HTTP response.
//!     let mut buf = Box::new([0u8; 4096]);
//!
//!     let pod_list = loop {
//!         // Read some bytes from the HTTP response into the buffer.
//!         // If this is an asynchronous operation, you would await or
//!         // otherwise defer here.
//!         let read = response.read_into(&mut *buf)?;
//!
//!         // `buf` now contains some data read from the response. Append it
//!         // to the `ResponseBody` and try to parse it into
//!         // the response type.
//!         //
//!         // For `Pod::list_core_v1_namespaced_pod` this is the
//!         // `ListCoreV1NamespacedPodResponse` type.
//!         //
//!         // `ResponseBody::append_slice_and_parse` internally calls
//!         // `Response::try_from_parts` for the response type. So you would
//!         // call that function directly if you were not using `ResponseBody`
//!         let response = response_body.append_slice_and_parse(&buf[..read]);
//!         match response {
//!             // Successful response (HTTP 200 and parsed successfully)
//!             Ok(api::ListCoreV1NamespacedPodResponse::Ok(pod_list)) =>
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
//!
//! Since `Response::try_from_parts` is implemented in terms of a status code and a byte buffer for the response body, it is independent of the method
//! of *actually executing* the HTTP request. This means you can use a synchronous client like `reqwest`, an asynchronous client like `hyper`,
//! a mock client that returns bytes read from a test file, or anything else you want.
//!
//! See the `get_single_value` and `get_multiple_values` functions in the `k8s-openapi-tests/` directory in the repository for an example of how to use
//! a synchronous client with this style of API.

extern crate base64;
extern crate bytes;
pub extern crate chrono;
pub extern crate http;
extern crate serde;
pub extern crate serde_json;
extern crate url;

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

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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

/// The type of errors returned by the Kubernetes API functions that prepare the HTTP request.
#[derive(Debug)]
pub enum RequestError {
    /// An error from preparing the HTTP request.
    Http(http::Error),

    /// An error while serializing a value into the JSON body of the HTTP request.
    Json(serde_json::Error),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            RequestError::Http(err) => Some(err),
            RequestError::Json(err) => Some(err),
        }
    }
}

/// A trait implemented by all response types corresponding to Kubernetes API functions.
pub trait Response: Sized {
    /// Tries to parse the response from the given status code and response body.
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ResponseError>;
}

/// A helper that holds a growable buffer that can be parsed into a Kubernetes API function's response.
pub struct ResponseBody {
    /// The HTTP status code of the response.
    pub status_code: http::StatusCode,

    buf: bytes::BytesMut,
}

impl ResponseBody {
    /// Construct a value for a response that has the specified HTTP status code.
    pub fn new(status_code: http::StatusCode) -> Self {
        ResponseBody {
            status_code,
            buf: Default::default(),
        }
    }

    /// Append a slice of data from the HTTP response to this buffer.
    pub fn append_slice(&mut self, buf: &[u8]) {
        self.buf.extend_from_slice(buf);
    }

    /// Append a slice of data from the HTTP response, and try to parse all the data buffered so far into a response type.
    pub fn append_slice_and_parse<T>(&mut self, buf: &[u8]) -> Result<T, ResponseError> where T: Response {
        self.append_slice(buf);

        match T::try_from_parts(self.status_code, &*self.buf) {
            Ok((result, read)) => {
                self.buf.advance(read);
                Ok(result)
            },

            Err(err) => Err(err),
        }
    }
}

impl std::ops::Deref for ResponseBody {
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            ResponseError::NeedMoreData => None,
            ResponseError::Json(err) => Some(err),
            ResponseError::Utf8(err) => Some(err),
        }
    }
}

#[cfg(feature = "v1_7")]
pub mod v1_7;

#[cfg(feature = "v1_8")]
pub mod v1_8;

#[cfg(feature = "v1_9")]
pub mod v1_9;

#[cfg(feature = "v1_10")]
pub mod v1_10;

#[cfg(feature = "v1_11")]
pub mod v1_11;

#[cfg(feature = "v1_12")]
pub mod v1_12;

include!(concat!(env!("OUT_DIR"), "/conditional_compilation_macros.rs"));
