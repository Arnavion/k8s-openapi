// Generated from definition io.k8s.api.authorization.v1beta1.SubjectAccessReviewStatus

/// SubjectAccessReviewStatus
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubjectAccessReviewStatus {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    pub allowed: bool,

    /// Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: Option<bool>,

    /// EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
    #[serde(rename = "evaluationError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: Option<String>,

    /// Reason is optional.  It indicates why a request was allowed or denied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
