// Generated from definition io.k8s.api.core.v1.ConfigMapVolumeSource

/// Adapts a ConfigMap into a volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. ConfigMap volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapVolumeSource {
    /// Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    pub items: Vec<crate::api::core::v1::KeyToPath>,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Specify whether the ConfigMap or its keys must be defined
    pub optional: Option<bool>,
}

impl<'de> crate::serde::Deserialize<'de> for ConfigMapVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_items,
            Key_name,
            Key_optional,
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
                            "defaultMode" => Field::Key_default_mode,
                            "items" => Field::Key_items,
                            "name" => Field::Key_name,
                            "optional" => Field::Key_optional,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigMapVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ConfigMapVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_items: Option<Vec<crate::api::core::v1::KeyToPath>> = None;
                let mut value_name: Option<String> = None;
                let mut value_optional: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapVolumeSource {
                    default_mode: value_default_mode,
                    items: value_items.unwrap_or_default(),
                    name: value_name,
                    optional: value_optional,
                })
            }
        }

        deserializer.deserialize_struct(
            "ConfigMapVolumeSource",
            &[
                "defaultMode",
                "items",
                "name",
                "optional",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ConfigMapVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ConfigMapVolumeSource",
            self.default_mode.as_ref().map_or(0, |_| 1) +
            usize::from(!self.items.is_empty()) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        if !self.items.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl ConfigMapVolumeSource {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Adapts a ConfigMap into a volume.\n\nThe contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. ConfigMap volumes support ownership management and SELinux relabeling.",
          "properties": {
            "defaultMode": {
              "description": "Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.",
              "format": "int32",
              "type": "integer"
            },
            "items": {
              "description": "If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.",
              "items": crate::api::core::v1::KeyToPath::schema(),
              "type": "array"
            },
            "name": {
              "description": "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names",
              "type": "string"
            },
            "optional": {
              "description": "Specify whether the ConfigMap or its keys must be defined",
              "type": "boolean"
            }
          },
          "type": "object"
        })
    }
}