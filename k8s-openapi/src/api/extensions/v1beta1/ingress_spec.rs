// Generated from definition io.k8s.api.extensions.v1beta1.IngressSpec

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IngressSpec {
    /// A default backend capable of servicing requests that don't match any rule. At least one of 'backend' or 'rules' must be specified. This field is optional to allow the loadbalancer controller or defaulting logic to specify a global default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<::api::extensions::v1beta1::IngressBackend>,

    /// A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<::api::extensions::v1beta1::IngressRule>>,

    /// TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<::api::extensions::v1beta1::IngressTLS>>,
}
