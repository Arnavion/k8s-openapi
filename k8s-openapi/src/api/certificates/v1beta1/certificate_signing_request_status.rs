// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestStatus

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CertificateSigningRequestStatus {
    /// If request was approved, the controller will place the issued certificate here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<::ByteString>,

    /// Conditions applied to the request, such as approval or denial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::certificates::v1beta1::CertificateSigningRequestCondition>>,
}
