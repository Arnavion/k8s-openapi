// Generated from definition io.k8s.api.rbac.v1alpha1.RoleRef

/// RoleRef contains information that points to the role being used
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RoleRef {
    /// APIGroup is the group for the resource being referenced
    #[serde(rename = "apiGroup")]
    pub api_group: String,

    /// Kind is the type of resource being referenced
    pub kind: String,

    /// Name is the name of resource being referenced
    pub name: String,
}
