// Generated from definition io.k8s.api.authorization.v1.NonResourceRule

/// NonResourceRule holds information that describes a rule for the non-resource
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NonResourceRule {
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  "*" means all.
    pub non_resource_urls: Option<Vec<String>>,

    /// Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.
    pub verbs: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for NonResourceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_urls,
            Key_verbs,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NonResourceRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NonResourceRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_urls => value_non_resource_urls = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourceRule {
                    non_resource_urls: value_non_resource_urls,
                    verbs: value_verbs.ok_or_else(|| serde::de::Error::missing_field("verbs"))?,
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

impl serde::Serialize for NonResourceRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourceRule",
            1 +
            self.non_resource_urls.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_urls {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        serde::ser::SerializeStruct::end(state)
    }
}
