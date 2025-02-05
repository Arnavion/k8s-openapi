// Generated from definition io.k8s.api.storage.v1.VolumeAttachmentSource

/// VolumeAttachmentSource represents a volume that should be attached. Right now only PersistentVolumes can be attached via external attacher, in the future we may allow also inline volumes in pods. Exactly one member can be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentSource {
    /// inlineVolumeSpec contains all the information necessary to attach a persistent volume defined by a pod's inline VolumeSource. This field is populated only for the CSIMigration feature. It contains translated fields from a pod's inline VolumeSource to a PersistentVolumeSpec. This field is beta-level and is only honored by servers that enabled the CSIMigration feature.
    pub inline_volume_spec: Option<crate::api::core::v1::PersistentVolumeSpec>,

    /// persistentVolumeName represents the name of the persistent volume to attach.
    pub persistent_volume_name: Option<std::string::String>,
}

impl crate::DeepMerge for VolumeAttachmentSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.inline_volume_spec, other.inline_volume_spec);
        crate::DeepMerge::merge_from(&mut self.persistent_volume_name, other.persistent_volume_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeAttachmentSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_inline_volume_spec,
            Key_persistent_volume_name,
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
                            "inlineVolumeSpec" => Field::Key_inline_volume_spec,
                            "persistentVolumeName" => Field::Key_persistent_volume_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeAttachmentSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_inline_volume_spec: Option<crate::api::core::v1::PersistentVolumeSpec> = None;
                let mut value_persistent_volume_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_inline_volume_spec => value_inline_volume_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_name => value_persistent_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentSource {
                    inline_volume_spec: value_inline_volume_spec,
                    persistent_volume_name: value_persistent_volume_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeAttachmentSource",
            &[
                "inlineVolumeSpec",
                "persistentVolumeName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeAttachmentSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentSource",
            self.inline_volume_spec.as_ref().map_or(0, |_| 1) +
            self.persistent_volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.inline_volume_spec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "inlineVolumeSpec", value)?;
        }
        if let Some(value) = &self.persistent_volume_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeAttachmentSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.VolumeAttachmentSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("VolumeAttachmentSource represents a volume that should be attached. Right now only PersistentVolumes can be attached via external attacher, in the future we may allow also inline volumes in pods. Exactly one member can be set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "inlineVolumeSpec".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PersistentVolumeSpec>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("inlineVolumeSpec contains all the information necessary to attach a persistent volume defined by a pod's inline VolumeSource. This field is populated only for the CSIMigration feature. It contains translated fields from a pod's inline VolumeSource to a PersistentVolumeSpec. This field is beta-level and is only honored by servers that enabled the CSIMigration feature.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "persistentVolumeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("persistentVolumeName represents the name of the persistent volume to attach.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
