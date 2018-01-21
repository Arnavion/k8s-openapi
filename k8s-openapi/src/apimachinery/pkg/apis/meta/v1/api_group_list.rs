// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIGroupList

/// APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIGroupList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// groups is a list of APIGroup.
    pub groups: Vec<::apimachinery::pkg::apis::meta::v1::APIGroup>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
