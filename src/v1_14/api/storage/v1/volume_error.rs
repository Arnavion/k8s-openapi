// Generated from definition io.k8s.api.storage.v1.VolumeError

/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeError {
    /// String detailing the error encountered during Attach or Detach operation. This string may be logged, so it should not contain sensitive information.
    pub message: Option<String>,

    /// Time the error was encountered.
    pub time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> serde::Deserialize<'de> for VolumeError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_message,
            Key_time,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VolumeError;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeError")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_message: Option<String> = None;
                let mut value_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time => value_time = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeError {
                    message: value_message,
                    time: value_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeError",
            &[
                "message",
                "time",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for VolumeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeError",
            self.message.as_ref().map_or(0, |_| 1) +
            self.time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.time {
            serde::ser::SerializeStruct::serialize_field(&mut state, "time", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
