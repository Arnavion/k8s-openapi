// Generated from definition io.k8s.api.core.v1.EndpointAddress

/// EndpointAddress is a tuple that describes single IP address.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EndpointAddress {
    /// The Hostname of this endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// The IP of this endpoint. May not be loopback (127.0.0.0/8), link-local (169.254.0.0/16), or link-local multicast ((224.0.0.0/24). IPv6 is also accepted but not fully supported on all platforms. Also, certain kubernetes components, like kube-proxy, are not IPv6 ready.
    pub ip: String,

    /// Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
    #[serde(rename = "nodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,

    /// Reference to object providing the endpoint.
    #[serde(rename = "targetRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<::api::core::v1::ObjectReference>,
}
