// Generated from definition io.k8s.api.storage.v1beta1.VolumeAttachmentSource

/// VolumeAttachmentSource represents a volume that should be attached. Right now only PersistenVolumes can be attached via external attacher, in future we may allow also inline volumes in pods. Exactly one member can be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentSource {
    /// Name of the persistent volume to attach.
    pub persistent_volume_name: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for VolumeAttachmentSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_persistent_volume_name,
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
                            "persistentVolumeName" => Field::Key_persistent_volume_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct VolumeAttachmentSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_persistent_volume_name: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_persistent_volume_name => value_persistent_volume_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentSource {
                    persistent_volume_name: value_persistent_volume_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeAttachmentSource",
            &[
                "persistentVolumeName",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for VolumeAttachmentSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentSource",
            0 +
            self.persistent_volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.persistent_volume_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeName", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
