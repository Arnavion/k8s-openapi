// Generated from definition io.k8s.api.core.v1.Probe

/// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Probe {
    /// One and only one of the following should be specified. Exec specifies the action to take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<::api::core::v1::ExecAction>,

    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,

    /// HTTPGet specifies the http request to perform.
    #[serde(rename = "httpGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<::api::core::v1::HTTPGetAction>,

    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename = "initialDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,

    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,

    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.
    #[serde(rename = "successThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,

    /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
    #[serde(rename = "tcpSocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<::api::core::v1::TCPSocketAction>,

    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename = "timeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
