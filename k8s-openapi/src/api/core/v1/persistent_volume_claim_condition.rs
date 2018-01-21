// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimCondition

/// PersistentVolumeClaimCondition contails details about state of pvc
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PersistentVolumeClaimCondition {
    /// Last time we probed the condition.
    #[serde(rename = "lastProbeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Human-readable message indicating details about last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "ResizeStarted" that means the underlying persistent volume is being resized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    pub status: String,

    #[serde(rename = "type")]
    pub type_: String,
}
