// Generated from definition io.k8s.api.core.v1.NodeSelectorTerm

/// A null or empty node selector term matches no objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSelectorTerm {
    /// Required. A list of node selector requirements. The requirements are ANDed.
    pub match_expressions: Vec<::v1_10::api::core::v1::NodeSelectorRequirement>,
}

impl<'de> ::serde::Deserialize<'de> for NodeSelectorTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
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
                let mut value_match_expressions: Option<Vec<::v1_10::api::core::v1::NodeSelectorRequirement>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSelectorTerm {
                    match_expressions: value_match_expressions.ok_or_else(|| ::serde::de::Error::missing_field("matchExpressions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSelectorTerm",
            &[
                "matchExpressions",
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
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", &self.match_expressions)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
