// Generated from definition io.k8s.api.core.v1.VolumeStatus

/// VolumeStatus represents the status of a mounted volume. At most one of its members must be specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeStatus {
    /// image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.
    pub image: Option<crate::api::core::v1::ImageVolumeStatus>,
}

impl crate::DeepMerge for VolumeStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.image, other.image);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_image,
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
                            "image" => Field::Key_image,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_image: Option<crate::api::core::v1::ImageVolumeStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeStatus {
                    image: value_image,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeStatus",
            &[
                "image",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeStatus",
            self.image.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.image {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.VolumeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "VolumeStatus represents the status of a mounted volume. At most one of its members must be specified.",
            "type": "object",
            "properties": {
                "image": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ImageVolumeStatus>();
                    schema_obj.ensure_object().insert("description".into(), "image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for VolumeStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.VolumeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("VolumeStatus represents the status of a mounted volume. At most one of its members must be specified.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "image".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ImageVolumeStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
