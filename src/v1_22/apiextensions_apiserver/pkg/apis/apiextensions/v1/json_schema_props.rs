// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps

/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONSchemaProps {
    pub ref_path: Option<String>,

    pub schema: Option<String>,

    pub additional_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>,

    pub additional_properties: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>,

    pub all_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub any_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    /// default is a default value for undefined object fields. Defaulting is a beta feature under the CustomResourceDefaulting feature gate. Defaulting requires spec.preserveUnknownFields to be false.
    pub default: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>,

    pub definitions: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub dependencies: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>>,

    pub description: Option<String>,

    pub enum_: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>>,

    pub example: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>,

    pub exclusive_maximum: Option<bool>,

    pub exclusive_minimum: Option<bool>,

    pub external_docs: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation>,

    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated:
    ///
    /// - bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 \[RFC1034\]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?3\[0-9a-f\]{3}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?4\[0-9a-f\]{3}-?\[89ab\]\[0-9a-f\]{3}-?\[0-9a-f\]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?5\[0-9a-f\]{3}-?\[89ab\]\[0-9a-f\]{3}-?\[0-9a-f\]{12}$ - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041" - isbn10: an ISBN10 number string like "0321751043" - isbn13: an ISBN13 number string like "978-0321751041" - creditcard: a credit card number defined by the regex ^(?:4\[0-9\]{12}(?:\[0-9\]{3})?|5\[1-5\]\[0-9\]{14}|6(?:011|5\[0-9\]\[0-9\])\[0-9\]{12}|3\[47\]\[0-9\]{13}|3(?:0\[0-5\]|\[68\]\[0-9\])\[0-9\]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}\[- \]?\\d{2}\[- \]?\\d{4}$ - hexcolor: an hexadecimal color code like "#FFFFFF: following the regex ^#?(\[0-9a-fA-F\]{3}|\[0-9a-fA-F\]{6})$ - rgbcolor: an RGB color code like rgb like "rgb(255,255,2559" - byte: base64 encoded binary data - password: any kind of string - date: a date string like "2006-01-02" as defined by full-date in RFC3339 - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
    pub format: Option<String>,

    pub id: Option<String>,

    pub items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray>,

    pub max_items: Option<i64>,

    pub max_length: Option<i64>,

    pub max_properties: Option<i64>,

    pub maximum: Option<f64>,

    pub min_items: Option<i64>,

    pub min_length: Option<i64>,

    pub min_properties: Option<i64>,

    pub minimum: Option<f64>,

    pub multiple_of: Option<f64>,

    pub not: Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub nullable: Option<bool>,

    pub one_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub pattern: Option<String>,

    pub pattern_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub required: Option<Vec<String>>,

    pub title: Option<String>,

    pub type_: Option<String>,

    pub unique_items: Option<bool>,

    /// x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).
    pub x_kubernetes_embedded_resource: Option<bool>,

    /// x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:
    ///
    /// 1) anyOf:
    ///    - type: integer
    ///    - type: string
    /// 2) allOf:
    ///    - anyOf:
    ///      - type: integer
    ///      - type: string
    ///    - ... zero or more
    pub x_kubernetes_int_or_string: Option<bool>,

    /// x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map` by specifying the keys used as the index of the map.
    ///
    /// This tag MUST only be used on lists that have the "x-kubernetes-list-type" extension set to "map". Also, the values specified for this attribute must be a scalar typed field of the child structure (no nesting is supported).
    ///
    /// The properties specified must either be required or have a default value, to ensure those properties are present for all list items.
    pub x_kubernetes_list_map_keys: Option<Vec<String>>,

    /// x-kubernetes-list-type annotates an array to further describe its topology. This extension must only be used on lists and may have 3 possible values:
    ///
    /// 1) `atomic`: the list is treated as a single entity, like a scalar.
    ///      Atomic lists will be entirely replaced when updated. This extension
    ///      may be used on any type of list (struct, scalar, ...).
    /// 2) `set`:
    ///      Sets are lists that must not have multiple items with the same value. Each
    ///      value must be a scalar, an object with x-kubernetes-map-type `atomic` or an
    ///      array with x-kubernetes-list-type `atomic`.
    /// 3) `map`:
    ///      These lists are like maps in that their elements have a non-index key
    ///      used to identify them. Order is preserved upon merge. The map tag
    ///      must only be used on a list with elements of type object.
    /// Defaults to atomic for arrays.
    pub x_kubernetes_list_type: Option<String>,

    /// x-kubernetes-map-type annotates an object to further describe its topology. This extension must only be used when type is object and may have 2 possible values:
    ///
    /// 1) `granular`:
    ///      These maps are actual maps (key-value pairs) and each fields are independent
    ///      from each other (they can each be manipulated by separate actors). This is
    ///      the default behaviour for all maps.
    /// 2) `atomic`: the list is treated as a single entity, like a scalar.
    ///      Atomic maps will be entirely replaced when updated.
    pub x_kubernetes_map_type: Option<String>,

    /// x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,
}

