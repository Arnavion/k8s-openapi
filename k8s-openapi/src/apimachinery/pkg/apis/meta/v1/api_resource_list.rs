// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList

/// APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIResourceList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// groupVersion is the group and version this APIResourceList is for.
    #[serde(rename = "groupVersion")]
    pub group_version: String,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// resources contains the name of the resources and if they are namespaced.
    pub resources: Vec<::apimachinery::pkg::apis::meta::v1::APIResource>,
}
