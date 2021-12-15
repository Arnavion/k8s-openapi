// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps

/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONSchemaProps {
    pub ref_path: Option<String>,

    pub schema: Option<String>,

    pub additional_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool>,

    pub additional_properties: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool>,

    pub all_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub any_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub default: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>,

    pub definitions: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub dependencies: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrStringArray>>,

    pub description: Option<String>,

    pub enum_: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>>,

    pub example: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>,

    pub exclusive_maximum: Option<bool>,

    pub exclusive_minimum: Option<bool>,

    pub external_docs: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::ExternalDocumentation>,

    pub format: Option<String>,

    pub id: Option<String>,

    pub items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrArray>,

    pub max_items: Option<i64>,

    pub max_length: Option<i64>,

    pub max_properties: Option<i64>,

    pub maximum: Option<f64>,

    pub min_items: Option<i64>,

    pub min_length: Option<i64>,

    pub min_properties: Option<i64>,

    pub minimum: Option<f64>,

    pub multiple_of: Option<f64>,

    pub not: Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub one_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub pattern: Option<String>,

    pub pattern_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>>,

    pub required: Option<Vec<String>>,

    pub title: Option<String>,

    pub type_: Option<String>,

    pub unique_items: Option<bool>,
}

impl<'de> crate::serde::Deserialize<'de> for JSONSchemaProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ref_path,
            Key_schema,
            Key_additional_items,
            Key_additional_properties,
            Key_all_of,
            Key_any_of,
            Key_default,
            Key_definitions,
            Key_dependencies,
            Key_description,
            Key_enum_,
            Key_example,
            Key_exclusive_maximum,
            Key_exclusive_minimum,
            Key_external_docs,
            Key_format,
            Key_id,
            Key_items,
            Key_max_items,
            Key_max_length,
            Key_max_properties,
            Key_maximum,
            Key_min_items,
            Key_min_length,
            Key_min_properties,
            Key_minimum,
            Key_multiple_of,
            Key_not,
            Key_one_of,
            Key_pattern,
            Key_pattern_properties,
            Key_properties,
            Key_required,
            Key_title,
            Key_type_,
            Key_unique_items,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "$ref" => Field::Key_ref_path,
                            "$schema" => Field::Key_schema,
                            "additionalItems" => Field::Key_additional_items,
                            "additionalProperties" => Field::Key_additional_properties,
                            "allOf" => Field::Key_all_of,
                            "anyOf" => Field::Key_any_of,
                            "default" => Field::Key_default,
                            "definitions" => Field::Key_definitions,
                            "dependencies" => Field::Key_dependencies,
                            "description" => Field::Key_description,
                            "enum" => Field::Key_enum_,
                            "example" => Field::Key_example,
                            "exclusiveMaximum" => Field::Key_exclusive_maximum,
                            "exclusiveMinimum" => Field::Key_exclusive_minimum,
                            "externalDocs" => Field::Key_external_docs,
                            "format" => Field::Key_format,
                            "id" => Field::Key_id,
                            "items" => Field::Key_items,
                            "maxItems" => Field::Key_max_items,
                            "maxLength" => Field::Key_max_length,
                            "maxProperties" => Field::Key_max_properties,
                            "maximum" => Field::Key_maximum,
                            "minItems" => Field::Key_min_items,
                            "minLength" => Field::Key_min_length,
                            "minProperties" => Field::Key_min_properties,
                            "minimum" => Field::Key_minimum,
                            "multipleOf" => Field::Key_multiple_of,
                            "not" => Field::Key_not,
                            "oneOf" => Field::Key_one_of,
                            "pattern" => Field::Key_pattern,
                            "patternProperties" => Field::Key_pattern_properties,
                            "properties" => Field::Key_properties,
                            "required" => Field::Key_required,
                            "title" => Field::Key_title,
                            "type" => Field::Key_type_,
                            "uniqueItems" => Field::Key_unique_items,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaProps;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSONSchemaProps")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ref_path: Option<String> = None;
                let mut value_schema: Option<String> = None;
                let mut value_additional_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool> = None;
                let mut value_additional_properties: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool> = None;
                let mut value_all_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_any_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_default: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON> = None;
                let mut value_definitions: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_dependencies: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrStringArray>> = None;
                let mut value_description: Option<String> = None;
                let mut value_enum_: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>> = None;
                let mut value_example: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON> = None;
                let mut value_exclusive_maximum: Option<bool> = None;
                let mut value_exclusive_minimum: Option<bool> = None;
                let mut value_external_docs: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::ExternalDocumentation> = None;
                let mut value_format: Option<String> = None;
                let mut value_id: Option<String> = None;
                let mut value_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrArray> = None;
                let mut value_max_items: Option<i64> = None;
                let mut value_max_length: Option<i64> = None;
                let mut value_max_properties: Option<i64> = None;
                let mut value_maximum: Option<f64> = None;
                let mut value_min_items: Option<i64> = None;
                let mut value_min_length: Option<i64> = None;
                let mut value_min_properties: Option<i64> = None;
                let mut value_minimum: Option<f64> = None;
                let mut value_multiple_of: Option<f64> = None;
                let mut value_not: Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_one_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_pattern: Option<String> = None;
                let mut value_pattern_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;
                let mut value_required: Option<Vec<String>> = None;
                let mut value_title: Option<String> = None;
                let mut value_type_: Option<String> = None;
                let mut value_unique_items: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ref_path => value_ref_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_schema => value_schema = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_items => value_additional_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_properties => value_additional_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_all_of => value_all_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_any_of => value_any_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default => value_default = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_definitions => value_definitions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dependencies => value_dependencies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_description => value_description = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enum_ => value_enum_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_example => value_example = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exclusive_maximum => value_exclusive_maximum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exclusive_minimum => value_exclusive_minimum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_docs => value_external_docs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_format => value_format = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_id => value_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_items => value_max_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_length => value_max_length = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_properties => value_max_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_maximum => value_maximum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_items => value_min_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_length => value_min_length = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_properties => value_min_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_minimum => value_minimum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_multiple_of => value_multiple_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not => value_not = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_one_of => value_one_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern => value_pattern = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern_properties => value_pattern_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_properties => value_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required => value_required = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_title => value_title = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unique_items => value_unique_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JSONSchemaProps {
                    ref_path: value_ref_path,
                    schema: value_schema,
                    additional_items: value_additional_items,
                    additional_properties: value_additional_properties,
                    all_of: value_all_of,
                    any_of: value_any_of,
                    default: value_default,
                    definitions: value_definitions,
                    dependencies: value_dependencies,
                    description: value_description,
                    enum_: value_enum_,
                    example: value_example,
                    exclusive_maximum: value_exclusive_maximum,
                    exclusive_minimum: value_exclusive_minimum,
                    external_docs: value_external_docs,
                    format: value_format,
                    id: value_id,
                    items: value_items,
                    max_items: value_max_items,
                    max_length: value_max_length,
                    max_properties: value_max_properties,
                    maximum: value_maximum,
                    min_items: value_min_items,
                    min_length: value_min_length,
                    min_properties: value_min_properties,
                    minimum: value_minimum,
                    multiple_of: value_multiple_of,
                    not: value_not,
                    one_of: value_one_of,
                    pattern: value_pattern,
                    pattern_properties: value_pattern_properties,
                    properties: value_properties,
                    required: value_required,
                    title: value_title,
                    type_: value_type_,
                    unique_items: value_unique_items,
                })
            }
        }

        deserializer.deserialize_struct(
            "JSONSchemaProps",
            &[
                "$ref",
                "$schema",
                "additionalItems",
                "additionalProperties",
                "allOf",
                "anyOf",
                "default",
                "definitions",
                "dependencies",
                "description",
                "enum",
                "example",
                "exclusiveMaximum",
                "exclusiveMinimum",
                "externalDocs",
                "format",
                "id",
                "items",
                "maxItems",
                "maxLength",
                "maxProperties",
                "maximum",
                "minItems",
                "minLength",
                "minProperties",
                "minimum",
                "multipleOf",
                "not",
                "oneOf",
                "pattern",
                "patternProperties",
                "properties",
                "required",
                "title",
                "type",
                "uniqueItems",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for JSONSchemaProps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JSONSchemaProps",
            self.ref_path.as_ref().map_or(0, |_| 1) +
            self.schema.as_ref().map_or(0, |_| 1) +
            self.additional_items.as_ref().map_or(0, |_| 1) +
            self.additional_properties.as_ref().map_or(0, |_| 1) +
            self.all_of.as_ref().map_or(0, |_| 1) +
            self.any_of.as_ref().map_or(0, |_| 1) +
            self.default.as_ref().map_or(0, |_| 1) +
            self.definitions.as_ref().map_or(0, |_| 1) +
            self.dependencies.as_ref().map_or(0, |_| 1) +
            self.description.as_ref().map_or(0, |_| 1) +
            self.enum_.as_ref().map_or(0, |_| 1) +
            self.example.as_ref().map_or(0, |_| 1) +
            self.exclusive_maximum.as_ref().map_or(0, |_| 1) +
            self.exclusive_minimum.as_ref().map_or(0, |_| 1) +
            self.external_docs.as_ref().map_or(0, |_| 1) +
            self.format.as_ref().map_or(0, |_| 1) +
            self.id.as_ref().map_or(0, |_| 1) +
            self.items.as_ref().map_or(0, |_| 1) +
            self.max_items.as_ref().map_or(0, |_| 1) +
            self.max_length.as_ref().map_or(0, |_| 1) +
            self.max_properties.as_ref().map_or(0, |_| 1) +
            self.maximum.as_ref().map_or(0, |_| 1) +
            self.min_items.as_ref().map_or(0, |_| 1) +
            self.min_length.as_ref().map_or(0, |_| 1) +
            self.min_properties.as_ref().map_or(0, |_| 1) +
            self.minimum.as_ref().map_or(0, |_| 1) +
            self.multiple_of.as_ref().map_or(0, |_| 1) +
            self.not.as_ref().map_or(0, |_| 1) +
            self.one_of.as_ref().map_or(0, |_| 1) +
            self.pattern.as_ref().map_or(0, |_| 1) +
            self.pattern_properties.as_ref().map_or(0, |_| 1) +
            self.properties.as_ref().map_or(0, |_| 1) +
            self.required.as_ref().map_or(0, |_| 1) +
            self.title.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1) +
            self.unique_items.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ref_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "$ref", value)?;
        }
        if let Some(value) = &self.schema {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "$schema", value)?;
        }
        if let Some(value) = &self.additional_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalItems", value)?;
        }
        if let Some(value) = &self.additional_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalProperties", value)?;
        }
        if let Some(value) = &self.all_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allOf", value)?;
        }
        if let Some(value) = &self.any_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "anyOf", value)?;
        }
        if let Some(value) = &self.default {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "default", value)?;
        }
        if let Some(value) = &self.definitions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "definitions", value)?;
        }
        if let Some(value) = &self.dependencies {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dependencies", value)?;
        }
        if let Some(value) = &self.description {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.enum_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "enum", value)?;
        }
        if let Some(value) = &self.example {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "example", value)?;
        }
        if let Some(value) = &self.exclusive_maximum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exclusiveMaximum", value)?;
        }
        if let Some(value) = &self.exclusive_minimum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exclusiveMinimum", value)?;
        }
        if let Some(value) = &self.external_docs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalDocs", value)?;
        }
        if let Some(value) = &self.format {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "format", value)?;
        }
        if let Some(value) = &self.id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "id", value)?;
        }
        if let Some(value) = &self.items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        if let Some(value) = &self.max_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxItems", value)?;
        }
        if let Some(value) = &self.max_length {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxLength", value)?;
        }
        if let Some(value) = &self.max_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxProperties", value)?;
        }
        if let Some(value) = &self.maximum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maximum", value)?;
        }
        if let Some(value) = &self.min_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minItems", value)?;
        }
        if let Some(value) = &self.min_length {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minLength", value)?;
        }
        if let Some(value) = &self.min_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minProperties", value)?;
        }
        if let Some(value) = &self.minimum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minimum", value)?;
        }
        if let Some(value) = &self.multiple_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "multipleOf", value)?;
        }
        if let Some(value) = &self.not {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "not", value)?;
        }
        if let Some(value) = &self.one_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "oneOf", value)?;
        }
        if let Some(value) = &self.pattern {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pattern", value)?;
        }
        if let Some(value) = &self.pattern_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "patternProperties", value)?;
        }
        if let Some(value) = &self.properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "properties", value)?;
        }
        if let Some(value) = &self.required {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "required", value)?;
        }
        if let Some(value) = &self.title {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "title", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        if let Some(value) = &self.unique_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uniqueItems", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSONSchemaProps {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "$ref".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "$schema".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "additionalItems".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool>(),
                    ),
                    (
                        "additionalProperties".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrBool>(),
                    ),
                    (
                        "allOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "anyOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "default".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>(),
                    ),
                    (
                        "definitions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "dependencies".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrStringArray>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "description".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "enum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "example".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSON>(),
                    ),
                    (
                        "exclusiveMaximum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "exclusiveMinimum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "externalDocs".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::ExternalDocumentation>(),
                    ),
                    (
                        "format".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "id".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "items".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaPropsOrArray>(),
                    ),
                    (
                        "maxItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxLength".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxProperties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maximum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Number))),
                            format: Some("double".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minLength".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minProperties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minimum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Number))),
                            format: Some("double".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "multipleOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Number))),
                            format: Some("double".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "not".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>(),
                    ),
                    (
                        "oneOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "pattern".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "patternProperties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "properties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "required".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "title".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uniqueItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ]).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
