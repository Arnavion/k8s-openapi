// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinition

/// CustomResourceDefinition represents a resource that should be exposed on the API server.  Its name MUST be in the format <.spec.name>.<.spec.group>.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CustomResourceDefinition {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec describes how the user wants the resources to appear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec>,

    /// Status indicates the actual state of the CustomResourceDefinition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus>,
}
