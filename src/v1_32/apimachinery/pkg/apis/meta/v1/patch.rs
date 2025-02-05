// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Patch

/// Patch is provided to give a concrete name and type to the Kubernetes PATCH request body.
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    Json(std::vec::Vec<crate::serde_json::Value>),
    Merge(crate::serde_json::Value),
    StrategicMerge(crate::serde_json::Value),
}

impl crate::serde::Serialize for Patch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        match self {
            Patch::Json(patch) => serializer.serialize_newtype_struct("Patch", patch),
            Patch::Merge(patch) |
            Patch::StrategicMerge(patch) => serializer.serialize_newtype_struct("Patch", patch),
        }
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Patch {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.Patch".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Patch is provided to give a concrete name and type to the Kubernetes PATCH request body.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            ..Default::default()
        })
    }
}
