// Generated from definition io.k8s.api.flowcontrol.v1.NonResourcePolicyRule

/// NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NonResourcePolicyRule {
    /// `nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty. For example:
    ///   - "/healthz" is legal
    ///   - "/hea*" is illegal
    ///   - "/hea" is legal but matches nothing
    ///   - "/hea/*" also matches nothing
    ///   - "/healthz/*" matches all per-component health checks.
    /// "*" matches all non-resource urls. if it is present, it must be the only entry. Required.
    pub non_resource_urls: std::vec::Vec<std::string::String>,

    /// `verbs` is a list of matching verbs and may not be empty. "*" matches all verbs. If it is present, it must be the only entry. Required.
    pub verbs: std::vec::Vec<std::string::String>,
}

impl crate::DeepMerge for NonResourcePolicyRule {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::set(&mut self.non_resource_urls, other.non_resource_urls);
        crate::merge_strategies::list::set(&mut self.verbs, other.verbs);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NonResourcePolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_urls,
            Key_verbs,
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
                            "nonResourceURLs" => Field::Key_non_resource_urls,
                            "verbs" => Field::Key_verbs,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NonResourcePolicyRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NonResourcePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_urls: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_verbs: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_urls => value_non_resource_urls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourcePolicyRule {
                    non_resource_urls: value_non_resource_urls.unwrap_or_default(),
                    verbs: value_verbs.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NonResourcePolicyRule",
            &[
                "nonResourceURLs",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NonResourcePolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourcePolicyRule",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", &self.non_resource_urls)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NonResourcePolicyRule {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.flowcontrol.v1.NonResourcePolicyRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.",
            "type": "object",
            "properties": {
                "nonResourceURLs": {
                    "description": "`nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty. For example:\n  - \"/healthz\" is legal\n  - \"/hea*\" is illegal\n  - \"/hea\" is legal but matches nothing\n  - \"/hea/*\" also matches nothing\n  - \"/healthz/*\" matches all per-component health checks.\n\"*\" matches all non-resource urls. if it is present, it must be the only entry. Required.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "verbs": {
                    "description": "`verbs` is a list of matching verbs and may not be empty. \"*\" matches all verbs. If it is present, it must be the only entry. Required.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "nonResourceURLs",
                "verbs",
            ],
        })
    }
}
