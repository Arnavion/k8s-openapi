// Generated from definition io.k8s.api.extensions.v1beta1.DaemonSetUpdateStrategy

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DaemonSetUpdateStrategy {
    /// Rolling update config params. Present only if type = "RollingUpdate".
    #[serde(rename = "rollingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<::api::extensions::v1beta1::RollingUpdateDaemonSet>,

    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is OnDelete.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
