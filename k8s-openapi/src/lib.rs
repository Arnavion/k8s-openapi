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

/// A value that may be a 32-bit integer or a string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntOrString {
    Int(i32),
    String(String),
}

impl Default for IntOrString {
    fn default() -> Self {
        IntOrString::Int(0)
    }
}

impl<'de> serde::Deserialize<'de> for IntOrString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IntOrString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a 32-bit integer or a string")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(IntOrString::Int(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {
                if v < std::i32::MIN as i64 || v > std::i32::MAX as i64 {
                    return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &"a 32-bit integer"));
                }

                Ok(IntOrString::Int(v as i32))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
                if v > std::i32::MAX as u64 {
                    return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"));
                }

                Ok(IntOrString::Int(v as i32))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                self.visit_string(v.to_string())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(IntOrString::String(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for IntOrString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            IntOrString::Int(i) => i.serialize(serializer),
            IntOrString::String(s) => s.serialize(serializer),
        }
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

include!(concat!(env!("OUT_DIR"), "/conditional_compilation_macros.rs"));
