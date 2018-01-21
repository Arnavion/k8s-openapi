// Generated from definition io.k8s.api.rbac.v1beta1.Subject

/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Subject {
    /// APIGroup holds the API group of the referenced subject. Defaults to "" for ServiceAccount subjects. Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
    #[serde(rename = "apiGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,

    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    pub kind: String,

    /// Name of the object being referenced.
    pub name: String,

    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
