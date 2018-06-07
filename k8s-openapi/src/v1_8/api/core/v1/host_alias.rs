// Generated from definition io.k8s.api.core.v1.HostAlias

/// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HostAlias {
    /// Hostnames for the above IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,

    /// IP address of the host file entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}
