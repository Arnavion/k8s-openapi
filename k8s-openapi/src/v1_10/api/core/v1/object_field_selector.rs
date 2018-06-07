// Generated from definition io.k8s.api.core.v1.ObjectFieldSelector

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}
