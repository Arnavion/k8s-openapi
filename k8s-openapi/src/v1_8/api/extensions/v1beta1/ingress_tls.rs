// Generated from definition io.k8s.api.extensions.v1beta1.IngressTLS

/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IngressTLS {
    /// Hosts are a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,

    /// SecretName is the name of the secret used to terminate SSL traffic on 443. Field is left optional to allow SSL routing based on SNI hostname alone. If the SNI host in a listener conflicts with the "Host" header field used by an IngressRule, the SNI host is used for termination and value of the Host header is used for routing.
    #[serde(rename = "secretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
