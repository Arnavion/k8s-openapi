// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrArray

/// JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps or an array of JSONSchemaProps. Mainly here for serialization purposes.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JSONSchemaPropsOrArray {
    #[serde(rename = "JSONSchemas")]
    pub json_schemas: Vec<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>,

    #[serde(rename = "Schema")]
    pub schema: Box<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>,
}
