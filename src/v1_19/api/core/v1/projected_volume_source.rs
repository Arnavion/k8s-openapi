// Generated from definition io.k8s.api.core.v1.ProjectedVolumeSource

/// Represents a projected volume source
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectedVolumeSource {
    /// Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// list of volume projections
    pub sources: Vec<crate::api::core::v1::VolumeProjection>,
}

impl<'de> crate::serde::Deserialize<'de> for ProjectedVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_sources,
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
                            "sources" => Field::Key_sources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectedVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ProjectedVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_sources: Option<Vec<crate::api::core::v1::VolumeProjection>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sources => value_sources = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ProjectedVolumeSource {
                    default_mode: value_default_mode,
                    sources: value_sources.ok_or_else(|| crate::serde::de::Error::missing_field("sources"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ProjectedVolumeSource",
            &[
                "defaultMode",
                "sources",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ProjectedVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ProjectedVolumeSource",
            1 +
            self.default_mode.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sources", &self.sources)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ProjectedVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents a projected volume source",
          "properties": {
            "defaultMode": {
              "description": "Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.",
              "format": "int32",
              "type": "integer"
            },
            "sources": {
              "description": "list of volume projections",
              "items": crate::api::core::v1::VolumeProjection::schema(),
              "type": "array"
            }
          },
          "required": [
            "sources"
          ],
          "type": "object"
        })
    }
}
