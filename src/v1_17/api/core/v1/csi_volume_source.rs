// Generated from definition io.k8s.api.core.v1.CSIVolumeSource

/// Represents a source location of a volume to mount, managed by an external CSI driver
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIVolumeSource {
    /// Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
    pub driver: String,

    /// Filesystem type to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
    pub fs_type: Option<String>,

    /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
    pub node_publish_secret_ref: Option<crate::api::core::v1::LocalObjectReference>,

    /// Specifies a read-only configuration for the volume. Defaults to false (read/write).
    pub read_only: Option<bool>,

    /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
    pub volume_attributes: std::collections::BTreeMap<String, String>,
}

impl<'de> crate::serde::Deserialize<'de> for CSIVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_fs_type,
            Key_node_publish_secret_ref,
            Key_read_only,
            Key_volume_attributes,
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
                            "nodePublishSecretRef" => Field::Key_node_publish_secret_ref,
                            "readOnly" => Field::Key_read_only,
                            "volumeAttributes" => Field::Key_volume_attributes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSIVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSIVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_node_publish_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_attributes: Option<std::collections::BTreeMap<String, String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_publish_secret_ref => value_node_publish_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_attributes => value_volume_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIVolumeSource {
                    driver: value_driver.ok_or_else(|| crate::serde::de::Error::missing_field("driver"))?,
                    fs_type: value_fs_type,
                    node_publish_secret_ref: value_node_publish_secret_ref,
                    read_only: value_read_only,
                    volume_attributes: value_volume_attributes.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIVolumeSource",
            &[
                "driver",
                "fsType",
                "nodePublishSecretRef",
                "readOnly",
                "volumeAttributes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSIVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.node_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            usize::from(!self.volume_attributes.is_empty()),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.node_publish_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodePublishSecretRef", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if !self.volume_attributes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeAttributes", &self.volume_attributes)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CSIVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents a source location of a volume to mount, managed by an external CSI driver",
          "properties": {
            "driver": {
              "description": "Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.",
              "type": "string"
            },
            "fsType": {
              "description": "Filesystem type to mount. Ex. \"ext4\", \"xfs\", \"ntfs\". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.",
              "type": "string"
            },
            "nodePublishSecretRef": crate::schema_ref_with_values(crate::api::core::v1::LocalObjectReference::schema(), serde_json::json!({"description": "NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed."})),
            "readOnly": {
              "description": "Specifies a read-only configuration for the volume. Defaults to false (read/write).",
              "type": "boolean"
            },
            "volumeAttributes": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.",
              "type": "object"
            }
          },
          "required": [
            "driver"
          ],
          "type": "object"
        })
    }
}
