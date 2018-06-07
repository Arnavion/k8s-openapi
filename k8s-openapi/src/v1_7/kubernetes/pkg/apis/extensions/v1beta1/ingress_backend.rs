// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.IngressBackend

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IngressBackend {
    /// Specifies the name of the referenced service.
    #[serde(rename = "serviceName")]
    pub service_name: String,

    /// Specifies the port of the referenced service.
    #[serde(rename = "servicePort")]
    pub service_port: ::v1_7::apimachinery::pkg::util::intstr::IntOrString,
}
