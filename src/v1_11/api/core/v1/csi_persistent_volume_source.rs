// Generated from definition io.k8s.api.core.v1.CSIPersistentVolumeSource

/// Represents storage that is managed by an external CSI volume driver (Beta feature)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIPersistentVolumeSource {
    /// ControllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub controller_publish_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// Driver is the name of the driver to use for this volume. Required.
    pub driver: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs".
    pub fs_type: Option<String>,

    /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_publish_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// NodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_stage_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    pub read_only: Option<bool>,

    /// Attributes of the volume to publish.
    pub volume_attributes: std::collections::BTreeMap<String, String>,

    /// VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    pub volume_handle: String,
}

impl<'de> crate::serde::Deserialize<'de> for CSIPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_controller_publish_secret_ref,
            Key_driver,
            Key_fs_type,
            Key_node_publish_secret_ref,
            Key_node_stage_secret_ref,
            Key_read_only,
            Key_volume_attributes,
            Key_volume_handle,
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
                            "controllerPublishSecretRef" => Field::Key_controller_publish_secret_ref,
                            "driver" => Field::Key_driver,
                            "fsType" => Field::Key_fs_type,
                            "nodePublishSecretRef" => Field::Key_node_publish_secret_ref,
                            "nodeStageSecretRef" => Field::Key_node_stage_secret_ref,
                            "readOnly" => Field::Key_read_only,
                            "volumeAttributes" => Field::Key_volume_attributes,
                            "volumeHandle" => Field::Key_volume_handle,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSIPersistentVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSIPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_controller_publish_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_driver: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_node_publish_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_node_stage_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_volume_handle: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller_publish_secret_ref => value_controller_publish_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_publish_secret_ref => value_node_publish_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_stage_secret_ref => value_node_stage_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_attributes => value_volume_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_handle => value_volume_handle = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIPersistentVolumeSource {
                    controller_publish_secret_ref: value_controller_publish_secret_ref,
                    driver: value_driver.ok_or_else(|| crate::serde::de::Error::missing_field("driver"))?,
                    fs_type: value_fs_type,
                    node_publish_secret_ref: value_node_publish_secret_ref,
                    node_stage_secret_ref: value_node_stage_secret_ref,
                    read_only: value_read_only,
                    volume_attributes: value_volume_attributes.unwrap_or_default(),
                    volume_handle: value_volume_handle.ok_or_else(|| crate::serde::de::Error::missing_field("volumeHandle"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIPersistentVolumeSource",
            &[
                "controllerPublishSecretRef",
                "driver",
                "fsType",
                "nodePublishSecretRef",
                "nodeStageSecretRef",
                "readOnly",
                "volumeAttributes",
                "volumeHandle",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSIPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIPersistentVolumeSource",
            2 +
            self.controller_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.node_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.node_stage_secret_ref.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            usize::from(!self.volume_attributes.is_empty()),
        )?;
        if let Some(value) = &self.controller_publish_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controllerPublishSecretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.node_publish_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodePublishSecretRef", value)?;
        }
        if let Some(value) = &self.node_stage_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeStageSecretRef", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if !self.volume_attributes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeAttributes", &self.volume_attributes)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeHandle", &self.volume_handle)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CSIPersistentVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents storage that is managed by an external CSI volume driver (Beta feature)",
          "properties": {
            "controllerPublishSecretRef": crate::schema_ref_with_description(crate::api::core::v1::SecretReference::schema(), "ControllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed."),
            "driver": {
              "description": "Driver is the name of the driver to use for this volume. Required.",
              "type": "string"
            },
            "fsType": {
              "description": "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\".",
              "type": "string"
            },
            "nodePublishSecretRef": crate::schema_ref_with_description(crate::api::core::v1::SecretReference::schema(), "NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed."),
            "nodeStageSecretRef": crate::schema_ref_with_description(crate::api::core::v1::SecretReference::schema(), "NodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed."),
            "readOnly": {
              "description": "Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).",
              "type": "boolean"
            },
            "volumeAttributes": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "Attributes of the volume to publish.",
              "type": "object"
            },
            "volumeHandle": {
              "description": "VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.",
              "type": "string"
            }
          },
          "required": [
            "driver",
            "volumeHandle"
          ],
          "type": "object"
        })
    }
}
