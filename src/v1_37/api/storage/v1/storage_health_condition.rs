// Generated from definition io.k8s.api.storage.v1.StorageHealthCondition

/// StorageHealthCondition represents an adverse health condition reported by a CSI driver for its storage backend on a node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageHealthCondition {
    /// accessMode is the access mode affected. Nil means all access modes are affected.
    pub access_mode: Option<std::string::String>,

    /// lastTransitionTime is when this condition first appeared at its current state.
    pub last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// message is a human-readable description. Maximum permitted length of a message is 1024 characters.
    pub message: Option<std::string::String>,

    /// reason is a brief CamelCase machine-parseable reason. Maximum permitted length of a reason is 256 characters.
    pub reason: std::string::String,

    /// status is the health status category. One of "StorageUnreachable", "StorageDegraded".
    pub status: std::string::String,

    /// volumeMode is the volume mode affected. Nil means both are affected.
    pub volume_mode: Option<std::string::String>,
}

impl crate::DeepMerge for StorageHealthCondition {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.access_mode, other.access_mode);
        crate::DeepMerge::merge_from(&mut self.last_transition_time, other.last_transition_time);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
        crate::DeepMerge::merge_from(&mut self.volume_mode, other.volume_mode);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageHealthCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_mode,
            Key_last_transition_time,
            Key_message,
            Key_reason,
            Key_status,
            Key_volume_mode,
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
                            "accessMode" => Field::Key_access_mode,
                            "lastTransitionTime" => Field::Key_last_transition_time,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "status" => Field::Key_status,
                            "volumeMode" => Field::Key_volume_mode,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageHealthCondition;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageHealthCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_mode: Option<std::string::String> = None;
                let mut value_last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_status: Option<std::string::String> = None;
                let mut value_volume_mode: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_mode => value_access_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_transition_time => value_last_transition_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mode => value_volume_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageHealthCondition {
                    access_mode: value_access_mode,
                    last_transition_time: value_last_transition_time,
                    message: value_message,
                    reason: value_reason.unwrap_or_default(),
                    status: value_status.unwrap_or_default(),
                    volume_mode: value_volume_mode,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageHealthCondition",
            &[
                "accessMode",
                "lastTransitionTime",
                "message",
                "reason",
                "status",
                "volumeMode",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageHealthCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageHealthCondition",
            2 +
            self.access_mode.as_ref().map_or(0, |_| 1) +
            self.last_transition_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.volume_mode.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.access_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessMode", value)?;
        }
        if let Some(value) = &self.last_transition_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", &self.reason)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        if let Some(value) = &self.volume_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMode", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageHealthCondition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.StorageHealthCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "StorageHealthCondition represents an adverse health condition reported by a CSI driver for its storage backend on a node.",
            "type": "object",
            "properties": {
                "accessMode": {
                    "description": "accessMode is the access mode affected. Nil means all access modes are affected.",
                    "type": "string",
                },
                "lastTransitionTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "lastTransitionTime is when this condition first appeared at its current state.".into());
                    schema_obj
                }),
                "message": {
                    "description": "message is a human-readable description. Maximum permitted length of a message is 1024 characters.",
                    "type": "string",
                },
                "reason": {
                    "description": "reason is a brief CamelCase machine-parseable reason. Maximum permitted length of a reason is 256 characters.",
                    "type": "string",
                },
                "status": {
                    "description": "status is the health status category. One of \"StorageUnreachable\", \"StorageDegraded\".",
                    "type": "string",
                },
                "volumeMode": {
                    "description": "volumeMode is the volume mode affected. Nil means both are affected.",
                    "type": "string",
                },
            },
            "required": [
                "reason",
                "status",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for StorageHealthCondition {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.StorageHealthCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("StorageHealthCondition represents an adverse health condition reported by a CSI driver for its storage backend on a node.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "accessMode".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("accessMode is the access mode affected. Nil means all access modes are affected.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lastTransitionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("lastTransitionTime is when this condition first appeared at its current state.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "message".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("message is a human-readable description. Maximum permitted length of a message is 1024 characters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("reason is a brief CamelCase machine-parseable reason. Maximum permitted length of a reason is 256 characters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "status".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("status is the health status category. One of \"StorageUnreachable\", \"StorageDegraded\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeMode".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("volumeMode is the volume mode affected. Nil means both are affected.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "reason".into(),
                    "status".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
