// Generated from definition io.k8s.api.core.v1.ServiceStatus

/// ServiceStatus represents the current status of a service.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServiceStatus {
    /// LoadBalancer contains the current status of the load-balancer, if one is present.
    #[serde(rename = "loadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<::api::core::v1::LoadBalancerStatus>,
}
