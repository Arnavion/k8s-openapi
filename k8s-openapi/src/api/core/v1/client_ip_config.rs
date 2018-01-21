// Generated from definition io.k8s.api.core.v1.ClientIPConfig

/// ClientIPConfig represents the configurations of Client IP based session affinity.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ClientIPConfig {
    /// timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be >0 && <=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).
    #[serde(rename = "timeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
