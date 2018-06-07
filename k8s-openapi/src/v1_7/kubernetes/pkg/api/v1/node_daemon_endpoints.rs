// Generated from definition io.k8s.kubernetes.pkg.api.v1.NodeDaemonEndpoints

/// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeDaemonEndpoints {
    /// Endpoint on which Kubelet is listening.
    #[serde(rename = "kubeletEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_endpoint: Option<::v1_7::kubernetes::pkg::api::v1::DaemonEndpoint>,
}
