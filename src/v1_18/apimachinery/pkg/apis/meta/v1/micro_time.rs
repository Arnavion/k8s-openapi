// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime

/// MicroTime is version of Time with microsecond level precision.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MicroTime(pub crate::chrono::DateTime<crate::chrono::Utc>);

impl<'de> crate::serde::Deserialize<'de> for MicroTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MicroTime;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MicroTime")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(MicroTime(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("MicroTime", Visitor)
    }
}

impl crate::serde::Serialize for MicroTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("MicroTime", &self.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true))
    }
}
