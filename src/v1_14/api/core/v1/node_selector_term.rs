// Generated from definition io.k8s.api.core.v1.NodeSelectorTerm

/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSelectorTerm {
    /// A list of node selector requirements by node's labels.
    pub match_expressions: Vec<crate::api::core::v1::NodeSelectorRequirement>,

    /// A list of node selector requirements by node's fields.
    pub match_fields: Vec<crate::api::core::v1::NodeSelectorRequirement>,
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
                    match_expressions: value_match_expressions.unwrap_or_default(),
                    match_fields: value_match_fields.unwrap_or_default(),
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
            usize::from(!self.match_expressions.is_empty()) +
            usize::from(!self.match_fields.is_empty()),
        )?;
        if !self.match_expressions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", &self.match_expressions)?;
        }
        if !self.match_fields.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchFields", &self.match_fields)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for NodeSelectorTerm {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.",
          "properties": {
            "matchExpressions": {
              "description": "A list of node selector requirements by node's labels.",
              "items": crate::api::core::v1::NodeSelectorRequirement::schema(),
              "type": "array"
            },
            "matchFields": {
              "description": "A list of node selector requirements by node's fields.",
              "items": crate::api::core::v1::NodeSelectorRequirement::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
