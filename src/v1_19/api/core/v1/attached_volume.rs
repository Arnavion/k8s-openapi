// Generated from definition io.k8s.api.core.v1.AttachedVolume

/// AttachedVolume describes a volume attached to a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AttachedVolume {
    /// DevicePath represents the device path where the volume should be available
    pub device_path: String,

    /// Name of the attached volume
    pub name: String,
}

impl<'de> crate::serde::Deserialize<'de> for AttachedVolume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_device_path,
            Key_name,
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
                            "devicePath" => Field::Key_device_path,
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
            type Value = AttachedVolume;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AttachedVolume")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_device_path: Option<String> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_device_path => value_device_path = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AttachedVolume {
                    device_path: value_device_path.ok_or_else(|| crate::serde::de::Error::missing_field("devicePath"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "AttachedVolume",
            &[
                "devicePath",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AttachedVolume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AttachedVolume",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devicePath", &self.device_path)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl AttachedVolume {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "AttachedVolume describes a volume attached to a node",
          "properties": {
            "devicePath": {
              "description": "DevicePath represents the device path where the volume should be available",
              "type": "string"
            },
            "name": {
              "description": "Name of the attached volume",
              "type": "string"
            }
          },
          "required": [
            "devicePath",
            "name"
          ],
          "type": "object"
        })
    }
}
