// Generated from definition io.k8s.api.authorization.v1beta1.SubjectRulesReviewStatus

/// SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubjectRulesReviewStatus {
    /// EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.
    #[serde(rename = "evaluationError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: Option<String>,

    /// Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
    pub incomplete: bool,

    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    #[serde(rename = "nonResourceRules")]
    pub non_resource_rules: Vec<::api::authorization::v1beta1::NonResourceRule>,

    /// ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    #[serde(rename = "resourceRules")]
    pub resource_rules: Vec<::api::authorization::v1beta1::ResourceRule>,
}
