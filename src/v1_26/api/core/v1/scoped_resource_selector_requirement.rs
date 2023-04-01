// Generated from definition io.k8s.api.core.v1.ScopedResourceSelectorRequirement

/// A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator that relates the scope name and values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScopedResourceSelectorRequirement {
    /// Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.
    ///
    pub operator: String,

    /// The name of the scope that the selector applies to.
    ///
    pub scope_name: String,

    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    pub values: Option<Vec<String>>,
}

impl crate::DeepMerge for ScopedResourceSelectorRequirement {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.operator, other.operator);
        crate::DeepMerge::merge_from(&mut self.scope_name, other.scope_name);
        crate::merge_strategies::list::atomic(&mut self.values, other.values);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ScopedResourceSelectorRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_operator,
            Key_scope_name,
            Key_values,
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
                            "operator" => Field::Key_operator,
                            "scopeName" => Field::Key_scope_name,
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
            type Value = ScopedResourceSelectorRequirement;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ScopedResourceSelectorRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_operator: Option<String> = None;
                let mut value_scope_name: Option<String> = None;
                let mut value_values: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_operator => value_operator = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope_name => value_scope_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values => value_values = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScopedResourceSelectorRequirement {
                    operator: value_operator.unwrap_or_default(),
                    scope_name: value_scope_name.unwrap_or_default(),
                    values: value_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScopedResourceSelectorRequirement",
            &[
                "operator",
                "scopeName",
                "values",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ScopedResourceSelectorRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScopedResourceSelectorRequirement",
            2 +
            self.values.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", &self.operator)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopeName", &self.scope_name)?;
        if let Some(value) = &self.values {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "values", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ScopedResourceSelectorRequirement {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ScopedResourceSelectorRequirement".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator that relates the scope name and values.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "operator".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "scopeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The name of the scope that the selector applies to.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "values".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.".to_owned()),
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
                ].into(),
                required: [
                    "operator".to_owned(),
                    "scopeName".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
