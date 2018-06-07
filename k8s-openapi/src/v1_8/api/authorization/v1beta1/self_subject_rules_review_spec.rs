// Generated from definition io.k8s.api.authorization.v1beta1.SelfSubjectRulesReviewSpec

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SelfSubjectRulesReviewSpec {
    /// Namespace to evaluate rules for. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
