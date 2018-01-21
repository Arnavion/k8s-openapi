// Generated from definition io.k8s.api.core.v1.HTTPGetAction

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HTTPGetAction {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<Vec<::api::core::v1::HTTPHeader>>,

    /// Path to access on the HTTP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: ::apimachinery::pkg::util::intstr::IntOrString,

    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
