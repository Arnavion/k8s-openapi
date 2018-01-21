// Generated from definition io.k8s.api.apps.v1beta2.DaemonSetUpdateStrategy

/// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DaemonSetUpdateStrategy {
    /// Rolling update config params. Present only if type = "RollingUpdate".
    #[serde(rename = "rollingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<::api::apps::v1beta2::RollingUpdateDaemonSet>,

    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
