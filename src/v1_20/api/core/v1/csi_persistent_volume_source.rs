// Generated from definition io.k8s.api.core.v1.CSIPersistentVolumeSource

/// Represents storage that is managed by an external CSI volume driver (Beta feature)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIPersistentVolumeSource {
    /// ControllerExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerExpandVolume call. This is an alpha field and requires enabling ExpandCSIVolumes feature gate. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub controller_expand_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// ControllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub controller_publish_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// Driver is the name of the driver to use for this volume. Required.
    pub driver: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs".
    pub fs_type: Option<String>,

    /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_publish_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// NodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_stage_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    pub read_only: Option<bool>,

    /// Attributes of the volume to publish.
    pub volume_attributes: Option<std::collections::BTreeMap<String, String>>,

    /// VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    pub volume_handle: String,
}

impl<'de> serde::Deserialize<'de> for CSIPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_controller_expand_secret_ref,
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
                            "controllerExpandSecretRef" => Field::Key_controller_expand_secret_ref,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CSIPersistentVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSIPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_controller_expand_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_controller_publish_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_driver: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_node_publish_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_node_stage_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_volume_handle: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller_expand_secret_ref => value_controller_expand_secret_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_controller_publish_secret_ref => value_controller_publish_secret_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_fs_type => value_fs_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_publish_secret_ref => value_node_publish_secret_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_stage_secret_ref => value_node_stage_secret_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_attributes => value_volume_attributes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_handle => value_volume_handle = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIPersistentVolumeSource {
                    controller_expand_secret_ref: value_controller_expand_secret_ref,
                    controller_publish_secret_ref: value_controller_publish_secret_ref,
                    driver: value_driver.ok_or_else(|| serde::de::Error::missing_field("driver"))?,
                    fs_type: value_fs_type,
                    node_publish_secret_ref: value_node_publish_secret_ref,
                    node_stage_secret_ref: value_node_stage_secret_ref,
                    read_only: value_read_only,
                    volume_attributes: value_volume_attributes,
                    volume_handle: value_volume_handle.ok_or_else(|| serde::de::Error::missing_field("volumeHandle"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIPersistentVolumeSource",
            &[
                "controllerExpandSecretRef",
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

impl serde::Serialize for CSIPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIPersistentVolumeSource",
            2 +
            self.controller_expand_secret_ref.as_ref().map_or(0, |_| 1) +
            self.controller_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.node_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.node_stage_secret_ref.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.volume_attributes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.controller_expand_secret_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "controllerExpandSecretRef", value)?;
        }
        if let Some(value) = &self.controller_publish_secret_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "controllerPublishSecretRef", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.node_publish_secret_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodePublishSecretRef", value)?;
        }
        if let Some(value) = &self.node_stage_secret_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodeStageSecretRef", value)?;
        }
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.volume_attributes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeAttributes", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "volumeHandle", &self.volume_handle)?;
        serde::ser::SerializeStruct::end(state)
    }
}
