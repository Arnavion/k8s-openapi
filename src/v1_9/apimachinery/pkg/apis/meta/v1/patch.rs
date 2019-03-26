// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Patch

/// Patch is provided to give a concrete name and type to the Kubernetes PATCH request body.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Patch(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for Patch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Patch;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Patch")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(Patch(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("Patch", Visitor)
    }
}

impl serde::Serialize for Patch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("Patch", &self.0)
    }
}
