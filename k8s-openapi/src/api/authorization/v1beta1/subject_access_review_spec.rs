// Generated from definition io.k8s.api.authorization.v1beta1.SubjectAccessReviewSpec

/// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubjectAccessReviewSpec {
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<::std::collections::BTreeMap<String, Vec<String>>>,

    /// Groups is the groups you're testing for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,

    /// NonResourceAttributes describes information for a non-resource access request
    #[serde(rename = "nonResourceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<::api::authorization::v1beta1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    #[serde(rename = "resourceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<::api::authorization::v1beta1::ResourceAttributes>,

    /// UID information about the requesting user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// User is the user you're testing for. If you specify "User" but not "Group", then is it interpreted as "What if User were not a member of any groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
