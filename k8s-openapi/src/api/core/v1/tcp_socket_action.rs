// Generated from definition io.k8s.api.core.v1.TCPSocketAction

/// TCPSocketAction describes an action based on opening a socket
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TCPSocketAction {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: ::apimachinery::pkg::util::intstr::IntOrString,
}
