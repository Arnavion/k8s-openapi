// Generated from definition io.k8s.api.authorization.v1beta1.NonResourceRule

/// NonResourceRule holds information that describes a rule for the non-resource
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NonResourceRule {
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  "*" means all.
    #[serde(rename = "nonResourceURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_ur_ls: Option<Vec<String>>,

    /// Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.
    pub verbs: Vec<String>,
}
