// Generated from definition io.k8s.api.admissionregistration.v1alpha1.AdmissionHookClientConfig

/// AdmissionHookClientConfig contains the information to make a TLS connection with the webhook
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AdmissionHookClientConfig {
    /// CABundle is a PEM encoded CA bundle which will be used to validate webhook's server certificate. Required
    #[serde(rename = "caBundle")]
    pub ca_bundle: ::ByteString,

    /// Service is a reference to the service for this webhook. If there is only one port open for the service, that port will be used. If there are multiple ports open, port 443 will be used if it is open, otherwise it is an error. Required
    pub service: ::v1_8::api::admissionregistration::v1alpha1::ServiceReference,
}
