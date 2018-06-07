// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewSpec

/// TokenReviewSpec is a description of the token authentication request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TokenReviewSpec {
    /// Token is the opaque bearer token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
