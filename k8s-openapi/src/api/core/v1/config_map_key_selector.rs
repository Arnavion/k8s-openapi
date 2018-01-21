// Generated from definition io.k8s.api.core.v1.ConfigMapKeySelector

/// Selects a key from a ConfigMap.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigMapKeySelector {
    /// The key to select.
    pub key: String,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specify whether the ConfigMap or it's key must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
