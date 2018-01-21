// Generated from definition io.k8s.api.authorization.v1.SelfSubjectAccessReview

/// SelfSubjectAccessReview checks whether or the current user can perform an action.  Not filling in a spec.namespace means "in all namespaces".  Self is a special case, because users should always be able to check whether they can perform an action
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SelfSubjectAccessReview {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec holds information about the request being evaluated.  user and groups must be empty
    pub spec: ::api::authorization::v1::SelfSubjectAccessReviewSpec,

    /// Status is filled in by the server and indicates whether the request is allowed or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::api::authorization::v1::SubjectAccessReviewStatus>,
}
