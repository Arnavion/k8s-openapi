// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerCondition

/// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HorizontalPodAutoscalerCondition {
    /// lastTransitionTime is the last time the condition transitioned from one status to another
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// message is a human-readable explanation containing details about the transition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// reason is the reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// status is the status of the condition (True, False, Unknown)
    pub status: String,

    /// type describes the current condition
    #[serde(rename = "type")]
    pub type_: String,
}
