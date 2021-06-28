// Generated from definition io.k8s.api.core.v1.DownwardAPIVolumeFile

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIVolumeFile {
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    pub field_ref: Option<crate::api::core::v1::ObjectFieldSelector>,

    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub mode: Option<i32>,

    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: String,

    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    pub resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector>,
}

impl<'de> crate::serde::Deserialize<'de> for DownwardAPIVolumeFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_field_ref,
            Key_mode,
            Key_path,
            Key_resource_field_ref,
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
                            "fieldRef" => Field::Key_field_ref,
                            "mode" => Field::Key_mode,
                            "path" => Field::Key_path,
                            "resourceFieldRef" => Field::Key_resource_field_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIVolumeFile;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DownwardAPIVolumeFile")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_field_ref: Option<crate::api::core::v1::ObjectFieldSelector> = None;
                let mut value_mode: Option<i32> = None;
                let mut value_path: Option<String> = None;
                let mut value_resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_field_ref => value_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mode => value_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_resource_field_ref => value_resource_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIVolumeFile {
                    field_ref: value_field_ref,
                    mode: value_mode,
                    path: value_path.ok_or_else(|| crate::serde::de::Error::missing_field("path"))?,
                    resource_field_ref: value_resource_field_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "DownwardAPIVolumeFile",
            &[
                "fieldRef",
                "mode",
                "path",
                "resourceFieldRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DownwardAPIVolumeFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIVolumeFile",
            1 +
            self.field_ref.as_ref().map_or(0, |_| 1) +
            self.mode.as_ref().map_or(0, |_| 1) +
            self.resource_field_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.field_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldRef", value)?;
        }
        if let Some(value) = &self.mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mode", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.resource_field_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceFieldRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for DownwardAPIVolumeFile {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "DownwardAPIVolumeFile represents information to create the file containing the pod field",
          "properties": {
            "fieldRef": crate::schema_ref_with_description(crate::api::core::v1::ObjectFieldSelector::schema(), "Required: Selects a field of the pod: only annotations, labels, name and namespace are supported."),
            "mode": {
              "description": "Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.",
              "format": "int32",
              "type": "integer"
            },
            "path": {
              "description": "Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'",
              "type": "string"
            },
            "resourceFieldRef": crate::schema_ref_with_description(crate::api::core::v1::ResourceFieldSelector::schema(), "Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.")
          },
          "required": [
            "path"
          ],
          "type": "object"
        })
    }
}
