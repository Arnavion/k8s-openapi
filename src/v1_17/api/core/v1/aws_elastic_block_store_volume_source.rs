// Generated from definition io.k8s.api.core.v1.AWSElasticBlockStoreVolumeSource

/// Represents a Persistent Disk resource in AWS.
///
/// An AWS EBS disk must exist before mounting to a container. The disk must also be in the same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS EBS volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AWSElasticBlockStoreVolumeSource {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub fs_type: Option<String>,

    /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
    pub partition: Option<i32>,

    /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If omitted, the default is "false". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub read_only: Option<bool>,

    /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub volume_id: String,
}

impl<'de> serde::Deserialize<'de> for AWSElasticBlockStoreVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_partition,
            Key_read_only,
            Key_volume_id,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "fsType" => Field::Key_fs_type,
                            "partition" => Field::Key_partition,
                            "readOnly" => Field::Key_read_only,
                            "volumeID" => Field::Key_volume_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AWSElasticBlockStoreVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AWSElasticBlockStoreVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_partition: Option<i32> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_id: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_partition => value_partition = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_id => value_volume_id = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AWSElasticBlockStoreVolumeSource {
                    fs_type: value_fs_type,
                    partition: value_partition,
                    read_only: value_read_only,
                    volume_id: value_volume_id.ok_or_else(|| serde::de::Error::missing_field("volumeID"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "AWSElasticBlockStoreVolumeSource",
            &[
                "fsType",
                "partition",
                "readOnly",
                "volumeID",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for AWSElasticBlockStoreVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AWSElasticBlockStoreVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.partition.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.partition {
            serde::ser::SerializeStruct::serialize_field(&mut state, "partition", value)?;
        }
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "volumeID", &self.volume_id)?;
        serde::ser::SerializeStruct::end(state)
    }
}
