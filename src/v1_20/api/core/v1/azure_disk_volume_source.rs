// Generated from definition io.k8s.api.core.v1.AzureDiskVolumeSource

/// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureDiskVolumeSource {
    /// Host Caching mode: None, Read Only, Read Write.
    pub caching_mode: Option<String>,

    /// The Name of the data disk in the blob storage
    pub disk_name: String,

    /// The URI the data disk in the blob storage
    pub disk_uri: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
    pub kind: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for AzureDiskVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_caching_mode,
            Key_disk_name,
            Key_disk_uri,
            Key_fs_type,
            Key_kind,
            Key_read_only,
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
                            "cachingMode" => Field::Key_caching_mode,
                            "diskName" => Field::Key_disk_name,
                            "diskURI" => Field::Key_disk_uri,
                            "fsType" => Field::Key_fs_type,
                            "kind" => Field::Key_kind,
                            "readOnly" => Field::Key_read_only,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AzureDiskVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AzureDiskVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_caching_mode: Option<String> = None;
                let mut value_disk_name: Option<String> = None;
                let mut value_disk_uri: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_caching_mode => value_caching_mode = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_disk_name => value_disk_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_disk_uri => value_disk_uri = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_fs_type => value_fs_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureDiskVolumeSource {
                    caching_mode: value_caching_mode,
                    disk_name: value_disk_name.ok_or_else(|| serde::de::Error::missing_field("diskName"))?,
                    disk_uri: value_disk_uri.ok_or_else(|| serde::de::Error::missing_field("diskURI"))?,
                    fs_type: value_fs_type,
                    kind: value_kind,
                    read_only: value_read_only,
                })
            }
        }

        deserializer.deserialize_struct(
            "AzureDiskVolumeSource",
            &[
                "cachingMode",
                "diskName",
                "diskURI",
                "fsType",
                "kind",
                "readOnly",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for AzureDiskVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AzureDiskVolumeSource",
            2 +
            self.caching_mode.as_ref().map_or(0, |_| 1) +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.caching_mode {
            serde::ser::SerializeStruct::serialize_field(&mut state, "cachingMode", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "diskName", &self.disk_name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "diskURI", &self.disk_uri)?;
        if let Some(value) = &self.fs_type {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.kind {
            serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
