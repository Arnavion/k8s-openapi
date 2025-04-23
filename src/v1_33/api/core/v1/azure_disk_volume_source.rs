// Generated from definition io.k8s.api.core.v1.AzureDiskVolumeSource

/// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureDiskVolumeSource {
    /// cachingMode is the Host Caching mode: None, Read Only, Read Write.
    pub caching_mode: Option<std::string::String>,

    /// diskName is the Name of the data disk in the blob storage
    pub disk_name: std::string::String,

    /// diskURI is the URI of data disk in the blob storage
    pub disk_uri: std::string::String,

    /// fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<std::string::String>,

    /// kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
    pub kind: Option<std::string::String>,

    /// readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,
}

impl crate::DeepMerge for AzureDiskVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.caching_mode, other.caching_mode);
        crate::DeepMerge::merge_from(&mut self.disk_name, other.disk_name);
        crate::DeepMerge::merge_from(&mut self.disk_uri, other.disk_uri);
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AzureDiskVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AzureDiskVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("AzureDiskVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_caching_mode: Option<std::string::String> = None;
                let mut value_disk_name: Option<std::string::String> = None;
                let mut value_disk_uri: Option<std::string::String> = None;
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_kind: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_caching_mode => value_caching_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_disk_name => value_disk_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_disk_uri => value_disk_uri = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureDiskVolumeSource {
                    caching_mode: value_caching_mode,
                    disk_name: value_disk_name.unwrap_or_default(),
                    disk_uri: value_disk_uri.unwrap_or_default(),
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

impl crate::serde::Serialize for AzureDiskVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AzureDiskVolumeSource",
            2 +
            self.caching_mode.as_ref().map_or(0, |_| 1) +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.caching_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cachingMode", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "diskName", &self.disk_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "diskURI", &self.disk_uri)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AzureDiskVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.AzureDiskVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "cachingMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("cachingMode is the Host Caching mode: None, Read Only, Read Write.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "diskName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("diskName is the Name of the data disk in the blob storage".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "diskURI".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("diskURI is the URI of data disk in the blob storage".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsType".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "diskName".into(),
                    "diskURI".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
