// Generated from definition io.k8s.api.core.v1.FCVolumeSource

/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FCVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// Optional: FC target lun number
    pub lun: Option<i32>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// Optional: FC target worldwide names (WWNs)
    pub target_wwns: Option<Vec<String>>,

    /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    pub wwids: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for FCVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_lun,
            Key_read_only,
            Key_target_wwns,
            Key_wwids,
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
                            "lun" => Field::Key_lun,
                            "readOnly" => Field::Key_read_only,
                            "targetWWNs" => Field::Key_target_wwns,
                            "wwids" => Field::Key_wwids,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FCVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct FCVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_target_wwns: Option<Vec<String>> = None;
                let mut value_wwids: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lun => value_lun = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_wwns => value_target_wwns = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_wwids => value_wwids = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FCVolumeSource {
                    fs_type: value_fs_type,
                    lun: value_lun,
                    read_only: value_read_only,
                    target_wwns: value_target_wwns,
                    wwids: value_wwids,
                })
            }
        }

        deserializer.deserialize_struct(
            "FCVolumeSource",
            &[
                "fsType",
                "lun",
                "readOnly",
                "targetWWNs",
                "wwids",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for FCVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FCVolumeSource",
            0 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.lun.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.target_wwns.as_ref().map_or(0, |_| 1) +
            self.wwids.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.lun {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "lun", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.target_wwns {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "targetWWNs", value)?;
        }
        if let Some(value) = &self.wwids {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "wwids", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
