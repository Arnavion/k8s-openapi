// Generated from definition io.k8s.api.storage.v1alpha1.VolumeError

/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeError {
    /// String detailing the error encountered during Attach or Detach operation. This string maybe logged, so it should not contain sensitive information.
    pub message: Option<String>,

    /// Time the error was encountered.
    pub time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> crate::serde::Deserialize<'de> for VolumeError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_message,
            Key_time,
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeError")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_message: Option<String> = None;
                let mut value_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time => value_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for VolumeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeError",
            self.message.as_ref().map_or(0, |_| 1) +
            self.time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "time", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl VolumeError {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "VolumeError captures an error encountered during a volume operation.",
          "properties": {
            "message": {
              "description": "String detailing the error encountered during Attach or Detach operation. This string maybe logged, so it should not contain sensitive information.",
              "type": "string"
            },
            "time": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "Time the error was encountered.")
          },
          "type": "object"
        })
    }
}
