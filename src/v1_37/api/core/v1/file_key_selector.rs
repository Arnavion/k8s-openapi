// Generated from definition io.k8s.api.core.v1.FileKeySelector

/// FileKeySelector selects a key of the env file.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FileKeySelector {
    /// The key within the env file. An invalid key will prevent the pod from starting. The keys defined within a source may consist of any printable ASCII characters except '='. During Alpha stage of the EnvFiles feature gate, the key size is limited to 128 characters.
    pub key: std::string::String,

    /// Specify whether the file or its key must be defined. If the file or key does not exist, then the env var is not published. If optional is set to true and the specified key does not exist, the environment variable will not be set in the Pod's containers.
    ///
    /// If optional is set to false and the specified key does not exist, an error will be returned during Pod creation.
    pub optional: Option<bool>,

    /// The path within the volume from which to select the file. Must be relative and may not contain the '..' path or start with '..'.
    pub path: std::string::String,

    /// The name of the volume mount containing the env file.
    pub volume_name: std::string::String,
}

impl crate::DeepMerge for FileKeySelector {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.key, other.key);
        crate::DeepMerge::merge_from(&mut self.optional, other.optional);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.volume_name, other.volume_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FileKeySelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_optional,
            Key_path,
            Key_volume_name,
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
                            "key" => Field::Key_key,
                            "optional" => Field::Key_optional,
                            "path" => Field::Key_path,
                            "volumeName" => Field::Key_volume_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FileKeySelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FileKeySelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<std::string::String> = None;
                let mut value_optional: Option<bool> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_volume_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FileKeySelector {
                    key: value_key.unwrap_or_default(),
                    optional: value_optional,
                    path: value_path.unwrap_or_default(),
                    volume_name: value_volume_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "FileKeySelector",
            &[
                "key",
                "optional",
                "path",
                "volumeName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FileKeySelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FileKeySelector",
            3 +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", &self.volume_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FileKeySelector {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.FileKeySelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "FileKeySelector selects a key of the env file.",
            "type": "object",
            "properties": {
                "key": {
                    "description": "The key within the env file. An invalid key will prevent the pod from starting. The keys defined within a source may consist of any printable ASCII characters except '='. During Alpha stage of the EnvFiles feature gate, the key size is limited to 128 characters.",
                    "type": "string",
                },
                "optional": {
                    "description": "Specify whether the file or its key must be defined. If the file or key does not exist, then the env var is not published. If optional is set to true and the specified key does not exist, the environment variable will not be set in the Pod's containers.\n\nIf optional is set to false and the specified key does not exist, an error will be returned during Pod creation.",
                    "type": "boolean",
                },
                "path": {
                    "description": "The path within the volume from which to select the file. Must be relative and may not contain the '..' path or start with '..'.",
                    "type": "string",
                },
                "volumeName": {
                    "description": "The name of the volume mount containing the env file.",
                    "type": "string",
                },
            },
            "required": [
                "key",
                "path",
                "volumeName",
            ],
        })
    }
}
