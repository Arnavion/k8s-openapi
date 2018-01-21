// Generated from definition io.k8s.api.extensions.v1beta1.IngressStatus

/// IngressStatus describe the current state of the Ingress.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IngressStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    #[serde(rename = "loadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<::api::core::v1::LoadBalancerStatus>,
}
