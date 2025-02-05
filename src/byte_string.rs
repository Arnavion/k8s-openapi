/// A wrapper around a list of bytes.
///
/// Used in Kubernetes types whose JSON representation uses a base64-encoded string for a list of bytes.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteString(pub std::vec::Vec<u8>);

impl<'de> serde::Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ByteString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a base64-encoded string")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ByteString(std::vec![]))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                deserializer.deserialize_str(self)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                let v = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, v).map_err(serde::de::Error::custom)?;
                Ok(ByteString(v))
            }
        }

        deserializer.deserialize_option(Visitor)
    }
}

impl serde::Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let s = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.0);
        s.serialize(serializer)
    }
}
