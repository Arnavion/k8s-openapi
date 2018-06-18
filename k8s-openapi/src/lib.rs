extern crate base64;
extern crate chrono;
extern crate http;
#[cfg(feature = "reqwest")] extern crate reqwest;
extern crate serde;
extern crate serde_json;
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

/// An interface for a synchronous Kubernetes API client.
pub trait Client {
    /// The type of HTTP response returned by this client.
    type Response: Response;

    /// The type of errors returned by this client.
    type Error;

    /// The base URL to which all API paths will be appended.
    fn base_url(&self) -> &url::Url;

    /// Executes an HTTP DELETE request against the given URL.
    fn delete(&self, url: url::Url) -> Result<Self::Response, Self::Error>;

    /// Executes an HTTP GET request against the given URL.
    fn get(&self, url: url::Url) -> Result<Self::Response, Self::Error>;

    /// Executes an HTTP PATCH request against the given URL with the given body serialized to JSON.
    fn patch<B>(&self, url: url::Url, body: &B) -> Result<Self::Response, Self::Error> where B: serde::Serialize;

    /// Executes an HTTP POST request against the given URL with the given body serialized to JSON.
    fn post<B>(&self, url: url::Url, body: &B) -> Result<Self::Response, Self::Error> where B: serde::Serialize;

    /// Executes an HTTP PUT request against the given URL with the given body serialized to JSON.
    fn put<B>(&self, url: url::Url, body: &B) -> Result<Self::Response, Self::Error> where B: serde::Serialize;
}

/// An interface of an HTTP response returned by [`Client`]
pub trait Response: std::io::Read {
    /// Gets the HTTP status code of this response.
    fn status_code(&self) -> http::StatusCode;
}

#[cfg(feature = "reqwest")]
impl Response for reqwest::Response {
    fn status_code(&self) -> http::StatusCode {
        let status_code = self.status();
        match http::StatusCode::from_u16(status_code.as_u16()) {
            Ok(status_code) => status_code,
            Err(_) => panic!("could not convert {} to http::StatusCode", status_code),
        }
    }
}

/// The type of errors returned by the Kubernetes API functions.
#[derive(Debug)]
pub enum Error<CE> {
    /// An error from the HTTP client.
    Client(CE),

    /// An error while reading the HTTP response.
    IO(std::io::Error),

    /// An error while deserializing the HTTP response as a JSON value.
    JSON(serde_json::Error),

    /// An error while constructing the HTTP request URL.
    URL(url::ParseError),
}

impl<CE> std::fmt::Display for Error<CE> where CE: std::error::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Client(err) => write!(f, "{}", err),
            Error::IO(err) => write!(f, "{}", err),
            Error::JSON(err) => write!(f, "{}", err),
            Error::URL(err) => write!(f, "{}", err),
        }
    }
}

impl<CE> std::error::Error for Error<CE> where CE: std::error::Error {
    fn description(&self) -> &str {
        match self {
            Error::Client(err) => std::error::Error::description(err),
            Error::IO(err) => std::error::Error::description(err),
            Error::JSON(err) => std::error::Error::description(err),
            Error::URL(err) => std::error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            Error::Client(err) => Some(err),
            Error::IO(err) => Some(err),
            Error::JSON(err) => Some(err),
            Error::URL(err) => Some(err),
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
