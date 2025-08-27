// Generated from definition io.k8s.api.core.v1.ISCSIPersistentVolumeSource

/// ISCSIPersistentVolumeSource represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ISCSIPersistentVolumeSource {
    /// chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication
    pub chap_auth_discovery: Option<bool>,

    /// chapAuthSession defines whether support iSCSI Session CHAP authentication
    pub chap_auth_session: Option<bool>,

    /// fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
    pub fs_type: Option<std::string::String>,

    /// initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface \<target portal\>:\<volume name\> will be created for the connection.
    pub initiator_name: Option<std::string::String>,

    /// iqn is Target iSCSI Qualified Name.
    pub iqn: std::string::String,

    /// iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
    pub iscsi_interface: Option<std::string::String>,

    /// lun is iSCSI Target Lun number.
    pub lun: i32,

    /// portals is the iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub portals: Option<std::vec::Vec<std::string::String>>,

    /// readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    pub read_only: Option<bool>,

    /// secretRef is the CHAP Secret for iSCSI target and initiator authentication
    pub secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub target_portal: std::string::String,
}

impl crate::DeepMerge for ISCSIPersistentVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.chap_auth_discovery, other.chap_auth_discovery);
        crate::DeepMerge::merge_from(&mut self.chap_auth_session, other.chap_auth_session);
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.initiator_name, other.initiator_name);
        crate::DeepMerge::merge_from(&mut self.iqn, other.iqn);
        crate::DeepMerge::merge_from(&mut self.iscsi_interface, other.iscsi_interface);
        crate::DeepMerge::merge_from(&mut self.lun, other.lun);
        crate::merge_strategies::list::atomic(&mut self.portals, other.portals);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
        crate::DeepMerge::merge_from(&mut self.target_portal, other.target_portal);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ISCSIPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_chap_auth_discovery,
            Key_chap_auth_session,
            Key_fs_type,
            Key_initiator_name,
            Key_iqn,
            Key_iscsi_interface,
            Key_lun,
            Key_portals,
            Key_read_only,
            Key_secret_ref,
            Key_target_portal,
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
                            "chapAuthDiscovery" => Field::Key_chap_auth_discovery,
                            "chapAuthSession" => Field::Key_chap_auth_session,
                            "fsType" => Field::Key_fs_type,
                            "initiatorName" => Field::Key_initiator_name,
                            "iqn" => Field::Key_iqn,
                            "iscsiInterface" => Field::Key_iscsi_interface,
                            "lun" => Field::Key_lun,
                            "portals" => Field::Key_portals,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            "targetPortal" => Field::Key_target_portal,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ISCSIPersistentVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ISCSIPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_chap_auth_discovery: Option<bool> = None;
                let mut value_chap_auth_session: Option<bool> = None;
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_initiator_name: Option<std::string::String> = None;
                let mut value_iqn: Option<std::string::String> = None;
                let mut value_iscsi_interface: Option<std::string::String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_portals: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_target_portal: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_chap_auth_discovery => value_chap_auth_discovery = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_chap_auth_session => value_chap_auth_session = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_initiator_name => value_initiator_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iqn => value_iqn = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iscsi_interface => value_iscsi_interface = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lun => value_lun = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_portals => value_portals = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_portal => value_target_portal = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ISCSIPersistentVolumeSource {
                    chap_auth_discovery: value_chap_auth_discovery,
                    chap_auth_session: value_chap_auth_session,
                    fs_type: value_fs_type,
                    initiator_name: value_initiator_name,
                    iqn: value_iqn.unwrap_or_default(),
                    iscsi_interface: value_iscsi_interface,
                    lun: value_lun.unwrap_or_default(),
                    portals: value_portals,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                    target_portal: value_target_portal.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ISCSIPersistentVolumeSource",
            &[
                "chapAuthDiscovery",
                "chapAuthSession",
                "fsType",
                "initiatorName",
                "iqn",
                "iscsiInterface",
                "lun",
                "portals",
                "readOnly",
                "secretRef",
                "targetPortal",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ISCSIPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ISCSIPersistentVolumeSource",
            3 +
            self.chap_auth_discovery.as_ref().map_or(0, |_| 1) +
            self.chap_auth_session.as_ref().map_or(0, |_| 1) +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.initiator_name.as_ref().map_or(0, |_| 1) +
            self.iscsi_interface.as_ref().map_or(0, |_| 1) +
            self.portals.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.chap_auth_discovery {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "chapAuthDiscovery", value)?;
        }
        if let Some(value) = &self.chap_auth_session {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "chapAuthSession", value)?;
        }
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.initiator_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initiatorName", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "iqn", &self.iqn)?;
        if let Some(value) = &self.iscsi_interface {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "iscsiInterface", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lun", &self.lun)?;
        if let Some(value) = &self.portals {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portals", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetPortal", &self.target_portal)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ISCSIPersistentVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ISCSIPersistentVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ISCSIPersistentVolumeSource represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.",
            "type": "object",
            "properties": {
                "chapAuthDiscovery": {
                    "description": "chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication",
                    "type": "boolean",
                },
                "chapAuthSession": {
                    "description": "chapAuthSession defines whether support iSCSI Session CHAP authentication",
                    "type": "boolean",
                },
                "fsType": {
                    "description": "fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi",
                    "type": "string",
                },
                "initiatorName": {
                    "description": "initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.",
                    "type": "string",
                },
                "iqn": {
                    "description": "iqn is Target iSCSI Qualified Name.",
                    "type": "string",
                },
                "iscsiInterface": {
                    "description": "iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).",
                    "type": "string",
                },
                "lun": {
                    "description": "lun is iSCSI Target Lun number.",
                    "type": "integer",
                    "format": "int32",
                },
                "portals": {
                    "description": "portals is the iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "readOnly": {
                    "description": "readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.",
                    "type": "boolean",
                },
                "secretRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>();
                    schema_obj.ensure_object().insert("description".into(), "secretRef is the CHAP Secret for iSCSI target and initiator authentication".into());
                    schema_obj
                }),
                "targetPortal": {
                    "description": "targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).",
                    "type": "string",
                },
            },
            "required": [
                "iqn",
                "lun",
                "targetPortal",
            ],
        })
    }
}
