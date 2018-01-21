// Generated from definition io.k8s.api.core.v1.SessionAffinityConfig

/// SessionAffinityConfig represents the configurations of session affinity.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SessionAffinityConfig {
    /// clientIP contains the configurations of Client IP based session affinity.
    #[serde(rename = "clientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<::api::core::v1::ClientIPConfig>,
}
