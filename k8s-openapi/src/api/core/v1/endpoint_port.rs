// Generated from definition io.k8s.api.core.v1.EndpointPort

/// EndpointPort is a tuple that describes a single port.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EndpointPort {
    /// The name of this port (corresponds to ServicePort.Name). Must be a DNS_LABEL. Optional only if one port is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The port number of the endpoint.
    pub port: i32,

    /// The IP protocol for this port. Must be UDP or TCP. Default is TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}
