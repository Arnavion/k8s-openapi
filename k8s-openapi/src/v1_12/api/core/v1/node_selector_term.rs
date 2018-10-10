// Generated from definition io.k8s.api.core.v1.NodeSelectorTerm

/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSelectorTerm {
    /// A list of node selector requirements by node's labels.
    pub match_expressions: Option<Vec<::v1_12::api::core::v1::NodeSelectorRequirement>>,

    /// A list of node selector requirements by node's fields.
    pub match_fields: Option<Vec<::v1_12::api::core::v1::NodeSelectorRequirement>>,
}

impl<'de> ::serde::Deserialize<'de> for NodeSelectorTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
            Key_match_fields,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSelectorTerm;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeSelectorTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_match_expressions: Option<Vec<::v1_12::api::core::v1::NodeSelectorRequirement>> = None;
                let mut value_match_fields: Option<Vec<::v1_12::api::core::v1::NodeSelectorRequirement>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_fields => value_match_fields = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for NodeSelectorTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSelectorTerm",
            0 +
            self.match_expressions.as_ref().map_or(0, |_| 1) +
            self.match_fields.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expressions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", value)?;
        }
        if let Some(value) = &self.match_fields {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "matchFields", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
