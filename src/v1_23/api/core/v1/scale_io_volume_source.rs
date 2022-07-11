// Generated from definition io.k8s.api.core.v1.ScaleIOVolumeSource

/// ScaleIOVolumeSource represents a persistent ScaleIO volume
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScaleIOVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
    pub fs_type: Option<String>,

    /// The host address of the ScaleIO API Gateway.
    pub gateway: String,

    /// The name of the ScaleIO Protection Domain for the configured storage.
    pub protection_domain: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
    pub secret_ref: crate::api::core::v1::LocalObjectReference,

    /// Flag to enable/disable SSL communication with Gateway, default false
    pub ssl_enabled: Option<bool>,

    /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
    pub storage_mode: Option<String>,

    /// The ScaleIO Storage Pool associated with the protection domain.
    pub storage_pool: Option<String>,

    /// The name of the storage system as configured in ScaleIO.
    pub system: String,

    /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
    pub volume_name: Option<String>,

}

#[cfg(feature = "dsl")]
impl ScaleIOVolumeSource  {
    /// Set [`Self::fs_type`]
    pub  fn fs_type_set(&mut self, fs_type: impl Into<Option<String>>) -> &mut Self {
        self.fs_type = fs_type.into(); self
    }

    pub  fn fs_type(&mut self) -> &mut String {
        if self.fs_type.is_none() { self.fs_type = Some(Default::default()) }
        self.fs_type.as_mut().unwrap()
    }

