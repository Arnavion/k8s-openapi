// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.FieldSelectorRequirement

/// FieldSelectorRequirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldSelectorRequirement {
    /// key is the field selector key that the requirement applies to.
    pub key: std::string::String,

    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. The list of operators may grow in the future.
    pub operator: std::string::String,

    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty.
    pub values: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for FieldSelectorRequirement {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.key, other.key);
        crate::DeepMerge::merge_from(&mut self.operator, other.operator);
        crate::merge_strategies::list::atomic(&mut self.values, other.values);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FieldSelectorRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_operator,
            Key_values,
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
                            "key" => Field::Key_key,
                            "operator" => Field::Key_operator,
                            "values" => Field::Key_values,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FieldSelectorRequirement;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FieldSelectorRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<std::string::String> = None;
                let mut value_operator: Option<std::string::String> = None;
                let mut value_values: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operator => value_operator = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values => value_values = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FieldSelectorRequirement {
                    key: value_key.unwrap_or_default(),
                    operator: value_operator.unwrap_or_default(),
                    values: value_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "FieldSelectorRequirement",
            &[
                "key",
                "operator",
                "values",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FieldSelectorRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FieldSelectorRequirement",
            2 +
            self.values.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", &self.operator)?;
        if let Some(value) = &self.values {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "values", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FieldSelectorRequirement {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.FieldSelectorRequirement".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("FieldSelectorRequirement is a selector that contains values, a key, and an operator that relates the key and values.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "key".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("key is the field selector key that the requirement applies to.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "operator".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. The list of operators may grow in the future.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "values".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "key".into(),
                    "operator".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
