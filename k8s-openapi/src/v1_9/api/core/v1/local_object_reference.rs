// Generated from definition io.k8s.api.core.v1.LocalObjectReference

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LocalObjectReference {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
