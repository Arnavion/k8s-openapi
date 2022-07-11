// Generated from definition io.k8s.api.storage.v1alpha1.VolumeAttachmentSource

/// VolumeAttachmentSource represents a volume that should be attached. Right now only PersistenVolumes can be attached via external attacher, in future we may allow also inline volumes in pods. Exactly one member can be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentSource {
    /// inlineVolumeSpec contains all the information necessary to attach a persistent volume defined by a pod's inline VolumeSource. This field is populated only for the CSIMigration feature. It contains translated fields from a pod's inline VolumeSource to a PersistentVolumeSpec. This field is alpha-level and is only honored by servers that enabled the CSIMigration feature.
    pub inline_volume_spec: Option<crate::api::core::v1::PersistentVolumeSpec>,

    /// Name of the persistent volume to attach.
    pub persistent_volume_name: Option<String>,

}

#[cfg(feature = "dsl")]
impl VolumeAttachmentSource  {
    /// Set [`Self::inline_volume_spec`]
    pub  fn inline_volume_spec_set(&mut self, inline_volume_spec: impl Into<Option<crate::api::core::v1::PersistentVolumeSpec>>) -> &mut Self {
        self.inline_volume_spec = inline_volume_spec.into(); self
    }

    pub  fn inline_volume_spec(&mut self) -> &mut crate::api::core::v1::PersistentVolumeSpec {
        if self.inline_volume_spec.is_none() { self.inline_volume_spec = Some(Default::default()) }
        self.inline_volume_spec.as_mut().unwrap()
    }

    /// Modify [`Self::inline_volume_spec`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn inline_volume_spec_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PersistentVolumeSpec)) -> &mut Self {
        if self.inline_volume_spec.is_none() { self.inline_volume_spec = Some(Default::default()) };
        func(self.inline_volume_spec.as_mut().unwrap()); self
    }


    /// Set [`Self::persistent_volume_name`]
    pub  fn persistent_volume_name_set(&mut self, persistent_volume_name: impl Into<Option<String>>) -> &mut Self {
        self.persistent_volume_name = persistent_volume_name.into(); self
    }

    pub  fn persistent_volume_name(&mut self) -> &mut String {
        if self.persistent_volume_name.is_none() { self.persistent_volume_name = Some(Default::default()) }
        self.persistent_volume_name.as_mut().unwrap()
    }

    /// Modify [`Self::persistent_volume_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn persistent_volume_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.persistent_volume_name.is_none() { self.persistent_volume_name = Some(Default::default()) };
        func(self.persistent_volume_name.as_mut().unwrap()); self
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeAttachmentSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_inline_volume_spec: Option<crate::api::core::v1::PersistentVolumeSpec> = None;
                let mut value_persistent_volume_name: Option<String> = None;

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
    fn schema_name() -> String {
        "io.k8s.api.storage.v1alpha1.VolumeAttachmentSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VolumeAttachmentSource represents a volume that should be attached. Right now only PersistenVolumes can be attached via external attacher, in future we may allow also inline volumes in pods. Exactly one member can be set.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "inlineVolumeSpec".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PersistentVolumeSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("inlineVolumeSpec contains all the information necessary to attach a persistent volume defined by a pod's inline VolumeSource. This field is populated only for the CSIMigration feature. It contains translated fields from a pod's inline VolumeSource to a PersistentVolumeSpec. This field is alpha-level and is only honored by servers that enabled the CSIMigration feature.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "persistentVolumeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the persistent volume to attach.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
