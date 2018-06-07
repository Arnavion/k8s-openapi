// Generated from definition io.k8s.api.core.v1.ContainerPort

/// ContainerPort represents a network port in a single container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerPort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
    #[serde(rename = "containerPort")]
    pub container_port: i32,

    /// What host IP to bind the external port to.
    #[serde(rename = "hostIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,

    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[serde(rename = "hostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,

    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Protocol for port. Must be UDP or TCP. Defaults to "TCP".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}
