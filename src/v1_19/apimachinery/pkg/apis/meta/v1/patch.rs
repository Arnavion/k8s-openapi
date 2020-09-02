// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Patch

/// Patch is provided to give a concrete name and type to the Kubernetes PATCH request body.
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    Json(Vec<serde_json::Value>),
    Merge(serde_json::Value),
    StrategicMerge(serde_json::Value),
}

impl serde::Serialize for Patch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            Patch::Json(patch) => serializer.serialize_newtype_struct("Patch", patch),
            Patch::Merge(patch) |
            Patch::StrategicMerge(patch) => serializer.serialize_newtype_struct("Patch", patch),
        }
    }
}
