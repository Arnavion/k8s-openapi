// Generated from definition io.k8s.api.core.v1.LocalVolumeSource

/// Local represents directly-attached storage with node affinity (Beta feature)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalVolumeSource {
    /// Filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default value is to auto-select a fileystem if unspecified.
    pub fs_type: Option<String>,

    /// The full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
    pub path: String,
}

impl<'de> serde::Deserialize<'de> for LocalVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_path,
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
                            "path" => Field::Key_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LocalVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LocalVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_path: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LocalVolumeSource {
                    fs_type: value_fs_type,
                    path: value_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "LocalVolumeSource",
            &[
                "fsType",
                "path",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for LocalVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LocalVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        serde::ser::SerializeStruct::end(state)
    }
}
