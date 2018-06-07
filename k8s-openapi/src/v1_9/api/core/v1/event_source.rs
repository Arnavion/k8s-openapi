// Generated from definition io.k8s.api.core.v1.EventSource

/// EventSource contains information for an event.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EventSource {
    /// Component from which the event is generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,

    /// Node name on which the event is generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}
