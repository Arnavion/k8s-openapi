// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.ThirdPartyResource

/// A ThirdPartyResource is a generic representation of a resource, it is used by add-ons and plugins to add new resource types to the API.  It consists of one or more Versions of the api.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ThirdPartyResource {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Description is the description of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Versions are versions for this third party object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::APIVersion>>,
}
