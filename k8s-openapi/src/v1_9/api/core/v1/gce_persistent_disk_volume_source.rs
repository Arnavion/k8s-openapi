// Generated from definition io.k8s.api.core.v1.GCEPersistentDiskVolumeSource

/// Represents a Persistent Disk resource in Google Compute Engine.
///
/// A GCE PD must exist before mounting to a container. The disk must also be in the same GCE project and zone as the kubelet. A GCE PD can only be mounted as read/write once or read-only many times. GCE PDs support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GCEPersistentDiskVolumeSource {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub fs_type: Option<String>,

    /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub partition: Option<i32>,

    /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub pd_name: String,

    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub read_only: Option<bool>,
}

impl<'de> ::serde::Deserialize<'de> for GCEPersistentDiskVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_partition,
            Key_pd_name,
            Key_read_only,
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
                            "partition" => Field::Key_partition,
                            "pdName" => Field::Key_pd_name,
                            "readOnly" => Field::Key_read_only,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GCEPersistentDiskVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct GCEPersistentDiskVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_partition: Option<i32> = None;
                let mut value_pd_name: Option<String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_partition => value_partition = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pd_name => value_pd_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GCEPersistentDiskVolumeSource {
                    fs_type: value_fs_type,
                    partition: value_partition,
                    pd_name: value_pd_name.ok_or_else(|| ::serde::de::Error::missing_field("pdName"))?,
                    read_only: value_read_only,
                })
            }
        }

        deserializer.deserialize_struct(
            "GCEPersistentDiskVolumeSource",
            &[
                "fsType",
                "partition",
                "pdName",
                "readOnly",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for GCEPersistentDiskVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GCEPersistentDiskVolumeSource",
            0 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.partition.as_ref().map_or(0, |_| 1) +
            1 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.partition {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "partition", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "pdName", &self.pd_name)?;
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
