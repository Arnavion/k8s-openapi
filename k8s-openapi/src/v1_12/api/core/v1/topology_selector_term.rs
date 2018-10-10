// Generated from definition io.k8s.api.core.v1.TopologySelectorTerm

/// A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopologySelectorTerm {
    /// A list of topology selector requirements by labels.
    pub match_label_expressions: Option<Vec<::v1_12::api::core::v1::TopologySelectorLabelRequirement>>,
}

impl<'de> ::serde::Deserialize<'de> for TopologySelectorTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_label_expressions,
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
                            "matchLabelExpressions" => Field::Key_match_label_expressions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TopologySelectorTerm;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct TopologySelectorTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_match_label_expressions: Option<Vec<::v1_12::api::core::v1::TopologySelectorLabelRequirement>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_label_expressions => value_match_label_expressions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TopologySelectorTerm {
                    match_label_expressions: value_match_label_expressions,
                })
            }
        }

        deserializer.deserialize_struct(
            "TopologySelectorTerm",
            &[
                "matchLabelExpressions",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for TopologySelectorTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TopologySelectorTerm",
            0 +
            self.match_label_expressions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_label_expressions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "matchLabelExpressions", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
