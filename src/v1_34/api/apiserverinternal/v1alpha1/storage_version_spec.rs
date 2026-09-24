// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.StorageVersionSpec

/// StorageVersionSpec is an empty spec.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageVersionSpec(pub crate::serde_json::Value);

impl crate::DeepMerge for StorageVersionSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageVersionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageVersionSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageVersionSpec")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(StorageVersionSpec(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("StorageVersionSpec", Visitor)
    }
}

impl From<crate::serde_json::Value> for StorageVersionSpec {
    fn from(inner: crate::serde_json::Value) -> Self {
        Self(inner)
    }
}

impl crate::serde::Serialize for StorageVersionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("StorageVersionSpec", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageVersionSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apiserverinternal.v1alpha1.StorageVersionSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "StorageVersionSpec is an empty spec.",
            "type": "object",
        })
    }
}
