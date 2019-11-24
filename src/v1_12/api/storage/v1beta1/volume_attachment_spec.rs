// Generated from definition io.k8s.api.storage.v1beta1.VolumeAttachmentSpec

/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentSpec {
    /// Attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
    pub attacher: String,

    /// The node that the volume should be attached to.
    pub node_name: String,

    /// Source represents the volume that should be attached.
    pub source: crate::api::storage::v1beta1::VolumeAttachmentSource,
}

impl<'de> serde::Deserialize<'de> for VolumeAttachmentSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attacher,
            Key_node_name,
            Key_source,
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
                            "attacher" => Field::Key_attacher,
                            "nodeName" => Field::Key_node_name,
                            "source" => Field::Key_source,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeAttachmentSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_attacher: Option<String> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_source: Option<crate::api::storage::v1beta1::VolumeAttachmentSource> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attacher => value_attacher = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_node_name => value_node_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_source => value_source = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentSpec {
                    attacher: value_attacher.ok_or_else(|| serde::de::Error::missing_field("attacher"))?,
                    node_name: value_node_name.ok_or_else(|| serde::de::Error::missing_field("nodeName"))?,
                    source: value_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeAttachmentSpec",
            &[
                "attacher",
                "nodeName",
                "source",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for VolumeAttachmentSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentSpec",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "attacher", &self.attacher)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", &self.node_name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "source", &self.source)?;
        serde::ser::SerializeStruct::end(state)
    }
}
