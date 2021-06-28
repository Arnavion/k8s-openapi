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
                        Field::Key_gateway => value_gateway = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_protection_domain => value_protection_domain = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ssl_enabled => value_ssl_enabled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_mode => value_storage_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_pool => value_storage_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_system => value_system = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScaleIOVolumeSource {
                    fs_type: value_fs_type,
                    gateway: value_gateway.ok_or_else(|| crate::serde::de::Error::missing_field("gateway"))?,
                    protection_domain: value_protection_domain,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref.ok_or_else(|| crate::serde::de::Error::missing_field("secretRef"))?,
                    ssl_enabled: value_ssl_enabled,
                    storage_mode: value_storage_mode,
                    storage_pool: value_storage_pool,
                    system: value_system.ok_or_else(|| crate::serde::de::Error::missing_field("system"))?,
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

#[cfg(feature = "schema")]
impl crate::Schema for ScaleIOVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ScaleIOVolumeSource represents a persistent ScaleIO volume",
          "properties": {
            "fsType": {
              "description": "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Default is \"xfs\".",
              "type": "string"
            },
            "gateway": {
              "description": "The host address of the ScaleIO API Gateway.",
              "type": "string"
            },
            "protectionDomain": {
              "description": "The name of the ScaleIO Protection Domain for the configured storage.",
              "type": "string"
            },
            "readOnly": {
              "description": "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
              "type": "boolean"
            },
            "secretRef": crate::schema_ref_with_description(crate::api::core::v1::LocalObjectReference::schema(), "SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail."),
            "sslEnabled": {
              "description": "Flag to enable/disable SSL communication with Gateway, default false",
              "type": "boolean"
            },
            "storageMode": {
              "description": "Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.",
              "type": "string"
            },
            "storagePool": {
              "description": "The ScaleIO Storage Pool associated with the protection domain.",
              "type": "string"
            },
            "system": {
              "description": "The name of the storage system as configured in ScaleIO.",
              "type": "string"
            },
            "volumeName": {
              "description": "The name of a volume already created in the ScaleIO system that is associated with this volume source.",
              "type": "string"
            }
          },
          "required": [
            "gateway",
            "secretRef",
            "system"
          ],
          "type": "object"
        })
    }
}
