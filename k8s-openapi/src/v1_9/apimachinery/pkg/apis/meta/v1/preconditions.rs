// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Preconditions

/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Preconditions {
    /// Specifies the target UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
