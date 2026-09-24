// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupBasicSchedulingPolicy

/// WorkloadPodGroupBasicSchedulingPolicy indicates standard Kubernetes scheduling behavior.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadPodGroupBasicSchedulingPolicy(pub crate::serde_json::Value);

impl crate::DeepMerge for WorkloadPodGroupBasicSchedulingPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadPodGroupBasicSchedulingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadPodGroupBasicSchedulingPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadPodGroupBasicSchedulingPolicy")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(WorkloadPodGroupBasicSchedulingPolicy(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("WorkloadPodGroupBasicSchedulingPolicy", Visitor)
    }
}

impl From<crate::serde_json::Value> for WorkloadPodGroupBasicSchedulingPolicy {
    fn from(inner: crate::serde_json::Value) -> Self {
        Self(inner)
    }
}

impl crate::serde::Serialize for WorkloadPodGroupBasicSchedulingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("WorkloadPodGroupBasicSchedulingPolicy", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadPodGroupBasicSchedulingPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupBasicSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadPodGroupBasicSchedulingPolicy indicates standard Kubernetes scheduling behavior.",
            "type": "object",
        })
    }
}
