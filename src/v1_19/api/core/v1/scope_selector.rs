// Generated from definition io.k8s.api.core.v1.ScopeSelector

/// A scope selector represents the AND of the selectors represented by the scoped-resource selector requirements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScopeSelector {
    /// A list of scope selector requirements by scope of the resources.
    pub match_expressions: Option<Vec<crate::api::core::v1::ScopedResourceSelectorRequirement>>,
}

impl<'de> serde::Deserialize<'de> for ScopeSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScopeSelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ScopeSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_match_expressions: Option<Vec<crate::api::core::v1::ScopedResourceSelectorRequirement>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScopeSelector {
                    match_expressions: value_match_expressions,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScopeSelector",
            &[
                "matchExpressions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ScopeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScopeSelector",
            self.match_expressions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expressions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
