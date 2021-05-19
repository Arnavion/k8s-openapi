// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.StorageVersionSpec

/// StorageVersionSpec is an empty spec.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageVersionSpec(pub crate::serde_json::Value);

impl<'de> crate::serde::Deserialize<'de> for StorageVersionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageVersionSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StorageVersionSpec")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(StorageVersionSpec(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("StorageVersionSpec", Visitor)
    }
}

impl crate::serde::Serialize for StorageVersionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("StorageVersionSpec", &self.0)
    }
}
