// Generated from definition io.k8s.api.core.v1.PodCondition

/// PodCondition contains details for the current condition of this pod.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodCondition {
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

    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    pub status: String,

    /// Type is the type of the condition. Currently only Ready. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    #[serde(rename = "type")]
    pub type_: String,
}
