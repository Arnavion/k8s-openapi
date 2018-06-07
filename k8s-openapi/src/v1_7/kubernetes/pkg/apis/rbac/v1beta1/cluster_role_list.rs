// Generated from definition io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleList

/// ClusterRoleList is a collection of ClusterRoles
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ClusterRoleList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Items is a list of ClusterRoles
    pub items: Vec<::v1_7::kubernetes::pkg::apis::rbac::v1beta1::ClusterRole>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ListMeta>,
}
