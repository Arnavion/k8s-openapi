// Generated from definition io.k8s.api.apps.v1.DeploymentCondition

/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeploymentCondition {
    /// Last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// The last time this condition was updated.
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// Type of deployment condition.
    #[serde(rename = "type")]
    pub type_: String,
}
