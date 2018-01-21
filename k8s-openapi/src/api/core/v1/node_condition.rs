// Generated from definition io.k8s.api.core.v1.NodeCondition

/// NodeCondition contains condition information for a node.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeCondition {
    /// Last time we got an update on a given condition.
    #[serde(rename = "lastHeartbeatTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Last time the condition transit from one status to another.
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Human readable message indicating details about last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// (brief) reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// Type of node condition.
    #[serde(rename = "type")]
    pub type_: String,
}
