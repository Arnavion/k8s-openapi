// Generated from definition io.k8s.api.storage.v1.VolumeAttachmentSpec

/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentSpec {
    /// attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
    pub attacher: std::string::String,

    /// nodeName represents the node that the volume should be attached to.
    pub node_name: std::string::String,

    /// source represents the volume that should be attached.
    pub source: crate::api::storage::v1::VolumeAttachmentSource,
}

impl crate::DeepMerge for VolumeAttachmentSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.attacher, other.attacher);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.source, other.source);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeAttachmentSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attacher,
            Key_node_name,
            Key_source,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeAttachmentSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_attacher: Option<std::string::String> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_source: Option<crate::api::storage::v1::VolumeAttachmentSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attacher => value_attacher = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source => value_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentSpec {
                    attacher: value_attacher.unwrap_or_default(),
                    node_name: value_node_name.unwrap_or_default(),
                    source: value_source.unwrap_or_default(),
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

impl crate::serde::Serialize for VolumeAttachmentSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentSpec",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attacher", &self.attacher)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", &self.node_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "source", &self.source)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeAttachmentSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.VolumeAttachmentSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("VolumeAttachmentSpec is the specification of a VolumeAttachment request.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "attacher".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeName represents the node that the volume should be attached to.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "source".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::storage::v1::VolumeAttachmentSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("source represents the volume that should be attached.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "attacher".into(),
                    "nodeName".into(),
                    "source".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
