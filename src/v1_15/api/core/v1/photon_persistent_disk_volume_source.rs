// Generated from definition io.k8s.api.core.v1.PhotonPersistentDiskVolumeSource

/// Represents a Photon Controller persistent disk resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PhotonPersistentDiskVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// ID that identifies Photon Controller persistent disk
    pub pd_id: String,
}

impl<'de> crate::serde::Deserialize<'de> for PhotonPersistentDiskVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_pd_id,
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
                            "pdID" => Field::Key_pd_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PhotonPersistentDiskVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PhotonPersistentDiskVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_pd_id: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pd_id => value_pd_id = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PhotonPersistentDiskVolumeSource {
                    fs_type: value_fs_type,
                    pd_id: value_pd_id.ok_or_else(|| crate::serde::de::Error::missing_field("pdID"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PhotonPersistentDiskVolumeSource",
            &[
                "fsType",
                "pdID",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PhotonPersistentDiskVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PhotonPersistentDiskVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pdID", &self.pd_id)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PhotonPersistentDiskVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents a Photon Controller persistent disk resource.",
          "properties": {
            "fsType": {
              "description": "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.",
              "type": "string"
            },
            "pdID": {
              "description": "ID that identifies Photon Controller persistent disk",
              "type": "string"
            }
          },
          "required": [
            "pdID"
          ],
          "type": "object"
        })
    }
}
