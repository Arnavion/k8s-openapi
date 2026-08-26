// Generated from definition io.k8s.api.storage.v1.CSINodeStatus

/// CSINodeStatus contains health and status information for storage on a node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeStatus {
    /// storageHealth contains backend health reports for CSI drivers registered on the node.
    pub storage_health: Option<std::vec::Vec<crate::api::storage::v1::StorageHealth>>,
}

impl crate::DeepMerge for CSINodeStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.storage_health,
            other.storage_health,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for CSINodeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_storage_health,
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
                            "storageHealth" => Field::Key_storage_health,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CSINodeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_storage_health: Option<std::vec::Vec<crate::api::storage::v1::StorageHealth>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_storage_health => value_storage_health = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeStatus {
                    storage_health: value_storage_health,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINodeStatus",
            &[
                "storageHealth",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSINodeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeStatus",
            self.storage_health.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.storage_health {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageHealth", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSINodeStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.CSINodeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CSINodeStatus contains health and status information for storage on a node.",
            "type": "object",
            "properties": {
                "storageHealth": {
                    "description": "storageHealth contains backend health reports for CSI drivers registered on the node.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::storage::v1::StorageHealth>()),
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for CSINodeStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.CSINodeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("CSINodeStatus contains health and status information for storage on a node.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "storageHealth".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("storageHealth contains backend health reports for CSI drivers registered on the node.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::storage::v1::StorageHealth>()))),
                                ..Default::default()
                            })),
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
