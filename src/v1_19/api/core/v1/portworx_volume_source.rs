// Generated from definition io.k8s.api.core.v1.PortworxVolumeSource

/// PortworxVolumeSource represents a Portworx volume resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PortworxVolumeSource {
    /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// VolumeID uniquely identifies a Portworx volume
    pub volume_id: String,
}

impl<'de> serde::Deserialize<'de> for PortworxVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
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
            type Value = PortworxVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PortworxVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_id: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_id => value_volume_id = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PortworxVolumeSource {
                    fs_type: value_fs_type,
                    read_only: value_read_only,
                    volume_id: value_volume_id.ok_or_else(|| serde::de::Error::missing_field("volumeID"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PortworxVolumeSource",
            &[
                "fsType",
                "readOnly",
                "volumeID",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PortworxVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PortworxVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "volumeID", &self.volume_id)?;
        serde::ser::SerializeStruct::end(state)
    }
}
