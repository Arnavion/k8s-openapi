// Generated from definition io.k8s.api.authorization.v1beta1.SelfSubjectAccessReviewSpec

/// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SelfSubjectAccessReviewSpec {
    /// NonResourceAttributes describes information for a non-resource access request
    #[serde(rename = "nonResourceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<::api::authorization::v1beta1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    #[serde(rename = "resourceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<::api::authorization::v1beta1::ResourceAttributes>,
}
