// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupAllDisruptionMode

/// WorkloadPodGroupAllDisruptionMode indicates that all pods in the group must be disrupted together.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadPodGroupAllDisruptionMode(pub crate::serde_json::Value);

impl crate::DeepMerge for WorkloadPodGroupAllDisruptionMode {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadPodGroupAllDisruptionMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadPodGroupAllDisruptionMode;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadPodGroupAllDisruptionMode")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(WorkloadPodGroupAllDisruptionMode(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("WorkloadPodGroupAllDisruptionMode", Visitor)
    }
}

impl From<crate::serde_json::Value> for WorkloadPodGroupAllDisruptionMode {
    fn from(inner: crate::serde_json::Value) -> Self {
        Self(inner)
    }
}

impl crate::serde::Serialize for WorkloadPodGroupAllDisruptionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("WorkloadPodGroupAllDisruptionMode", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadPodGroupAllDisruptionMode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupAllDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadPodGroupAllDisruptionMode indicates that all pods in the group must be disrupted together.",
            "type": "object",
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for WorkloadPodGroupAllDisruptionMode {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupAllDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("WorkloadPodGroupAllDisruptionMode indicates that all pods in the group must be disrupted together.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            ..Default::default()
        })
    }
}
