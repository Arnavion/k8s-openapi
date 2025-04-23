// Generated from definition io.k8s.api.storage.v1.VolumeError

/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeError {
    /// errorCode is a numeric gRPC code representing the error encountered during Attach or Detach operations.
    ///
    /// This is an optional, alpha field that requires the MutableCSINodeAllocatableCount feature gate being enabled to be set.
    pub error_code: Option<i32>,

    /// message represents the error encountered during Attach or Detach operation. This string may be logged, so it should not contain sensitive information.
    pub message: Option<std::string::String>,

    /// time represents the time the error was encountered.
    pub time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for VolumeError {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.error_code, other.error_code);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.time, other.time);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_error_code,
            Key_message,
            Key_time,
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
                            "errorCode" => Field::Key_error_code,
                            "message" => Field::Key_message,
                            "time" => Field::Key_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeError;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeError")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_error_code: Option<i32> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_error_code => value_error_code = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time => value_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeError {
                    error_code: value_error_code,
                    message: value_message,
                    time: value_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeError",
            &[
                "errorCode",
                "message",
                "time",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeError",
            self.error_code.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.error_code {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "errorCode", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "time", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeError {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.VolumeError".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("VolumeError captures an error encountered during a volume operation.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "errorCode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("errorCode is a numeric gRPC code representing the error encountered during Attach or Detach operations.\n\nThis is an optional, alpha field that requires the MutableCSINodeAllocatableCount feature gate being enabled to be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "message".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("message represents the error encountered during Attach or Detach operation. This string may be logged, so it should not contain sensitive information.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "time".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("time represents the time the error was encountered.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
