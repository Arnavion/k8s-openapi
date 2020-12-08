// Generated from definition io.k8s.api.core.v1.KeyToPath

/// Maps a string key to a path within a volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyToPath {
    /// The key to project.
    pub key: String,

    /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub mode: Option<i32>,

    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}

impl<'de> serde::Deserialize<'de> for KeyToPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_mode,
            Key_path,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyToPath;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("KeyToPath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_mode: Option<i32> = None;
                let mut value_path: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_mode => value_mode = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(KeyToPath {
                    key: value_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    mode: value_mode,
                    path: value_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
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

impl serde::Serialize for KeyToPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "KeyToPath",
            2 +
            self.mode.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.mode {
            serde::ser::SerializeStruct::serialize_field(&mut state, "mode", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        serde::ser::SerializeStruct::end(state)
    }
}
