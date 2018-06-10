// Generated from definition io.k8s.kubernetes.pkg.api.v1.ISCSIVolumeSource

/// Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ISCSIVolumeSource {
    /// whether support iSCSI Discovery CHAP authentication
    pub chap_auth_discovery: Option<bool>,

    /// whether support iSCSI Session CHAP authentication
    pub chap_auth_session: Option<bool>,

    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
    pub fs_type: Option<String>,

    /// Target iSCSI Qualified Name.
    pub iqn: String,

    /// Optional: Defaults to 'default' (tcp). iSCSI interface name that uses an iSCSI transport.
    pub iscsi_interface: Option<String>,

    /// iSCSI target lun number.
    pub lun: i32,

    /// iSCSI target portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub portals: Option<Vec<String>>,

    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    pub read_only: Option<bool>,

    /// CHAP secret for iSCSI target and initiator authentication
    pub secret_ref: Option<::v1_7::kubernetes::pkg::api::v1::LocalObjectReference>,

    /// iSCSI target portal. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub target_portal: String,
}

impl<'de> ::serde::Deserialize<'de> for ISCSIVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_chap_auth_discovery,
            Key_chap_auth_session,
            Key_fs_type,
            Key_iqn,
            Key_iscsi_interface,
            Key_lun,
            Key_portals,
            Key_read_only,
            Key_secret_ref,
            Key_target_portal,
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
                            "chapAuthDiscovery" => Field::Key_chap_auth_discovery,
                            "chapAuthSession" => Field::Key_chap_auth_session,
                            "fsType" => Field::Key_fs_type,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ISCSIVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ISCSIVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_chap_auth_discovery: Option<bool> = None;
                let mut value_chap_auth_session: Option<bool> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_iqn: Option<String> = None;
                let mut value_iscsi_interface: Option<String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_portals: Option<Vec<String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<::v1_7::kubernetes::pkg::api::v1::LocalObjectReference> = None;
                let mut value_target_portal: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_chap_auth_discovery => value_chap_auth_discovery = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_chap_auth_session => value_chap_auth_session = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iqn => value_iqn = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_iscsi_interface => value_iscsi_interface = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lun => value_lun = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_portals => value_portals = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_portal => value_target_portal = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ISCSIVolumeSource {
                    chap_auth_discovery: value_chap_auth_discovery,
                    chap_auth_session: value_chap_auth_session,
                    fs_type: value_fs_type,
                    iqn: value_iqn.ok_or_else(|| ::serde::de::Error::missing_field("iqn"))?,
                    iscsi_interface: value_iscsi_interface,
                    lun: value_lun.ok_or_else(|| ::serde::de::Error::missing_field("lun"))?,
                    portals: value_portals,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                    target_portal: value_target_portal.ok_or_else(|| ::serde::de::Error::missing_field("targetPortal"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ISCSIVolumeSource",
            &[
                "chapAuthDiscovery",
                "chapAuthSession",
                "fsType",
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

impl ::serde::Serialize for ISCSIVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ISCSIVolumeSource",
            0 +
            (if self.chap_auth_discovery.is_some() { 1 } else { 0 }) +
            (if self.chap_auth_session.is_some() { 1 } else { 0 }) +
            (if self.fs_type.is_some() { 1 } else { 0 }) +
            1 +
            (if self.iscsi_interface.is_some() { 1 } else { 0 }) +
            1 +
            (if self.portals.is_some() { 1 } else { 0 }) +
            (if self.read_only.is_some() { 1 } else { 0 }) +
            (if self.secret_ref.is_some() { 1 } else { 0 }) +
            1,
        )?;
        if let Some(value) = &self.chap_auth_discovery {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "chapAuthDiscovery", value)?;
        }
        if let Some(value) = &self.chap_auth_session {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "chapAuthSession", value)?;
        }
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "iqn", &self.iqn)?;
        if let Some(value) = &self.iscsi_interface {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "iscsiInterface", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "lun", &self.lun)?;
        if let Some(value) = &self.portals {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "portals", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "targetPortal", &self.target_portal)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
