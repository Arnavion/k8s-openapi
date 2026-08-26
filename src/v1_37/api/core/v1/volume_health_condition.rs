// Generated from definition io.k8s.api.core.v1.VolumeHealthCondition

/// VolumeHealthCondition represents an adverse health condition reported for a volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeHealthCondition {
    /// message is a human-readable description. Maximum permitted length of a message is 1024 bytes.
    pub message: Option<std::string::String>,

    /// reason is a brief CamelCase machine-parseable reason. Together with status it forms the unique identity of a condition entry. Maximum permitted length of a reason is 256 bytes.
    pub reason: std::string::String,

    /// status is the machine-parseable health category. Possible values: - "Inaccessible": the volume cannot be accessed. - "DataLoss": data loss has been detected on the volume. - "Degraded": the volume is functioning with reduced capability.
    pub status: std::string::String,
}

impl crate::DeepMerge for VolumeHealthCondition {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeHealthCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_message,
            Key_reason,
            Key_status,
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
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeHealthCondition;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeHealthCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_message: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_status: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeHealthCondition {
                    message: value_message,
                    reason: value_reason.unwrap_or_default(),
                    status: value_status.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeHealthCondition",
            &[
                "message",
                "reason",
                "status",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeHealthCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeHealthCondition",
            2 +
            self.message.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", &self.reason)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeHealthCondition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.VolumeHealthCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "VolumeHealthCondition represents an adverse health condition reported for a volume.",
            "type": "object",
            "properties": {
                "message": {
                    "description": "message is a human-readable description. Maximum permitted length of a message is 1024 bytes.",
                    "type": "string",
                },
                "reason": {
                    "description": "reason is a brief CamelCase machine-parseable reason. Together with status it forms the unique identity of a condition entry. Maximum permitted length of a reason is 256 bytes.",
                    "type": "string",
                },
                "status": {
                    "description": "status is the machine-parseable health category. Possible values: - \"Inaccessible\": the volume cannot be accessed. - \"DataLoss\": data loss has been detected on the volume. - \"Degraded\": the volume is functioning with reduced capability.",
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
impl crate::schemars08::JsonSchema for VolumeHealthCondition {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.VolumeHealthCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("VolumeHealthCondition represents an adverse health condition reported for a volume.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "message".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("message is a human-readable description. Maximum permitted length of a message is 1024 bytes.".into()),
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
                                description: Some("reason is a brief CamelCase machine-parseable reason. Together with status it forms the unique identity of a condition entry. Maximum permitted length of a reason is 256 bytes.".into()),
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
                                description: Some("status is the machine-parseable health category. Possible values: - \"Inaccessible\": the volume cannot be accessed. - \"DataLoss\": data loss has been detected on the volume. - \"Degraded\": the volume is functioning with reduced capability.".into()),
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
