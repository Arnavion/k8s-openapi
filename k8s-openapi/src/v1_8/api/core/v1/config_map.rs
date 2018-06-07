// Generated from definition io.k8s.api.core.v1.ConfigMap

/// ConfigMap holds configuration data for pods to consume.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigMap {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Data contains the configuration data. Each key must consist of alphanumeric characters, '-', '_' or '.'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, String>>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}
