// Generated from definition io.k8s.api.core.v1.VolumeDevice

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeDevice {
    /// devicePath is the path inside of the container that the device will be mapped to.
    pub device_path: String,

    /// name must match the name of a persistentVolumeClaim in the pod
    pub name: String,
}

impl<'de> crate::serde::Deserialize<'de> for VolumeDevice {
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
            type Value = VolumeDevice;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeDevice")
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

                Ok(VolumeDevice {
                    device_path: value_device_path.ok_or_else(|| crate::serde::de::Error::missing_field("devicePath"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeDevice",
            &[
                "devicePath",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeDevice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeDevice",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devicePath", &self.device_path)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for VolumeDevice {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "volumeDevice describes a mapping of a raw block device within a container.",
          "properties": {
            "devicePath": {
              "description": "devicePath is the path inside of the container that the device will be mapped to.",
              "type": "string"
            },
            "name": {
              "description": "name must match the name of a persistentVolumeClaim in the pod",
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
