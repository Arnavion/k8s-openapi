// Generated from definition io.k8s.api.core.v1.KeyToPath

/// Maps a string key to a path within a volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyToPath {
    /// The key to project.
    pub key: String,

    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub mode: Option<i32>,

    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}

impl<'de> crate::serde::Deserialize<'de> for KeyToPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_mode,
            Key_path,
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
                            "key" => Field::Key_key,
                            "mode" => Field::Key_mode,
                            "path" => Field::Key_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = KeyToPath;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("KeyToPath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_mode: Option<i32> = None;
                let mut value_path: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_mode => value_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(KeyToPath {
                    key: value_key.ok_or_else(|| crate::serde::de::Error::missing_field("key"))?,
                    mode: value_mode,
                    path: value_path.ok_or_else(|| crate::serde::de::Error::missing_field("path"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "KeyToPath",
            &[
                "key",
                "mode",
                "path",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for KeyToPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "KeyToPath",
            2 +
            self.mode.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mode", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for KeyToPath {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Maps a string key to a path within a volume.",
          "properties": {
            "key": {
              "description": "The key to project.",
              "type": "string"
            },
            "mode": {
              "description": "Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.",
              "format": "int32",
              "type": "integer"
            },
            "path": {
              "description": "The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.",
              "type": "string"
            }
          },
          "required": [
            "key",
            "path"
          ],
          "type": "object"
        })
    }
}