    /// Modify [`Self::fs_type`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fs_type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.fs_type.is_none() { self.fs_type = Some(Default::default()) };
        func(self.fs_type.as_mut().unwrap()); self
    }


    /// Set [`Self::gateway`]
    pub  fn gateway_set(&mut self, gateway: impl Into<String>) -> &mut Self {
        self.gateway = gateway.into(); self
    }

    pub  fn gateway(&mut self) -> &mut String {
        &mut self.gateway
    }

    /// Modify [`Self::gateway`] with a `func`
    pub  fn gateway_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.gateway); self
    }


    /// Set [`Self::protection_domain`]
    pub  fn protection_domain_set(&mut self, protection_domain: impl Into<Option<String>>) -> &mut Self {
        self.protection_domain = protection_domain.into(); self
    }

    pub  fn protection_domain(&mut self) -> &mut String {
        if self.protection_domain.is_none() { self.protection_domain = Some(Default::default()) }
        self.protection_domain.as_mut().unwrap()
    }

    /// Modify [`Self::protection_domain`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn protection_domain_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.protection_domain.is_none() { self.protection_domain = Some(Default::default()) };
        func(self.protection_domain.as_mut().unwrap()); self
    }


    /// Set [`Self::read_only`]
    pub  fn read_only_set(&mut self, read_only: impl Into<Option<bool>>) -> &mut Self {
        self.read_only = read_only.into(); self
    }

    pub  fn read_only(&mut self) -> &mut bool {
        if self.read_only.is_none() { self.read_only = Some(Default::default()) }
        self.read_only.as_mut().unwrap()
    }

    /// Modify [`Self::read_only`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn read_only_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.read_only.is_none() { self.read_only = Some(Default::default()) };
        func(self.read_only.as_mut().unwrap()); self
    }


    /// Set [`Self::secret_ref`]
    pub  fn secret_ref_set(&mut self, secret_ref: impl Into<crate::api::core::v1::LocalObjectReference>) -> &mut Self {
        self.secret_ref = secret_ref.into(); self
    }

    pub  fn secret_ref(&mut self) -> &mut crate::api::core::v1::LocalObjectReference {
        &mut self.secret_ref
    }

    /// Modify [`Self::secret_ref`] with a `func`
    pub  fn secret_ref_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::LocalObjectReference)) -> &mut Self {
        func(&mut self.secret_ref); self
    }


    /// Set [`Self::ssl_enabled`]
    pub  fn ssl_enabled_set(&mut self, ssl_enabled: impl Into<Option<bool>>) -> &mut Self {
        self.ssl_enabled = ssl_enabled.into(); self
    }

    pub  fn ssl_enabled(&mut self) -> &mut bool {
        if self.ssl_enabled.is_none() { self.ssl_enabled = Some(Default::default()) }
        self.ssl_enabled.as_mut().unwrap()
    }

    /// Modify [`Self::ssl_enabled`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ssl_enabled_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.ssl_enabled.is_none() { self.ssl_enabled = Some(Default::default()) };
        func(self.ssl_enabled.as_mut().unwrap()); self
    }


    /// Set [`Self::storage_mode`]
    pub  fn storage_mode_set(&mut self, storage_mode: impl Into<Option<String>>) -> &mut Self {
        self.storage_mode = storage_mode.into(); self
    }

    pub  fn storage_mode(&mut self) -> &mut String {
        if self.storage_mode.is_none() { self.storage_mode = Some(Default::default()) }
        self.storage_mode.as_mut().unwrap()
    }

    /// Modify [`Self::storage_mode`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storage_mode_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.storage_mode.is_none() { self.storage_mode = Some(Default::default()) };
        func(self.storage_mode.as_mut().unwrap()); self
    }


    /// Set [`Self::storage_pool`]
    pub  fn storage_pool_set(&mut self, storage_pool: impl Into<Option<String>>) -> &mut Self {
        self.storage_pool = storage_pool.into(); self
    }

    pub  fn storage_pool(&mut self) -> &mut String {
        if self.storage_pool.is_none() { self.storage_pool = Some(Default::default()) }
        self.storage_pool.as_mut().unwrap()
    }

    /// Modify [`Self::storage_pool`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storage_pool_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.storage_pool.is_none() { self.storage_pool = Some(Default::default()) };
        func(self.storage_pool.as_mut().unwrap()); self
    }


    /// Set [`Self::system`]
    pub  fn system_set(&mut self, system: impl Into<String>) -> &mut Self {
        self.system = system.into(); self
    }

    pub  fn system(&mut self) -> &mut String {
        &mut self.system
    }

    /// Modify [`Self::system`] with a `func`
    pub  fn system_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.system); self
    }


    /// Set [`Self::volume_name`]
    pub  fn volume_name_set(&mut self, volume_name: impl Into<Option<String>>) -> &mut Self {
        self.volume_name = volume_name.into(); self
    }

    pub  fn volume_name(&mut self) -> &mut String {
        if self.volume_name.is_none() { self.volume_name = Some(Default::default()) }
        self.volume_name.as_mut().unwrap()
    }

    /// Modify [`Self::volume_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volume_name.is_none() { self.volume_name = Some(Default::default()) };
        func(self.volume_name.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ScaleIOVolumeSource {
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            type Value = ScaleIOVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ScaleIOVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_gateway: Option<String> = None;
                let mut value_protection_domain: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_ssl_enabled: Option<bool> = None;
                let mut value_storage_mode: Option<String> = None;
                let mut value_storage_pool: Option<String> = None;
                let mut value_system: Option<String> = None;
                let mut value_volume_name: Option<String> = None;

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

                Ok(ScaleIOVolumeSource {
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
            "ScaleIOVolumeSource",
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

impl crate::serde::Serialize for ScaleIOVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScaleIOVolumeSource",
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
impl crate::schemars::JsonSchema for ScaleIOVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ScaleIOVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ScaleIOVolumeSource represents a persistent ScaleIO volume".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fsType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Default is \"xfs\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gateway".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The host address of the ScaleIO API Gateway.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "protectionDomain".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The name of the ScaleIO Protection Domain for the configured storage.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "sslEnabled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Flag to enable/disable SSL communication with Gateway, default false".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storageMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storagePool".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The ScaleIO Storage Pool associated with the protection domain.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "system".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The name of the storage system as configured in ScaleIO.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The name of a volume already created in the ScaleIO system that is associated with this volume source.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "gateway".to_owned(),
                    "secretRef".to_owned(),
                    "system".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
