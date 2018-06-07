// Generated from definition io.k8s.api.authorization.v1.ResourceAttributes

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceAttributes {
    /// Group is the API Group of the Resource.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Resource is one of the existing resource types.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    /// Subresource is one of the existing resource types.  "" means none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,

    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,

    /// Version is the API Version of the Resource.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
