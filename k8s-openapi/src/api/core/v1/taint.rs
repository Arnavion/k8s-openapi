// Generated from definition io.k8s.api.core.v1.Taint

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Taint {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,

    /// Required. The taint key to be applied to a node.
    pub key: String,

    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    #[serde(rename = "timeAdded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_added: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Required. The taint value corresponding to the taint key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
