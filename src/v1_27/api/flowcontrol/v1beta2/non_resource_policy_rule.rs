// Generated from definition io.k8s.api.flowcontrol.v1beta2.NonResourcePolicyRule

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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NonResourcePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

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
    fn schema_name() -> String {
        "io.k8s.api.flowcontrol.v1beta2.NonResourcePolicyRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nonResourceURLs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty. For example:\n  - \"/healthz\" is legal\n  - \"/hea*\" is illegal\n  - \"/hea\" is legal but matches nothing\n  - \"/hea/*\" also matches nothing\n  - \"/healthz/*\" matches all per-component health checks.\n\"*\" matches all non-resource urls. if it is present, it must be the only entry. Required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "verbs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`verbs` is a list of matching verbs and may not be empty. \"*\" matches all verbs. If it is present, it must be the only entry. Required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "nonResourceURLs".to_owned(),
                    "verbs".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
