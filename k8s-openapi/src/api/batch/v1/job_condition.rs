// Generated from definition io.k8s.api.batch.v1.JobCondition

/// JobCondition describes current state of a job.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JobCondition {
    /// Last time the condition was checked.
    #[serde(rename = "lastProbeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

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

    /// Type of job condition, Complete or Failed.
    #[serde(rename = "type")]
    pub type_: String,
}
