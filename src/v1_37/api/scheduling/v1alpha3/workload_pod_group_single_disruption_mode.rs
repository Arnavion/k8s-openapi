// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupSingleDisruptionMode

/// WorkloadPodGroupSingleDisruptionMode indicates that individual pods can be disrupted independently.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadPodGroupSingleDisruptionMode(pub crate::serde_json::Value);

impl crate::DeepMerge for WorkloadPodGroupSingleDisruptionMode {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadPodGroupSingleDisruptionMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadPodGroupSingleDisruptionMode;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadPodGroupSingleDisruptionMode")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(WorkloadPodGroupSingleDisruptionMode(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("WorkloadPodGroupSingleDisruptionMode", Visitor)
    }
}

impl From<crate::serde_json::Value> for WorkloadPodGroupSingleDisruptionMode {
    fn from(inner: crate::serde_json::Value) -> Self {
        Self(inner)
    }
}

impl crate::serde::Serialize for WorkloadPodGroupSingleDisruptionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("WorkloadPodGroupSingleDisruptionMode", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadPodGroupSingleDisruptionMode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupSingleDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadPodGroupSingleDisruptionMode indicates that individual pods can be disrupted independently.",
            "type": "object",
        })
    }
}
