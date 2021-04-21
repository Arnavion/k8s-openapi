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

impl<'de> serde::Deserialize<'de> for JSONSchemaProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaProps;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSONSchemaProps")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
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

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ref_path => value_ref_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_schema => value_schema = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_items => value_additional_items = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_properties => value_additional_properties = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_all_of => value_all_of = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_any_of => value_any_of = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default => value_default = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_definitions => value_definitions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dependencies => value_dependencies = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_description => value_description = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enum_ => value_enum_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_example => value_example = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exclusive_maximum => value_exclusive_maximum = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exclusive_minimum => value_exclusive_minimum = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_docs => value_external_docs = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_format => value_format = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_id => value_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_items => value_max_items = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_length => value_max_length = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_properties => value_max_properties = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_maximum => value_maximum = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_items => value_min_items = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_length => value_min_length = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_properties => value_min_properties = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_minimum => value_minimum = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_multiple_of => value_multiple_of = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not => value_not = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_one_of => value_one_of = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern => value_pattern = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern_properties => value_pattern_properties = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_properties => value_properties = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required => value_required = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_title => value_title = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unique_items => value_unique_items = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(JSONSchemaProps {
                    ref_path: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("ref_path"))?,
                    schema: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("schema"))?,
                    additional_items: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("additional_items"))?,
                    additional_properties: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("additional_properties"))?,
                    all_of: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("all_of"))?,
                    any_of: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("any_of"))?,
                    default: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("default"))?,
                    definitions: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("definitions"))?,
                    dependencies: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("dependencies"))?,
                    description: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    enum_: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("enum_"))?,
                    example: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("example"))?,
                    exclusive_maximum: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("exclusive_maximum"))?,
                    exclusive_minimum: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("exclusive_minimum"))?,
                    external_docs: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("external_docs"))?,
                    format: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("format"))?,
                    id: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    items: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("items"))?,
                    max_items: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("max_items"))?,
                    max_length: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("max_length"))?,
                    max_properties: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("max_properties"))?,
                    maximum: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("maximum"))?,
                    min_items: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("min_items"))?,
                    min_length: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("min_length"))?,
                    min_properties: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("min_properties"))?,
                    minimum: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("minimum"))?,
                    multiple_of: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("multiple_of"))?,
                    not: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("not"))?,
                    one_of: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("one_of"))?,
                    pattern: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    pattern_properties: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("pattern_properties"))?,
                    properties: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("properties"))?,
                    required: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("required"))?,
                    title: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("title"))?,
                    type_: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("type_"))?,
                    unique_items: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("unique_items"))?,
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

impl serde::Serialize for JSONSchemaProps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
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
            serde::ser::SerializeStruct::serialize_field(&mut state, "$ref", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "$ref")?;
        }
        if let Some(value) = &self.schema {
            serde::ser::SerializeStruct::serialize_field(&mut state, "$schema", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "$schema")?;
        }
        if let Some(value) = &self.additional_items {
            serde::ser::SerializeStruct::serialize_field(&mut state, "additionalItems", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "additionalItems")?;
        }
        if let Some(value) = &self.additional_properties {
            serde::ser::SerializeStruct::serialize_field(&mut state, "additionalProperties", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "additionalProperties")?;
        }
        if let Some(value) = &self.all_of {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allOf", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "allOf")?;
        }
        if let Some(value) = &self.any_of {
            serde::ser::SerializeStruct::serialize_field(&mut state, "anyOf", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "anyOf")?;
        }
        if let Some(value) = &self.default {
            serde::ser::SerializeStruct::serialize_field(&mut state, "default", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "default")?;
        }
        if let Some(value) = &self.definitions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "definitions", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "definitions")?;
        }
        if let Some(value) = &self.dependencies {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dependencies", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "dependencies")?;
        }
        if let Some(value) = &self.description {
            serde::ser::SerializeStruct::serialize_field(&mut state, "description", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "description")?;
        }
        if let Some(value) = &self.enum_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "enum", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "enum")?;
        }
        if let Some(value) = &self.example {
            serde::ser::SerializeStruct::serialize_field(&mut state, "example", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "example")?;
        }
        if let Some(value) = &self.exclusive_maximum {
            serde::ser::SerializeStruct::serialize_field(&mut state, "exclusiveMaximum", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "exclusiveMaximum")?;
        }
        if let Some(value) = &self.exclusive_minimum {
            serde::ser::SerializeStruct::serialize_field(&mut state, "exclusiveMinimum", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "exclusiveMinimum")?;
        }
        if let Some(value) = &self.external_docs {
            serde::ser::SerializeStruct::serialize_field(&mut state, "externalDocs", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "externalDocs")?;
        }
        if let Some(value) = &self.format {
            serde::ser::SerializeStruct::serialize_field(&mut state, "format", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "format")?;
        }
        if let Some(value) = &self.id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "id", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "id")?;
        }
        if let Some(value) = &self.items {
            serde::ser::SerializeStruct::serialize_field(&mut state, "items", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "items")?;
        }
        if let Some(value) = &self.max_items {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxItems", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "maxItems")?;
        }
        if let Some(value) = &self.max_length {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxLength", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "maxLength")?;
        }
        if let Some(value) = &self.max_properties {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxProperties", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "maxProperties")?;
        }
        if let Some(value) = &self.maximum {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maximum", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "maximum")?;
        }
        if let Some(value) = &self.min_items {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minItems", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "minItems")?;
        }
        if let Some(value) = &self.min_length {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minLength", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "minLength")?;
        }
        if let Some(value) = &self.min_properties {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minProperties", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "minProperties")?;
        }
        if let Some(value) = &self.minimum {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minimum", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "minimum")?;
        }
        if let Some(value) = &self.multiple_of {
            serde::ser::SerializeStruct::serialize_field(&mut state, "multipleOf", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "multipleOf")?;
        }
        if let Some(value) = &self.not {
            serde::ser::SerializeStruct::serialize_field(&mut state, "not", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "not")?;
        }
        if let Some(value) = &self.one_of {
            serde::ser::SerializeStruct::serialize_field(&mut state, "oneOf", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "oneOf")?;
        }
        if let Some(value) = &self.pattern {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pattern", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "pattern")?;
        }
        if let Some(value) = &self.pattern_properties {
            serde::ser::SerializeStruct::serialize_field(&mut state, "patternProperties", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "patternProperties")?;
        }
        if let Some(value) = &self.properties {
            serde::ser::SerializeStruct::serialize_field(&mut state, "properties", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "properties")?;
        }
        if let Some(value) = &self.required {
            serde::ser::SerializeStruct::serialize_field(&mut state, "required", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "required")?;
        }
        if let Some(value) = &self.title {
            serde::ser::SerializeStruct::serialize_field(&mut state, "title", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "title")?;
        }
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "type")?;
        }
        if let Some(value) = &self.unique_items {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uniqueItems", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "uniqueItems")?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
