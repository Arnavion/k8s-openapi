// Generated from definition io.k8s.api.networking.v1.NetworkPolicyPort

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NetworkPolicyPort {
    /// The port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<::apimachinery::pkg::util::intstr::IntOrString>,

    /// The protocol (TCP or UDP) which traffic must match. If not specified, this field defaults to TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}
