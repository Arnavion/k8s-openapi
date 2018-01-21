// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps

/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JSONSchemaProps {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_path: Option<String>,

    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    #[serde(rename = "additionalItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_items: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool>,

    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool>,

    #[serde(rename = "allOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(rename = "anyOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<::std::collections::BTreeMap<String, ::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<::std::collections::BTreeMap<String, ::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrStringArray>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>,

    #[serde(rename = "exclusiveMaximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,

    #[serde(rename = "exclusiveMinimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,

    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::ExternalDocumentation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrArray>,

    #[serde(rename = "maxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,

    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,

    #[serde(rename = "maxProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    #[serde(rename = "minItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,

    #[serde(rename = "minLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,

    #[serde(rename = "minProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    #[serde(rename = "multipleOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(rename = "oneOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(rename = "patternProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<::std::collections::BTreeMap<String, ::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::BTreeMap<String, ::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "uniqueItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
}
