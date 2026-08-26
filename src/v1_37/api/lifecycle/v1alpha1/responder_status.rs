// Generated from definition io.k8s.api.lifecycle.v1alpha1.ResponderStatus

/// ResponderStatus represents the last observed status of the eviction process of the responder. It should be only updated by the designated responder whose name is .name field.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponderStatus {
    /// completionTime tracks the time at which the Responder stopped processing the eviction request. Completion means that the responders has either fully or partially completed the eviction process, which may have resulted in target eviction (e.g. pod termination). It should reflect the present time when set. This field becomes immutable once set.
    pub completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// expectedCompletionTime is the time at which the eviction process step is expected to end for the responder. The time cannot be set to the past. May be omitted if no estimate can be made.
    pub expected_completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// heartbeatTime is the last time at which the eviction process was reported to be in progress by the responder. It should reflect the present time when set. Responders should avoid heartbeats more frequent than 20 seconds to avoid overloading the control-plane.
    pub heartbeat_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// message provides human-readable details about the state of the responder and the eviction process. Maximum length is 4000 characters.
    pub message: Option<std::string::String>,

    /// name allows you to identify the responder reacting to the Eviction.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). This field is initialized by Kubernetes and must be unique for each responder. This field is required.
    pub name: std::string::String,

    /// startTime tracks the time at which this responder was designated as active and should start processing the eviction request. It should reflect the present time when set. This field is initialized by Kubernetes when this responder becomes active. This field becomes immutable once set.
    pub start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for ResponderStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.completion_time, other.completion_time);
        crate::DeepMerge::merge_from(&mut self.expected_completion_time, other.expected_completion_time);
        crate::DeepMerge::merge_from(&mut self.heartbeat_time, other.heartbeat_time);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.start_time, other.start_time);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResponderStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_completion_time,
            Key_expected_completion_time,
            Key_heartbeat_time,
            Key_message,
            Key_name,
            Key_start_time,
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
                            "completionTime" => Field::Key_completion_time,
                            "expectedCompletionTime" => Field::Key_expected_completion_time,
                            "heartbeatTime" => Field::Key_heartbeat_time,
                            "message" => Field::Key_message,
                            "name" => Field::Key_name,
                            "startTime" => Field::Key_start_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResponderStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResponderStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_expected_completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_heartbeat_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_completion_time => value_completion_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expected_completion_time => value_expected_completion_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_heartbeat_time => value_heartbeat_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResponderStatus {
                    completion_time: value_completion_time,
                    expected_completion_time: value_expected_completion_time,
                    heartbeat_time: value_heartbeat_time,
                    message: value_message,
                    name: value_name.unwrap_or_default(),
                    start_time: value_start_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResponderStatus",
            &[
                "completionTime",
                "expectedCompletionTime",
                "heartbeatTime",
                "message",
                "name",
                "startTime",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResponderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResponderStatus",
            1 +
            self.completion_time.as_ref().map_or(0, |_| 1) +
            self.expected_completion_time.as_ref().map_or(0, |_| 1) +
            self.heartbeat_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.completion_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completionTime", value)?;
        }
        if let Some(value) = &self.expected_completion_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expectedCompletionTime", value)?;
        }
        if let Some(value) = &self.heartbeat_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "heartbeatTime", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.start_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResponderStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.ResponderStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResponderStatus represents the last observed status of the eviction process of the responder. It should be only updated by the designated responder whose name is .name field.",
            "type": "object",
            "properties": {
                "completionTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "completionTime tracks the time at which the Responder stopped processing the eviction request. Completion means that the responders has either fully or partially completed the eviction process, which may have resulted in target eviction (e.g. pod termination). It should reflect the present time when set. This field becomes immutable once set.".into());
                    schema_obj
                }),
                "expectedCompletionTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "expectedCompletionTime is the time at which the eviction process step is expected to end for the responder. The time cannot be set to the past. May be omitted if no estimate can be made.".into());
                    schema_obj
                }),
                "heartbeatTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "heartbeatTime is the last time at which the eviction process was reported to be in progress by the responder. It should reflect the present time when set. Responders should avoid heartbeats more frequent than 20 seconds to avoid overloading the control-plane.".into());
                    schema_obj
                }),
                "message": {
                    "description": "message provides human-readable details about the state of the responder and the eviction process. Maximum length is 4000 characters.",
                    "type": "string",
                },
                "name": {
                    "description": "name allows you to identify the responder reacting to the Eviction.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). This field is initialized by Kubernetes and must be unique for each responder. This field is required.",
                    "type": "string",
                },
                "startTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "startTime tracks the time at which this responder was designated as active and should start processing the eviction request. It should reflect the present time when set. This field is initialized by Kubernetes when this responder becomes active. This field becomes immutable once set.".into());
                    schema_obj
                }),
            },
            "required": [
                "name",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for ResponderStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.ResponderStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("ResponderStatus represents the last observed status of the eviction process of the responder. It should be only updated by the designated responder whose name is .name field.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "completionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("completionTime tracks the time at which the Responder stopped processing the eviction request. Completion means that the responders has either fully or partially completed the eviction process, which may have resulted in target eviction (e.g. pod termination). It should reflect the present time when set. This field becomes immutable once set.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "expectedCompletionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("expectedCompletionTime is the time at which the eviction process step is expected to end for the responder. The time cannot be set to the past. May be omitted if no estimate can be made.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "heartbeatTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("heartbeatTime is the last time at which the eviction process was reported to be in progress by the responder. It should reflect the present time when set. Responders should avoid heartbeats more frequent than 20 seconds to avoid overloading the control-plane.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "message".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("message provides human-readable details about the state of the responder and the eviction process. Maximum length is 4000 characters.".into()),
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
                                description: Some("name allows you to identify the responder reacting to the Eviction.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). This field is initialized by Kubernetes and must be unique for each responder. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("startTime tracks the time at which this responder was designated as active and should start processing the eviction request. It should reflect the present time when set. This field is initialized by Kubernetes when this responder becomes active. This field becomes immutable once set.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
