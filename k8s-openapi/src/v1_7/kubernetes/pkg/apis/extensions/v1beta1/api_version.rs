// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.APIVersion

/// An APIVersion represents a single concrete version of an object model.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIVersion {
    /// Name of this version (e.g. 'v1').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
