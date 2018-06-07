// Generated from definition io.k8s.api.core.v1.Capabilities

/// Adds and removes POSIX capabilities from running containers.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Capabilities {
    /// Added capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,

    /// Removed capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}
