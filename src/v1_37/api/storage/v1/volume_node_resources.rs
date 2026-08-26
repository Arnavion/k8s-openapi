// Generated from definition io.k8s.api.storage.v1.VolumeNodeResources

/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeNodeResources {
    /// count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.
    pub count: Option<i32>,
}

impl crate::DeepMerge for VolumeNodeResources {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.count, other.count);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeNodeResources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_count,
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
                            "count" => Field::Key_count,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeNodeResources;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeNodeResources")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_count: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_count => value_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeNodeResources {
                    count: value_count,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeNodeResources",
            &[
                "count",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeNodeResources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeNodeResources",
            self.count.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "count", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeNodeResources {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.VolumeNodeResources".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "VolumeNodeResources is a set of resource limits for scheduling of volumes.",
            "type": "object",
            "properties": {
                "count": {
                    "description": "count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.",
                    "type": "integer",
                    "format": "int32",
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for VolumeNodeResources {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.VolumeNodeResources".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("VolumeNodeResources is a set of resource limits for scheduling of volumes.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "count".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
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
