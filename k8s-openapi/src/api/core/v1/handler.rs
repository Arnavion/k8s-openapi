// Generated from definition io.k8s.api.core.v1.Handler

/// Handler defines a specific action that should be taken
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Handler {
    /// One and only one of the following should be specified. Exec specifies the action to take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<::api::core::v1::ExecAction>,

    /// HTTPGet specifies the http request to perform.
    #[serde(rename = "httpGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<::api::core::v1::HTTPGetAction>,

    /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
    #[serde(rename = "tcpSocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<::api::core::v1::TCPSocketAction>,
}
