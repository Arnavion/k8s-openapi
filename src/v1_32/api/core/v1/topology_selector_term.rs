// Generated from definition io.k8s.api.core.v1.TopologySelectorTerm

/// A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopologySelectorTerm {
    /// A list of topology selector requirements by labels.
    pub match_label_expressions: Option<std::vec::Vec<crate::api::core::v1::TopologySelectorLabelRequirement>>,
}

impl crate::DeepMerge for TopologySelectorTerm {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.match_label_expressions, other.match_label_expressions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TopologySelectorTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_label_expressions,
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
                            "matchLabelExpressions" => Field::Key_match_label_expressions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TopologySelectorTerm;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TopologySelectorTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_label_expressions: Option<std::vec::Vec<crate::api::core::v1::TopologySelectorLabelRequirement>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_label_expressions => value_match_label_expressions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for TopologySelectorTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TopologySelectorTerm",
            self.match_label_expressions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_label_expressions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchLabelExpressions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TopologySelectorTerm {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.TopologySelectorTerm".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.",
            "type": "object",
            "properties": {
                "matchLabelExpressions": {
                    "description": "A list of topology selector requirements by labels.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::TopologySelectorLabelRequirement>()),
                },
            },
        })
    }
}
