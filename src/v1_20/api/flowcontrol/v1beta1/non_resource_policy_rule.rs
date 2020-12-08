// Generated from definition io.k8s.api.flowcontrol.v1beta1.NonResourcePolicyRule

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
    pub non_resource_urls: Vec<String>,

    /// `verbs` is a list of matching verbs and may not be empty. "*" matches all verbs. If it is present, it must be the only entry. Required.
    pub verbs: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for NonResourcePolicyRule {
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
            type Value = NonResourcePolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NonResourcePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_urls => value_non_resource_urls = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_verbs => value_verbs = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourcePolicyRule {
                    non_resource_urls: value_non_resource_urls.ok_or_else(|| serde::de::Error::missing_field("nonResourceURLs"))?,
                    verbs: value_verbs.ok_or_else(|| serde::de::Error::missing_field("verbs"))?,
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

impl serde::Serialize for NonResourcePolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourcePolicyRule",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", &self.non_resource_urls)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        serde::ser::SerializeStruct::end(state)
    }
}
