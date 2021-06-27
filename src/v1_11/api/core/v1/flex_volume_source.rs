// Generated from definition io.k8s.api.core.v1.FlexVolumeSource

/// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    pub fs_type: Option<String>,

    /// Optional: Extra command options if any.
    pub options: std::collections::BTreeMap<String, String>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    pub secret_ref: Option<crate::api::core::v1::LocalObjectReference>,
}

impl<'de> crate::serde::Deserialize<'de> for FlexVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_fs_type,
            Key_options,
            Key_read_only,
            Key_secret_ref,
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
                            "driver" => Field::Key_driver,
                            "fsType" => Field::Key_fs_type,
                            "options" => Field::Key_options,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlexVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlexVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_options: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlexVolumeSource {
                    driver: value_driver.ok_or_else(|| crate::serde::de::Error::missing_field("driver"))?,
                    fs_type: value_fs_type,
                    options: value_options.unwrap_or_default(),
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlexVolumeSource",
            &[
                "driver",
                "fsType",
                "options",
                "readOnly",
                "secretRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FlexVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlexVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            usize::from(!self.options.is_empty()) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if !self.options.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "options", &self.options)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl FlexVolumeSource {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.",
          "properties": {
            "driver": {
              "description": "Driver is the name of the driver to use for this volume.",
              "type": "string"
            },
            "fsType": {
              "description": "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default filesystem depends on FlexVolume script.",
              "type": "string"
            },
            "options": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "Optional: Extra command options if any.",
              "type": "object"
            },
            "readOnly": {
              "description": "Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
              "type": "boolean"
            },
            "secretRef": crate::schema_ref_with_description(crate::api::core::v1::LocalObjectReference::schema(), "Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.")
          },
          "required": [
            "driver"
          ],
          "type": "object"
        })
    }
}
