// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIServiceCondition

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIServiceCondition {
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

    /// Status is the status of the condition. Can be True, False, Unknown.
    pub status: String,

    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub type_: String,
}
