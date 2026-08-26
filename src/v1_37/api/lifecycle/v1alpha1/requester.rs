// Generated from definition io.k8s.api.lifecycle.v1alpha1.Requester

/// Requester allows you to identify the entity, that requested the eviction of the target.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Requester {
    /// intent specifies the action that should be taken for the specified target.
    ///
    /// - Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.
    ///   If all requesters' intents are withdrawn, the eviction will be canceled.
    ///   Cancellation consequences:
    ///   - Inactive responders will never run.
    ///   - Active responders are expected to cancel the eviction.
    ///   - Completed or Interrupted responders should not take any action.
    pub intent: std::string::String,

    /// name allows you to identify the entity, that requested the eviction of the target.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). This field must be unique for each requester. This field is required.
    pub name: std::string::String,
}

impl crate::DeepMerge for Requester {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.intent, other.intent);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Requester {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_intent,
            Key_name,
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
                            "intent" => Field::Key_intent,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Requester;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Requester")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_intent: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_intent => value_intent = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Requester {
                    intent: value_intent.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Requester",
            &[
                "intent",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Requester {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Requester",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "intent", &self.intent)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Requester {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.Requester".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Requester allows you to identify the entity, that requested the eviction of the target.",
            "type": "object",
            "properties": {
                "intent": {
                    "description": "intent specifies the action that should be taken for the specified target.\n\n- Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.\n  If all requesters' intents are withdrawn, the eviction will be canceled.\n  Cancellation consequences:\n  - Inactive responders will never run.\n  - Active responders are expected to cancel the eviction.\n  - Completed or Interrupted responders should not take any action.",
                    "type": "string",
                },
                "name": {
                    "description": "name allows you to identify the entity, that requested the eviction of the target.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). This field must be unique for each requester. This field is required.",
                    "type": "string",
                },
            },
            "required": [
                "intent",
                "name",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for Requester {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.Requester".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("Requester allows you to identify the entity, that requested the eviction of the target.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "intent".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("intent specifies the action that should be taken for the specified target.\n\n- Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.\n  If all requesters' intents are withdrawn, the eviction will be canceled.\n  Cancellation consequences:\n  - Inactive responders will never run.\n  - Active responders are expected to cancel the eviction.\n  - Completed or Interrupted responders should not take any action.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("name allows you to identify the entity, that requested the eviction of the target.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). This field must be unique for each requester. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "intent".into(),
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
