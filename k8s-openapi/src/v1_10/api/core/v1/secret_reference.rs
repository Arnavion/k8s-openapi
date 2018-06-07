// Generated from definition io.k8s.api.core.v1.SecretReference

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace defines the space within which the secret name must be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
