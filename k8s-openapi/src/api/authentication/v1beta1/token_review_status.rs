// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewStatus

/// TokenReviewStatus is the result of the token authentication request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TokenReviewStatus {
    /// Authenticated indicates that the token was associated with a known user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated: Option<bool>,

    /// Error indicates that the token couldn't be checked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// User is the UserInfo associated with the provided token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<::api::authentication::v1beta1::UserInfo>,
}
