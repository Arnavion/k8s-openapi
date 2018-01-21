// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceValidation

/// CustomResourceValidation is a list of validation methods for CustomResources.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CustomResourceValidation {
    /// OpenAPIV3Schema is the OpenAPI v3 schema to be validated against.
    #[serde(rename = "openAPIV3Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_api_v3_schema: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>,
}
