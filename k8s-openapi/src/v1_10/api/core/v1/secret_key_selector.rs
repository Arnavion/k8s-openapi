// Generated from definition io.k8s.api.core.v1.SecretKeySelector

/// SecretKeySelector selects a key of a Secret.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SecretKeySelector {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specify whether the Secret or it's key must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
