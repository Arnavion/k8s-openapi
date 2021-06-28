// Generated from definition io.k8s.api.core.v1.VsphereVirtualDiskVolumeSource

/// Represents a vSphere volume resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VsphereVirtualDiskVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
    pub storage_policy_id: Option<String>,

    /// Storage Policy Based Management (SPBM) profile name.
    pub storage_policy_name: Option<String>,

    /// Path that identifies vSphere volume vmdk
    pub volume_path: String,
}

impl<'de> crate::serde::Deserialize<'de> for VsphereVirtualDiskVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_storage_policy_id,
            Key_storage_policy_name,
            Key_volume_path,
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
                            "storagePolicyID" => Field::Key_storage_policy_id,
                            "storagePolicyName" => Field::Key_storage_policy_name,
                            "volumePath" => Field::Key_volume_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VsphereVirtualDiskVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VsphereVirtualDiskVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_storage_policy_id: Option<String> = None;
                let mut value_storage_policy_name: Option<String> = None;
                let mut value_volume_path: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_policy_id => value_storage_policy_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_policy_name => value_storage_policy_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_path => value_volume_path = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VsphereVirtualDiskVolumeSource {
                    fs_type: value_fs_type,
                    storage_policy_id: value_storage_policy_id,
                    storage_policy_name: value_storage_policy_name,
                    volume_path: value_volume_path.ok_or_else(|| crate::serde::de::Error::missing_field("volumePath"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "VsphereVirtualDiskVolumeSource",
            &[
                "fsType",
                "storagePolicyID",
                "storagePolicyName",
                "volumePath",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VsphereVirtualDiskVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VsphereVirtualDiskVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.storage_policy_id.as_ref().map_or(0, |_| 1) +
            self.storage_policy_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.storage_policy_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storagePolicyID", value)?;
        }
        if let Some(value) = &self.storage_policy_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storagePolicyName", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumePath", &self.volume_path)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for VsphereVirtualDiskVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents a vSphere volume resource.",
          "properties": {
            "fsType": {
              "description": "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.",
              "type": "string"
            },
            "storagePolicyID": {
              "description": "Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.",
              "type": "string"
            },
            "storagePolicyName": {
              "description": "Storage Policy Based Management (SPBM) profile name.",
              "type": "string"
            },
            "volumePath": {
              "description": "Path that identifies vSphere volume vmdk",
              "type": "string"
            }
          },
          "required": [
            "volumePath"
          ],
          "type": "object"
        })
    }
}
