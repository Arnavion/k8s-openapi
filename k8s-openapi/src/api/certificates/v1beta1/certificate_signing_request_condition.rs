// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestCondition

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CertificateSigningRequestCondition {
    /// timestamp for the last update to this condition
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// human readable message with details about the request state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// brief reason for the request state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// request approval state, currently Approved or Denied.
    #[serde(rename = "type")]
    pub type_: String,
}
