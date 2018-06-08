extern crate base64;
extern crate chrono;
extern crate serde;

/// A wrapper around a list of bytes.
///
/// Used in Kubernetes types whose JSON representation uses a base64-encoded string for a list of bytes.
#[derive(Debug, Default)]
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
#[derive(Debug)]
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

#[cfg(feature = "v1_7")]
pub mod v1_7;

#[cfg(feature = "v1_8")]
pub mod v1_8;

#[cfg(feature = "v1_9")]
pub mod v1_9;

#[cfg(feature = "v1_10")]
pub mod v1_10;
