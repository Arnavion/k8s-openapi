// Generated from definition io.k8s.api.authorization.v1.NonResourceRule

/// NonResourceRule holds information that describes a rule for the non-resource
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NonResourceRule {
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  "*" means all.
    pub non_resource_urls: Option<std::vec::Vec<std::string::String>>,

    /// Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.
    pub verbs: std::vec::Vec<std::string::String>,
}

impl crate::DeepMerge for NonResourceRule {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.non_resource_urls, other.non_resource_urls);
        crate::merge_strategies::list::atomic(&mut self.verbs, other.verbs);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NonResourceRule {
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
            type Value = NonResourceRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NonResourceRule")
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

                Ok(NonResourceRule {
                    non_resource_urls: value_non_resource_urls,
                    verbs: value_verbs.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NonResourceRule",
            &[
                "nonResourceURLs",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NonResourceRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourceRule",
            1 +
            self.non_resource_urls.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_urls {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NonResourceRule {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authorization.v1.NonResourceRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NonResourceRule holds information that describes a rule for the non-resource",
            "type": "object",
            "properties": {
                "nonResourceURLs": {
                    "description": "NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  \"*\" means all.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "verbs": {
                    "description": "Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  \"*\" means all.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "verbs",
            ],
        })
    }
}
