// Generated from definition io.k8s.api.core.v1.ISCSIPersistentVolumeSource

/// ISCSIPersistentVolumeSource represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ISCSIPersistentVolumeSource {
    /// whether support iSCSI Discovery CHAP authentication
    pub chap_auth_discovery: Option<bool>,

    /// whether support iSCSI Session CHAP authentication
    pub chap_auth_session: Option<bool>,

    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
    pub fs_type: Option<String>,

    /// Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface \<target portal\>:\<volume name\> will be created for the connection.
    pub initiator_name: Option<String>,

    /// Target iSCSI Qualified Name.
    pub iqn: String,

    /// iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
    pub iscsi_interface: Option<String>,

    /// iSCSI Target Lun number.
    pub lun: i32,

    /// iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub portals: Option<Vec<String>>,

    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    pub read_only: Option<bool>,

    /// CHAP Secret for iSCSI target and initiator authentication
    pub secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub target_portal: String,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ISCSIPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_chap_auth_discovery: Option<bool> = None;
                let mut value_chap_auth_session: Option<bool> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_initiator_name: Option<String> = None;
                let mut value_iqn: Option<String> = None;
                let mut value_iscsi_interface: Option<String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_portals: Option<Vec<String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_target_portal: Option<String> = None;

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
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ISCSIPersistentVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ISCSIPersistentVolumeSource represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "chapAuthDiscovery".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("whether support iSCSI Discovery CHAP authentication".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "chapAuthSession".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("whether support iSCSI Session CHAP authentication".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "initiatorName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "iqn".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Target iSCSI Qualified Name.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "iscsiInterface".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lun".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("iSCSI Target Lun number.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "portals".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("CHAP Secret for iSCSI target and initiator authentication".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "targetPortal".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "iqn".to_owned(),
                    "lun".to_owned(),
                    "targetPortal".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
