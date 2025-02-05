// Generated from definition io.k8s.api.core.v1.StorageOSVolumeSource

/// Represents a StorageOS persistent volume resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageOSVolumeSource {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<std::string::String>,

    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    pub secret_ref: Option<crate::api::core::v1::LocalObjectReference>,

    /// volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    pub volume_name: Option<std::string::String>,

    /// volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    pub volume_namespace: Option<std::string::String>,
}

impl crate::DeepMerge for StorageOSVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
        crate::DeepMerge::merge_from(&mut self.volume_name, other.volume_name);
        crate::DeepMerge::merge_from(&mut self.volume_namespace, other.volume_namespace);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageOSVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_read_only,
            Key_secret_ref,
            Key_volume_name,
            Key_volume_namespace,
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
                            "fsType" => Field::Key_fs_type,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            "volumeName" => Field::Key_volume_name,
                            "volumeNamespace" => Field::Key_volume_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageOSVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageOSVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_volume_name: Option<std::string::String> = None;
                let mut value_volume_namespace: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_namespace => value_volume_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageOSVolumeSource {
                    fs_type: value_fs_type,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                    volume_name: value_volume_name,
                    volume_namespace: value_volume_namespace,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageOSVolumeSource",
            &[
                "fsType",
                "readOnly",
                "secretRef",
                "volumeName",
                "volumeNamespace",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageOSVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageOSVolumeSource",
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1) +
            self.volume_namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.volume_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        if let Some(value) = &self.volume_namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeNamespace", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageOSVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.StorageOSVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a StorageOS persistent volume resource.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fsType".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.".into()),
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
                                description: Some("readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "volumeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeNamespace".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to \"default\" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
