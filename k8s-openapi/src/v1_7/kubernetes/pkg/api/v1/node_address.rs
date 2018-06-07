// Generated from definition io.k8s.kubernetes.pkg.api.v1.NodeAddress

/// NodeAddress contains information for the node's address.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeAddress {
    /// The node address.
    pub address: String,

    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    #[serde(rename = "type")]
    pub type_: String,
}