impl crate::DeepMerge for JSONSchemaProps {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ref_path, other.ref_path);
        crate::DeepMerge::merge_from(&mut self.schema, other.schema);
        crate::DeepMerge::merge_from(&mut self.additional_items, other.additional_items);
        crate::DeepMerge::merge_from(&mut self.additional_properties, other.additional_properties);
        crate::merge_strategies::list::atomic(&mut self.all_of, other.all_of);
        crate::merge_strategies::list::atomic(&mut self.any_of, other.any_of);
        crate::DeepMerge::merge_from(&mut self.default, other.default);
        crate::merge_strategies::map::granular(&mut self.definitions, other.definitions, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.dependencies, other.dependencies, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.description, other.description);
        crate::merge_strategies::list::atomic(&mut self.enum_, other.enum_);
        crate::DeepMerge::merge_from(&mut self.example, other.example);
        crate::DeepMerge::merge_from(&mut self.exclusive_maximum, other.exclusive_maximum);
        crate::DeepMerge::merge_from(&mut self.exclusive_minimum, other.exclusive_minimum);
        crate::DeepMerge::merge_from(&mut self.external_docs, other.external_docs);
        crate::DeepMerge::merge_from(&mut self.format, other.format);
        crate::DeepMerge::merge_from(&mut self.id, other.id);
        crate::DeepMerge::merge_from(&mut self.items, other.items);
        crate::DeepMerge::merge_from(&mut self.max_items, other.max_items);
        crate::DeepMerge::merge_from(&mut self.max_length, other.max_length);
        crate::DeepMerge::merge_from(&mut self.max_properties, other.max_properties);
        crate::DeepMerge::merge_from(&mut self.maximum, other.maximum);
        crate::DeepMerge::merge_from(&mut self.min_items, other.min_items);
        crate::DeepMerge::merge_from(&mut self.min_length, other.min_length);
        crate::DeepMerge::merge_from(&mut self.min_properties, other.min_properties);
        crate::DeepMerge::merge_from(&mut self.minimum, other.minimum);
        crate::DeepMerge::merge_from(&mut self.multiple_of, other.multiple_of);
        crate::DeepMerge::merge_from(&mut self.not, other.not);
        crate::DeepMerge::merge_from(&mut self.nullable, other.nullable);
        crate::merge_strategies::list::atomic(&mut self.one_of, other.one_of);
        crate::DeepMerge::merge_from(&mut self.pattern, other.pattern);
        crate::merge_strategies::map::granular(&mut self.pattern_properties, other.pattern_properties, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.properties, other.properties, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.required, other.required);
        crate::DeepMerge::merge_from(&mut self.title, other.title);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
        crate::DeepMerge::merge_from(&mut self.unique_items, other.unique_items);
        crate::DeepMerge::merge_from(&mut self.x_kubernetes_embedded_resource, other.x_kubernetes_embedded_resource);
        crate::DeepMerge::merge_from(&mut self.x_kubernetes_int_or_string, other.x_kubernetes_int_or_string);
        crate::merge_strategies::list::atomic(&mut self.x_kubernetes_list_map_keys, other.x_kubernetes_list_map_keys);
        crate::DeepMerge::merge_from(&mut self.x_kubernetes_list_type, other.x_kubernetes_list_type);
        crate::DeepMerge::merge_from(&mut self.x_kubernetes_map_type, other.x_kubernetes_map_type);
        crate::DeepMerge::merge_from(&mut self.x_kubernetes_preserve_unknown_fields, other.x_kubernetes_preserve_unknown_fields);
    }
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
            Key_nullable,
            Key_one_of,
            Key_pattern,
            Key_pattern_properties,
            Key_properties,
            Key_required,
            Key_title,
            Key_type_,
            Key_unique_items,
            Key_x_kubernetes_embedded_resource,
            Key_x_kubernetes_int_or_string,
            Key_x_kubernetes_list_map_keys,
            Key_x_kubernetes_list_type,
            Key_x_kubernetes_map_type,
            Key_x_kubernetes_preserve_unknown_fields,
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
                            "nullable" => Field::Key_nullable,
                            "oneOf" => Field::Key_one_of,
                            "pattern" => Field::Key_pattern,
                            "patternProperties" => Field::Key_pattern_properties,
                            "properties" => Field::Key_properties,
                            "required" => Field::Key_required,
                            "title" => Field::Key_title,
                            "type" => Field::Key_type_,
                            "uniqueItems" => Field::Key_unique_items,
                            "x-kubernetes-embedded-resource" => Field::Key_x_kubernetes_embedded_resource,
                            "x-kubernetes-int-or-string" => Field::Key_x_kubernetes_int_or_string,
                            "x-kubernetes-list-map-keys" => Field::Key_x_kubernetes_list_map_keys,
                            "x-kubernetes-list-type" => Field::Key_x_kubernetes_list_type,
                            "x-kubernetes-map-type" => Field::Key_x_kubernetes_map_type,
                            "x-kubernetes-preserve-unknown-fields" => Field::Key_x_kubernetes_preserve_unknown_fields,
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
                let mut value_additional_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool> = None;
                let mut value_additional_properties: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool> = None;
                let mut value_all_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_any_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_default: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON> = None;
                let mut value_definitions: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_dependencies: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>> = None;
                let mut value_description: Option<String> = None;
                let mut value_enum_: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>> = None;
                let mut value_example: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON> = None;
                let mut value_exclusive_maximum: Option<bool> = None;
                let mut value_exclusive_minimum: Option<bool> = None;
                let mut value_external_docs: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation> = None;
                let mut value_format: Option<String> = None;
                let mut value_id: Option<String> = None;
                let mut value_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray> = None;
                let mut value_max_items: Option<i64> = None;
                let mut value_max_length: Option<i64> = None;
                let mut value_max_properties: Option<i64> = None;
                let mut value_maximum: Option<f64> = None;
                let mut value_min_items: Option<i64> = None;
                let mut value_min_length: Option<i64> = None;
                let mut value_min_properties: Option<i64> = None;
                let mut value_minimum: Option<f64> = None;
                let mut value_multiple_of: Option<f64> = None;
                let mut value_not: Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_nullable: Option<bool> = None;
                let mut value_one_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_pattern: Option<String> = None;
                let mut value_pattern_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_required: Option<Vec<String>> = None;
                let mut value_title: Option<String> = None;
                let mut value_type_: Option<String> = None;
                let mut value_unique_items: Option<bool> = None;
                let mut value_x_kubernetes_embedded_resource: Option<bool> = None;
                let mut value_x_kubernetes_int_or_string: Option<bool> = None;
                let mut value_x_kubernetes_list_map_keys: Option<Vec<String>> = None;
                let mut value_x_kubernetes_list_type: Option<String> = None;
                let mut value_x_kubernetes_map_type: Option<String> = None;
                let mut value_x_kubernetes_preserve_unknown_fields: Option<bool> = None;

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
                        Field::Key_nullable => value_nullable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_one_of => value_one_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern => value_pattern = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern_properties => value_pattern_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_properties => value_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required => value_required = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_title => value_title = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unique_items => value_unique_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_embedded_resource => value_x_kubernetes_embedded_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_int_or_string => value_x_kubernetes_int_or_string = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_list_map_keys => value_x_kubernetes_list_map_keys = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_list_type => value_x_kubernetes_list_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_map_type => value_x_kubernetes_map_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_preserve_unknown_fields => value_x_kubernetes_preserve_unknown_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    nullable: value_nullable,
                    one_of: value_one_of,
                    pattern: value_pattern,
                    pattern_properties: value_pattern_properties,
                    properties: value_properties,
                    required: value_required,
                    title: value_title,
                    type_: value_type_,
                    unique_items: value_unique_items,
                    x_kubernetes_embedded_resource: value_x_kubernetes_embedded_resource,
                    x_kubernetes_int_or_string: value_x_kubernetes_int_or_string,
                    x_kubernetes_list_map_keys: value_x_kubernetes_list_map_keys,
                    x_kubernetes_list_type: value_x_kubernetes_list_type,
                    x_kubernetes_map_type: value_x_kubernetes_map_type,
                    x_kubernetes_preserve_unknown_fields: value_x_kubernetes_preserve_unknown_fields,
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
                "nullable",
                "oneOf",
                "pattern",
                "patternProperties",
                "properties",
                "required",
                "title",
                "type",
                "uniqueItems",
                "x-kubernetes-embedded-resource",
                "x-kubernetes-int-or-string",
                "x-kubernetes-list-map-keys",
                "x-kubernetes-list-type",
                "x-kubernetes-map-type",
                "x-kubernetes-preserve-unknown-fields",
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
            self.nullable.as_ref().map_or(0, |_| 1) +
            self.one_of.as_ref().map_or(0, |_| 1) +
            self.pattern.as_ref().map_or(0, |_| 1) +
            self.pattern_properties.as_ref().map_or(0, |_| 1) +
            self.properties.as_ref().map_or(0, |_| 1) +
            self.required.as_ref().map_or(0, |_| 1) +
            self.title.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1) +
            self.unique_items.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_embedded_resource.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_int_or_string.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_list_map_keys.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_list_type.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_map_type.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_preserve_unknown_fields.as_ref().map_or(0, |_| 1),
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
        if let Some(value) = &self.nullable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nullable", value)?;
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
        if let Some(value) = &self.x_kubernetes_embedded_resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-embedded-resource", value)?;
        }
        if let Some(value) = &self.x_kubernetes_int_or_string {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-int-or-string", value)?;
        }
        if let Some(value) = &self.x_kubernetes_list_map_keys {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-list-map-keys", value)?;
        }
        if let Some(value) = &self.x_kubernetes_list_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-list-type", value)?;
        }
        if let Some(value) = &self.x_kubernetes_map_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-map-type", value)?;
        }
        if let Some(value) = &self.x_kubernetes_preserve_unknown_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-preserve-unknown-fields", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSONSchemaProps {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
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
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>(),
                    ),
                    (
                        "additionalProperties".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>(),
                    ),
                    (
                        "allOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>()))),
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
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "default".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("default is a default value for undefined object fields. Defaulting is a beta feature under the CustomResourceDefaulting feature gate. Defaulting requires spec.preserveUnknownFields to be false.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "definitions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>())),
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
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>())),
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
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "example".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>(),
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
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation>(),
                    ),
                    (
                        "format".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated:\n\n- bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 [RFC1034]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?3[0-9a-f]{3}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?4[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?5[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - isbn: an ISBN10 or ISBN13 number string like \"0321751043\" or \"978-0321751041\" - isbn10: an ISBN10 number string like \"0321751043\" - isbn13: an ISBN13 number string like \"978-0321751041\" - creditcard: a credit card number defined by the regex ^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}[- ]?\\d{2}[- ]?\\d{4}$ - hexcolor: an hexadecimal color code like \"#FFFFFF: following the regex ^#?([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$ - rgbcolor: an RGB color code like rgb like \"rgb(255,255,2559\" - byte: base64 encoded binary data - password: any kind of string - date: a date string like \"2006-01-02\" as defined by full-date in RFC3339 - duration: a duration string like \"22 ns\" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like \"2014-12-15T19:30:20.000Z\" as defined by date-time in RFC3339.".to_owned()),
                                ..Default::default()
                            })),
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
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray>(),
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
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>(),
                    ),
                    (
                        "nullable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "oneOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>()))),
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
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>())),
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
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>())),
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
                    (
                        "x-kubernetes-embedded-resource".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-int-or-string".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:\n\n1) anyOf:\n   - type: integer\n   - type: string\n2) allOf:\n   - anyOf:\n     - type: integer\n     - type: string\n   - ... zero or more".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-list-map-keys".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map` by specifying the keys used as the index of the map.\n\nThis tag MUST only be used on lists that have the \"x-kubernetes-list-type\" extension set to \"map\". Also, the values specified for this attribute must be a scalar typed field of the child structure (no nesting is supported).\n\nThe properties specified must either be required or have a default value, to ensure those properties are present for all list items.".to_owned()),
                                ..Default::default()
                            })),
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
                        "x-kubernetes-list-type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-list-type annotates an array to further describe its topology. This extension must only be used on lists and may have 3 possible values:\n\n1) `atomic`: the list is treated as a single entity, like a scalar.\n     Atomic lists will be entirely replaced when updated. This extension\n     may be used on any type of list (struct, scalar, ...).\n2) `set`:\n     Sets are lists that must not have multiple items with the same value. Each\n     value must be a scalar, an object with x-kubernetes-map-type `atomic` or an\n     array with x-kubernetes-list-type `atomic`.\n3) `map`:\n     These lists are like maps in that their elements have a non-index key\n     used to identify them. Order is preserved upon merge. The map tag\n     must only be used on a list with elements of type object.\nDefaults to atomic for arrays.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-map-type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-map-type annotates an object to further describe its topology. This extension must only be used when type is object and may have 2 possible values:\n\n1) `granular`:\n     These maps are actual maps (key-value pairs) and each fields are independent\n     from each other (they can each be manipulated by separate actors). This is\n     the default behaviour for all maps.\n2) `atomic`: the list is treated as a single entity, like a scalar.\n     Atomic maps will be entirely replaced when updated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-preserve-unknown-fields".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
