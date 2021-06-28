// Generated from definition io.k8s.api.core.v1.DownwardAPIVolumeSource

/// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIVolumeSource {
    /// Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// Items is a list of downward API volume file
    pub items: Vec<crate::api::core::v1::DownwardAPIVolumeFile>,
}

impl<'de> crate::serde::Deserialize<'de> for DownwardAPIVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_items,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DownwardAPIVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_items: Option<Vec<crate::api::core::v1::DownwardAPIVolumeFile>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIVolumeSource {
                    default_mode: value_default_mode,
                    items: value_items.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DownwardAPIVolumeSource",
            &[
                "defaultMode",
                "items",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DownwardAPIVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIVolumeSource",
            self.default_mode.as_ref().map_or(0, |_| 1) +
            usize::from(!self.items.is_empty()),
        )?;
        if let Some(value) = &self.default_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        if !self.items.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for DownwardAPIVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.",
          "properties": {
            "defaultMode": {
              "description": "Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.",
              "format": "int32",
              "type": "integer"
            },
            "items": {
              "description": "Items is a list of downward API volume file",
              "items": crate::api::core::v1::DownwardAPIVolumeFile::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
