// Generated from definition io.k8s.api.authorization.v1.SubjectAccessReview

/// SubjectAccessReview checks whether or not a user or group can perform an action.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubjectAccessReview {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec holds information about the request being evaluated
    pub spec: ::api::authorization::v1::SubjectAccessReviewSpec,

    /// Status is filled in by the server and indicates whether the request is allowed or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::api::authorization::v1::SubjectAccessReviewStatus>,
}
