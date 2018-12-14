// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime

/// MicroTime is version of Time with microsecond level precision.
#[derive(Clone, Debug, PartialEq)]
pub struct MicroTime(pub ::chrono::DateTime<::chrono::Utc>);

impl<'de> ::serde::Deserialize<'de> for MicroTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MicroTime;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "MicroTime")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: ::serde::Deserializer<'de> {
                Ok(MicroTime(::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("MicroTime", Visitor)
    }
}

impl ::serde::Serialize for MicroTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_newtype_struct("MicroTime", &self.0)
    }
}
