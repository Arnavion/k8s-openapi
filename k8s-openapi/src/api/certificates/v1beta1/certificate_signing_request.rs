// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequest

/// Describes a certificate signing request
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CertificateSigningRequest {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The certificate request itself and any additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::api::certificates::v1beta1::CertificateSigningRequestSpec>,

    /// Derived information about the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::api::certificates::v1beta1::CertificateSigningRequestStatus>,
}
