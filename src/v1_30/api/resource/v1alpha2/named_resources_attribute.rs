// Generated from definition io.k8s.api.resource.v1alpha2.NamedResourcesAttribute

/// NamedResourcesAttribute is a combination of an attribute name and its value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamedResourcesAttribute {
    /// BoolValue is a true/false value.
    pub bool: Option<bool>,

    /// IntValue is a 64-bit integer.
    pub int: Option<i64>,

    /// IntSliceValue is an array of 64-bit integers.
    pub int_slice: Option<crate::api::resource::v1alpha2::NamedResourcesIntSlice>,

    /// Name is unique identifier among all resource instances managed by the driver on the node. It must be a DNS subdomain.
    pub name: std::string::String,

    /// QuantityValue is a quantity.
    pub quantity: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// StringValue is a string.
    pub string: Option<std::string::String>,

    /// StringSliceValue is an array of strings.
    pub string_slice: Option<crate::api::resource::v1alpha2::NamedResourcesStringSlice>,

    /// VersionValue is a semantic version according to semver.org spec 2.0.0.
    pub version: Option<std::string::String>,
}

impl crate::DeepMerge for NamedResourcesAttribute {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.bool, other.bool);
        crate::DeepMerge::merge_from(&mut self.int, other.int);
        crate::DeepMerge::merge_from(&mut self.int_slice, other.int_slice);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.quantity, other.quantity);
        crate::DeepMerge::merge_from(&mut self.string, other.string);
        crate::DeepMerge::merge_from(&mut self.string_slice, other.string_slice);
        crate::DeepMerge::merge_from(&mut self.version, other.version);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NamedResourcesAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bool,
            Key_int,
            Key_int_slice,
            Key_name,
            Key_quantity,
            Key_string,
            Key_string_slice,
            Key_version,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "bool" => Field::Key_bool,
                            "int" => Field::Key_int,
                            "intSlice" => Field::Key_int_slice,
                            "name" => Field::Key_name,
                            "quantity" => Field::Key_quantity,
                            "string" => Field::Key_string,
                            "stringSlice" => Field::Key_string_slice,
                            "version" => Field::Key_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamedResourcesAttribute;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NamedResourcesAttribute")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_bool: Option<bool> = None;
                let mut value_int: Option<i64> = None;
                let mut value_int_slice: Option<crate::api::resource::v1alpha2::NamedResourcesIntSlice> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_quantity: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_string: Option<std::string::String> = None;
                let mut value_string_slice: Option<crate::api::resource::v1alpha2::NamedResourcesStringSlice> = None;
                let mut value_version: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bool => value_bool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_int => value_int = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_int_slice => value_int_slice = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_quantity => value_quantity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_string => value_string = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_string_slice => value_string_slice = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamedResourcesAttribute {
                    bool: value_bool,
                    int: value_int,
                    int_slice: value_int_slice,
                    name: value_name.unwrap_or_default(),
                    quantity: value_quantity,
                    string: value_string,
                    string_slice: value_string_slice,
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamedResourcesAttribute",
            &[
                "bool",
                "int",
                "intSlice",
                "name",
                "quantity",
                "string",
                "stringSlice",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamedResourcesAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamedResourcesAttribute",
            1 +
            self.bool.as_ref().map_or(0, |_| 1) +
            self.int.as_ref().map_or(0, |_| 1) +
            self.int_slice.as_ref().map_or(0, |_| 1) +
            self.quantity.as_ref().map_or(0, |_| 1) +
            self.string.as_ref().map_or(0, |_| 1) +
            self.string_slice.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bool {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bool", value)?;
        }
        if let Some(value) = &self.int {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "int", value)?;
        }
        if let Some(value) = &self.int_slice {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "intSlice", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.quantity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "quantity", value)?;
        }
        if let Some(value) = &self.string {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "string", value)?;
        }
        if let Some(value) = &self.string_slice {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stringSlice", value)?;
        }
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamedResourcesAttribute {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha2.NamedResourcesAttribute".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NamedResourcesAttribute is a combination of an attribute name and its value.",
            "type": "object",
            "properties": {
                "bool": {
                    "description": "BoolValue is a true/false value.",
                    "type": "boolean",
                },
                "int": {
                    "description": "IntValue is a 64-bit integer.",
                    "type": "integer",
                    "format": "int64",
                },
                "intSlice": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::NamedResourcesIntSlice>();
                    schema_obj.ensure_object().insert("description".into(), "IntSliceValue is an array of 64-bit integers.".into());
                    schema_obj
                }),
                "name": {
                    "description": "Name is unique identifier among all resource instances managed by the driver on the node. It must be a DNS subdomain.",
                    "type": "string",
                },
                "quantity": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "QuantityValue is a quantity.".into());
                    schema_obj
                }),
                "string": {
                    "description": "StringValue is a string.",
                    "type": "string",
                },
                "stringSlice": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::NamedResourcesStringSlice>();
                    schema_obj.ensure_object().insert("description".into(), "StringSliceValue is an array of strings.".into());
                    schema_obj
                }),
                "version": {
                    "description": "VersionValue is a semantic version according to semver.org spec 2.0.0.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
