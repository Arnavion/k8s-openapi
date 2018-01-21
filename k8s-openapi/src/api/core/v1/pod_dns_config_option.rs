// Generated from definition io.k8s.api.core.v1.PodDNSConfigOption

/// PodDNSConfigOption defines DNS resolver options of a pod.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodDNSConfigOption {
    /// Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
