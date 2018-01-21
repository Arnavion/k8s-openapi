// Generated from definition io.k8s.api.core.v1.EndpointSubset

/// EndpointSubset is a group of addresses with a common set of ports. The expanded set of endpoints is the Cartesian product of Addresses x Ports. For example, given:
///   {
///     Addresses: [{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}],
///     Ports:     [{"name": "a", "port": 8675}, {"name": "b", "port": 309}]
///   }
/// The resulting set of endpoints can be viewed as:
///     a: [ 10.10.1.1:8675, 10.10.2.2:8675 ],
///     b: [ 10.10.1.1:309, 10.10.2.2:309 ]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EndpointSubset {
    /// IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<::api::core::v1::EndpointAddress>>,

    /// IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.
    #[serde(rename = "notReadyAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ready_addresses: Option<Vec<::api::core::v1::EndpointAddress>>,

    /// Port numbers available on the related IP addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<::api::core::v1::EndpointPort>>,
}
