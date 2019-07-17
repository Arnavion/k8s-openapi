// Generated from definition io.k8s.api.storage.v1beta1.VolumeAttachmentList

/// VolumeAttachmentList is a collection of VolumeAttachment objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentList {
    /// Items is the list of VolumeAttachments
    pub items: Vec<crate::v1_15::api::storage::v1beta1::VolumeAttachment>,

    /// Standard list metadata More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_15::apimachinery::pkg::apis::meta::v1::ListMeta>,
}

impl crate::Resource for VolumeAttachmentList {
    fn api_version() -> &'static str {
        "storage.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "storage.k8s.io"
    }

    fn kind() -> &'static str {
        "VolumeAttachmentList"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for VolumeAttachmentList {
    type Ty = crate::v1_15::apimachinery::pkg::apis::meta::v1::ListMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for VolumeAttachmentList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_items,
            Key_metadata,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "items" => Field::Key_items,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentList;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct VolumeAttachmentList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_items: Option<Vec<crate::v1_15::api::storage::v1beta1::VolumeAttachment>> = None;
                let mut value_metadata: Option<crate::v1_15::apimachinery::pkg::apis::meta::v1::ListMeta> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_items => value_items = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentList {
                    items: value_items.ok_or_else(|| serde::de::Error::missing_field("items"))?,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeAttachmentList",
            &[
                "apiVersion",
                "kind",
                "items",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for VolumeAttachmentList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentList",
            3 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
