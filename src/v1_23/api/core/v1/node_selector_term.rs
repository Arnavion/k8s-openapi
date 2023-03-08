// Generated from definition io.k8s.api.core.v1.NodeSelectorTerm

/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSelectorTerm {
    /// A list of node selector requirements by node's labels.
    pub match_expressions: Option<Vec<crate::api::core::v1::NodeSelectorRequirement>>,

    /// A list of node selector requirements by node's fields.
    pub match_fields: Option<Vec<crate::api::core::v1::NodeSelectorRequirement>>,
}

impl crate::DeepMerge for NodeSelectorTerm {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.match_expressions, other.match_expressions);
        crate::merge_strategies::list::atomic(&mut self.match_fields, other.match_fields);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeSelectorTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
            Key_match_fields,
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
                            "matchExpressions" => Field::Key_match_expressions,
                            "matchFields" => Field::Key_match_fields,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSelectorTerm;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeSelectorTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_expressions: Option<Vec<crate::api::core::v1::NodeSelectorRequirement>> = None;
                let mut value_match_fields: Option<Vec<crate::api::core::v1::NodeSelectorRequirement>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_fields => value_match_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSelectorTerm {
                    match_expressions: value_match_expressions,
                    match_fields: value_match_fields,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSelectorTerm",
            &[
                "matchExpressions",
                "matchFields",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeSelectorTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSelectorTerm",
            self.match_expressions.as_ref().map_or(0, |_| 1) +
            self.match_fields.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expressions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", value)?;
        }
        if let Some(value) = &self.match_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchFields", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeSelectorTerm {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NodeSelectorTerm".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "matchExpressions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of node selector requirements by node's labels.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::NodeSelectorRequirement>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchFields".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of node selector requirements by node's fields.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::NodeSelectorRequirement>()))),
                                ..Default::default()
                            })),
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
