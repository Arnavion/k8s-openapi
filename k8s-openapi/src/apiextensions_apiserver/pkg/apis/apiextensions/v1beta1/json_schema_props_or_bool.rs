// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrBool

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JSONSchemaPropsOrBool {
    #[serde(rename = "Allows")]
    pub allows: bool,

    #[serde(rename = "Schema")]
    pub schema: Box<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>,
}
