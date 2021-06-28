// Generated from definition io.k8s.api.core.v1.FCVolumeSource

/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FCVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// Optional: FC target lun number
    pub lun: Option<i32>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// Optional: FC target worldwide names (WWNs)
    pub target_wwns: Vec<String>,

    /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    pub wwids: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for FCVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_lun,
            Key_read_only,
            Key_target_wwns,
            Key_wwids,
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
                            "fsType" => Field::Key_fs_type,
                            "lun" => Field::Key_lun,
                            "readOnly" => Field::Key_read_only,
                            "targetWWNs" => Field::Key_target_wwns,
                            "wwids" => Field::Key_wwids,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FCVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FCVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_target_wwns: Option<Vec<String>> = None;
                let mut value_wwids: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lun => value_lun = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_wwns => value_target_wwns = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_wwids => value_wwids = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FCVolumeSource {
                    fs_type: value_fs_type,
                    lun: value_lun,
                    read_only: value_read_only,
                    target_wwns: value_target_wwns.unwrap_or_default(),
                    wwids: value_wwids.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "FCVolumeSource",
            &[
                "fsType",
                "lun",
                "readOnly",
                "targetWWNs",
                "wwids",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FCVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FCVolumeSource",
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.lun.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            usize::from(!self.target_wwns.is_empty()) +
            usize::from(!self.wwids.is_empty()),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.lun {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lun", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if !self.target_wwns.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetWWNs", &self.target_wwns)?;
        }
        if !self.wwids.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "wwids", &self.wwids)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for FCVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.",
          "properties": {
            "fsType": {
              "description": "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.",
              "type": "string"
            },
            "lun": {
              "description": "Optional: FC target lun number",
              "format": "int32",
              "type": "integer"
            },
            "readOnly": {
              "description": "Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
              "type": "boolean"
            },
            "targetWWNs": {
              "description": "Optional: FC target worldwide names (WWNs)",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "wwids": {
              "description": "Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
