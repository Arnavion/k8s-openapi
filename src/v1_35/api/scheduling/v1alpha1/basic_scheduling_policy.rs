// Generated from definition io.k8s.api.scheduling.v1alpha1.BasicSchedulingPolicy

/// BasicSchedulingPolicy indicates that standard Kubernetes scheduling behavior should be used.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BasicSchedulingPolicy(pub crate::serde_json::Value);

impl crate::DeepMerge for BasicSchedulingPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for BasicSchedulingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = BasicSchedulingPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("BasicSchedulingPolicy")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(BasicSchedulingPolicy(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("BasicSchedulingPolicy", Visitor)
    }
}

impl crate::serde::Serialize for BasicSchedulingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("BasicSchedulingPolicy", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BasicSchedulingPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha1.BasicSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "BasicSchedulingPolicy indicates that standard Kubernetes scheduling behavior should be used.",
            "type": "object",
        })
    }
}
