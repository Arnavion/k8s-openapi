// Generated from definition io.k8s.api.core.v1.ScaleIOPersistentVolumeSource

/// ScaleIOPersistentVolumeSource represents a persistent ScaleIO volume
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScaleIOPersistentVolumeSource {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs"
    pub fs_type: Option<std::string::String>,

    /// gateway is the host address of the ScaleIO API Gateway.
    pub gateway: std::string::String,

    /// protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
    pub protection_domain: Option<std::string::String>,

    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
    pub secret_ref: crate::api::core::v1::SecretReference,

    /// sslEnabled is the flag to enable/disable SSL communication with Gateway, default false
    pub ssl_enabled: Option<bool>,

    /// storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
    pub storage_mode: Option<std::string::String>,

    /// storagePool is the ScaleIO Storage Pool associated with the protection domain.
    pub storage_pool: Option<std::string::String>,

    /// system is the name of the storage system as configured in ScaleIO.
    pub system: std::string::String,

    /// volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.
    pub volume_name: Option<std::string::String>,
}

impl crate::DeepMerge for ScaleIOPersistentVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.gateway, other.gateway);
        crate::DeepMerge::merge_from(&mut self.protection_domain, other.protection_domain);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
        crate::DeepMerge::merge_from(&mut self.ssl_enabled, other.ssl_enabled);
        crate::DeepMerge::merge_from(&mut self.storage_mode, other.storage_mode);
        crate::DeepMerge::merge_from(&mut self.storage_pool, other.storage_pool);
        crate::DeepMerge::merge_from(&mut self.system, other.system);
        crate::DeepMerge::merge_from(&mut self.volume_name, other.volume_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ScaleIOPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_gateway,
            Key_protection_domain,
            Key_read_only,
            Key_secret_ref,
            Key_ssl_enabled,
            Key_storage_mode,
            Key_storage_pool,
            Key_system,
            Key_volume_name,
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
                            "gateway" => Field::Key_gateway,
                            "protectionDomain" => Field::Key_protection_domain,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            "sslEnabled" => Field::Key_ssl_enabled,
                            "storageMode" => Field::Key_storage_mode,
                            "storagePool" => Field::Key_storage_pool,
                            "system" => Field::Key_system,
                            "volumeName" => Field::Key_volume_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ScaleIOPersistentVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ScaleIOPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_gateway: Option<std::string::String> = None;
                let mut value_protection_domain: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_ssl_enabled: Option<bool> = None;
                let mut value_storage_mode: Option<std::string::String> = None;
                let mut value_storage_pool: Option<std::string::String> = None;
                let mut value_system: Option<std::string::String> = None;
                let mut value_volume_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gateway => value_gateway = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protection_domain => value_protection_domain = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ssl_enabled => value_ssl_enabled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_mode => value_storage_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_pool => value_storage_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_system => value_system = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScaleIOPersistentVolumeSource {
                    fs_type: value_fs_type,
                    gateway: value_gateway.unwrap_or_default(),
                    protection_domain: value_protection_domain,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref.unwrap_or_default(),
                    ssl_enabled: value_ssl_enabled,
                    storage_mode: value_storage_mode,
                    storage_pool: value_storage_pool,
                    system: value_system.unwrap_or_default(),
                    volume_name: value_volume_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScaleIOPersistentVolumeSource",
            &[
                "fsType",
                "gateway",
                "protectionDomain",
                "readOnly",
                "secretRef",
                "sslEnabled",
                "storageMode",
                "storagePool",
                "system",
                "volumeName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ScaleIOPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScaleIOPersistentVolumeSource",
            3 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.protection_domain.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.ssl_enabled.as_ref().map_or(0, |_| 1) +
            self.storage_mode.as_ref().map_or(0, |_| 1) +
            self.storage_pool.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gateway", &self.gateway)?;
        if let Some(value) = &self.protection_domain {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protectionDomain", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", &self.secret_ref)?;
        if let Some(value) = &self.ssl_enabled {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sslEnabled", value)?;
        }
        if let Some(value) = &self.storage_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageMode", value)?;
        }
        if let Some(value) = &self.storage_pool {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storagePool", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "system", &self.system)?;
        if let Some(value) = &self.volume_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ScaleIOPersistentVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ScaleIOPersistentVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ScaleIOPersistentVolumeSource represents a persistent ScaleIO volume".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fsType".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Default is \"xfs\"".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gateway".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("gateway is the host address of the ScaleIO API Gateway.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "protectionDomain".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.".into()),
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
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "sslEnabled".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("sslEnabled is the flag to enable/disable SSL communication with Gateway, default false".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storageMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storagePool".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("storagePool is the ScaleIO Storage Pool associated with the protection domain.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "system".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("system is the name of the storage system as configured in ScaleIO.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "gateway".into(),
                    "secretRef".into(),
                    "system".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
