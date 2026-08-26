// Generated from definition io.k8s.api.lifecycle.v1alpha1.EvictionRequestSpec

/// EvictionRequestSpec is a specification of an EvictionRequest.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionRequestSpec {
    /// intent specifies the action that should be taken for the specified target.
    ///
    /// - Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.
    ///   If all requesters' intents are withdrawn for a common target, the eviction will be canceled.
    ///   Cancellation consequences:
    ///   - Inactive responders will never run.
    ///   - Active responders are expected to cancel the eviction.
    ///   - Completed or Interrupted responders should not take any action.
    pub intent: std::string::String,

    /// requester allows you to identify the entity, that requested the eviction of the target.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field is required and immutable.
    pub requester: std::string::String,

    /// target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.
    pub target: crate::api::lifecycle::v1alpha1::EvictionRequestTarget,
}

impl crate::DeepMerge for EvictionRequestSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.intent, other.intent);
        crate::DeepMerge::merge_from(&mut self.requester, other.requester);
        crate::DeepMerge::merge_from(&mut self.target, other.target);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_intent,
            Key_requester,
            Key_target,
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
                            "requester" => Field::Key_requester,
                            "target" => Field::Key_target,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionRequestSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_intent: Option<std::string::String> = None;
                let mut value_requester: Option<std::string::String> = None;
                let mut value_target: Option<crate::api::lifecycle::v1alpha1::EvictionRequestTarget> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_intent => value_intent = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requester => value_requester = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target => value_target = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionRequestSpec {
                    intent: value_intent.unwrap_or_default(),
                    requester: value_requester.unwrap_or_default(),
                    target: value_target.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionRequestSpec",
            &[
                "intent",
                "requester",
                "target",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionRequestSpec",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "intent", &self.intent)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requester", &self.requester)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionRequestSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.EvictionRequestSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionRequestSpec is a specification of an EvictionRequest.",
            "type": "object",
            "properties": {
                "intent": {
                    "description": "intent specifies the action that should be taken for the specified target.\n\n- Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.\n  If all requesters' intents are withdrawn for a common target, the eviction will be canceled.\n  Cancellation consequences:\n  - Inactive responders will never run.\n  - Active responders are expected to cancel the eviction.\n  - Completed or Interrupted responders should not take any action.",
                    "type": "string",
                },
                "requester": {
                    "description": "requester allows you to identify the entity, that requested the eviction of the target.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field is required and immutable.",
                    "type": "string",
                },
                "target": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionRequestTarget>();
                    schema_obj.ensure_object().insert("description".into(), "target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.".into());
                    schema_obj
                }),
            },
            "required": [
                "intent",
                "requester",
                "target",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionRequestSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.EvictionRequestSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionRequestSpec is a specification of an EvictionRequest.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "intent".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("intent specifies the action that should be taken for the specified target.\n\n- Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.\n  If all requesters' intents are withdrawn for a common target, the eviction will be canceled.\n  Cancellation consequences:\n  - Inactive responders will never run.\n  - Active responders are expected to cancel the eviction.\n  - Completed or Interrupted responders should not take any action.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "requester".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("requester allows you to identify the entity, that requested the eviction of the target.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field is required and immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "target".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionRequestTarget>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "intent".into(),
                    "requester".into(),
                    "target".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
