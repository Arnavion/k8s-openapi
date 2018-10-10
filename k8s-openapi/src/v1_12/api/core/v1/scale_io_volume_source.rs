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
    pub secret_ref: ::v1_12::api::core::v1::LocalObjectReference,

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

impl<'de> ::serde::Deserialize<'de> for ScaleIOVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScaleIOVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ScaleIOVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_gateway: Option<String> = None;
                let mut value_protection_domain: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<::v1_12::api::core::v1::LocalObjectReference> = None;
                let mut value_ssl_enabled: Option<bool> = None;
                let mut value_storage_mode: Option<String> = None;
                let mut value_storage_pool: Option<String> = None;
                let mut value_system: Option<String> = None;
                let mut value_volume_name: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gateway => value_gateway = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_protection_domain => value_protection_domain = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ssl_enabled => value_ssl_enabled = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_mode => value_storage_mode = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_pool => value_storage_pool = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_system => value_system = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_volume_name => value_volume_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScaleIOVolumeSource {
                    fs_type: value_fs_type,
                    gateway: value_gateway.ok_or_else(|| ::serde::de::Error::missing_field("gateway"))?,
                    protection_domain: value_protection_domain,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref.ok_or_else(|| ::serde::de::Error::missing_field("secretRef"))?,
                    ssl_enabled: value_ssl_enabled,
                    storage_mode: value_storage_mode,
                    storage_pool: value_storage_pool,
                    system: value_system.ok_or_else(|| ::serde::de::Error::missing_field("system"))?,
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

impl ::serde::Serialize for ScaleIOVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScaleIOVolumeSource",
            0 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            1 +
            self.protection_domain.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            1 +
            self.ssl_enabled.as_ref().map_or(0, |_| 1) +
            self.storage_mode.as_ref().map_or(0, |_| 1) +
            self.storage_pool.as_ref().map_or(0, |_| 1) +
            1 +
            self.volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "gateway", &self.gateway)?;
        if let Some(value) = &self.protection_domain {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "protectionDomain", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", &self.secret_ref)?;
        if let Some(value) = &self.ssl_enabled {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "sslEnabled", value)?;
        }
        if let Some(value) = &self.storage_mode {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "storageMode", value)?;
        }
        if let Some(value) = &self.storage_pool {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "storagePool", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "system", &self.system)?;
        if let Some(value) = &self.volume_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
