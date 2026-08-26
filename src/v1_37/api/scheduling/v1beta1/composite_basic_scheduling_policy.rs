// Generated from definition io.k8s.api.scheduling.v1beta1.CompositeBasicSchedulingPolicy

/// CompositeBasicSchedulingPolicy indicates that the groups belonging to the composite group should be scheduled independently.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompositeBasicSchedulingPolicy(pub crate::serde_json::Value);

impl crate::DeepMerge for CompositeBasicSchedulingPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CompositeBasicSchedulingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CompositeBasicSchedulingPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CompositeBasicSchedulingPolicy")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(CompositeBasicSchedulingPolicy(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("CompositeBasicSchedulingPolicy", Visitor)
    }
}

impl From<crate::serde_json::Value> for CompositeBasicSchedulingPolicy {
    fn from(inner: crate::serde_json::Value) -> Self {
        Self(inner)
    }
}

impl crate::serde::Serialize for CompositeBasicSchedulingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("CompositeBasicSchedulingPolicy", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CompositeBasicSchedulingPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1beta1.CompositeBasicSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CompositeBasicSchedulingPolicy indicates that the groups belonging to the composite group should be scheduled independently.",
            "type": "object",
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for CompositeBasicSchedulingPolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1beta1.CompositeBasicSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("CompositeBasicSchedulingPolicy indicates that the groups belonging to the composite group should be scheduled independently.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            ..Default::default()
        })
    }
}
