// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrStringArray

/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JSONSchemaPropsOrStringArray {
    #[serde(rename = "Property")]
    pub property: Vec<String>,

    #[serde(rename = "Schema")]
    pub schema: ::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps,
}
