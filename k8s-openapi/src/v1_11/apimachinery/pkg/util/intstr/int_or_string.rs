// Generated from definition io.k8s.apimachinery.pkg.util.intstr.IntOrString

/// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
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

impl<'de> ::serde::Deserialize<'de> for IntOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntOrString;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "enum IntOrString")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: ::serde::de::Error {
                Ok(IntOrString::Int(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: ::serde::de::Error {
                if v < ::std::i32::MIN as i64 || v > ::std::i32::MAX as i64 {
                    return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Signed(v), &"a 32-bit integer"));
                }

                Ok(IntOrString::Int(v as i32))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: ::serde::de::Error {
                if v > ::std::i32::MAX as u64 {
                    return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"));
                }

                Ok(IntOrString::Int(v as i32))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                self.visit_string(v.to_string())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
                Ok(IntOrString::String(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl ::serde::Serialize for IntOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        match self {
            IntOrString::Int(i) => i.serialize(serializer),
            IntOrString::String(s) => s.serialize(serializer),
        }
    }
}
