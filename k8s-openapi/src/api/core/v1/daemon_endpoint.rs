// Generated from definition io.k8s.api.core.v1.DaemonEndpoint

/// DaemonEndpoint contains information about a single Daemon endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DaemonEndpoint {
    /// Port number of the given endpoint.
    #[serde(rename = "Port")]
    pub port: i32,
}
