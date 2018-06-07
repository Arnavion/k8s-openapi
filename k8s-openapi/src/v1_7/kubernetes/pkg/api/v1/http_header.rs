// Generated from definition io.k8s.kubernetes.pkg.api.v1.HTTPHeader

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HTTPHeader {
    /// The header field name
    pub name: String,

    /// The header field value
    pub value: String,
}
