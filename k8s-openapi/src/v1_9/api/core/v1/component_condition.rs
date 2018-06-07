// Generated from definition io.k8s.api.core.v1.ComponentCondition

/// Information about the condition of a component.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ComponentCondition {
    /// Condition error code for a component. For example, a health check error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Message about the condition for a component. For example, information about a health check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Status of the condition for a component. Valid values for "Healthy": "True", "False", or "Unknown".
    pub status: String,

    /// Type of condition for a component. Valid value: "Healthy"
    #[serde(rename = "type")]
    pub type_: String,
}
