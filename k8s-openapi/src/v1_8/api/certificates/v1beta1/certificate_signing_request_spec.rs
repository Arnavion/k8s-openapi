// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestSpec

/// This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CertificateSigningRequestSpec {
    /// Extra information about the requesting user. See user.Info interface for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<::std::collections::BTreeMap<String, Vec<String>>>,

    /// Group information about the requesting user. See user.Info interface for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,

    /// Base64-encoded PKCS#10 CSR data
    pub request: ::ByteString,

    /// UID information about the requesting user. See user.Info interface for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    ///      https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<String>>,

    /// Information about the requesting user. See user.Info interface for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
