// Generated from definition io.k8s.api.core.v1.NodeSelector

/// A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    pub node_selector_terms: Vec<::v1_12::api::core::v1::NodeSelectorTerm>,
}

impl<'de> ::serde::Deserialize<'de> for NodeSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_selector_terms,
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
                            "nodeSelectorTerms" => Field::Key_node_selector_terms,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSelector;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_node_selector_terms: Option<Vec<::v1_12::api::core::v1::NodeSelectorTerm>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_selector_terms => value_node_selector_terms = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSelector {
                    node_selector_terms: value_node_selector_terms.ok_or_else(|| ::serde::de::Error::missing_field("nodeSelectorTerms"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSelector",
            &[
                "nodeSelectorTerms",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NodeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSelector",
            0 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelectorTerms", &self.node_selector_terms)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
