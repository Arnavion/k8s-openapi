// Generated from definition io.k8s.api.batch.v1.PodFailurePolicy

/// PodFailurePolicy describes how failed pods influence the backoffLimit.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodFailurePolicy {
    /// A list of pod failure policy rules. The rules are evaluated in order. Once a rule matches a Pod failure, the remaining of the rules are ignored. When no rule matches the Pod failure, the default handling applies - the counter of pod failures is incremented and it is checked against the backoffLimit. At most 20 elements are allowed.
    pub rules: std::vec::Vec<crate::api::batch::v1::PodFailurePolicyRule>,
}

impl crate::DeepMerge for PodFailurePolicy {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.rules, other.rules);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodFailurePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rules,
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
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodFailurePolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodFailurePolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_rules: Option<std::vec::Vec<crate::api::batch::v1::PodFailurePolicyRule>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodFailurePolicy {
                    rules: value_rules.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodFailurePolicy",
            &[
                "rules",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodFailurePolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodFailurePolicy",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodFailurePolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.batch.v1.PodFailurePolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodFailurePolicy describes how failed pods influence the backoffLimit.",
            "type": "object",
            "properties": {
                "rules": {
                    "description": "A list of pod failure policy rules. The rules are evaluated in order. Once a rule matches a Pod failure, the remaining of the rules are ignored. When no rule matches the Pod failure, the default handling applies - the counter of pod failures is incremented and it is checked against the backoffLimit. At most 20 elements are allowed.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::batch::v1::PodFailurePolicyRule>()),
                },
            },
            "required": [
                "rules",
            ],
        })
    }
}
