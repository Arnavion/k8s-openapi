// Generated from definition io.k8s.api.storage.v1.StorageHealth

/// StorageHealth contains storage backend health reported by a CSI driver on a node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageHealth {
    /// healthConditions are the adverse storage backend conditions reported by the CSI driver. At most 16 conditions may be reported.
    pub health_conditions: Option<std::vec::Vec<crate::api::storage::v1::StorageHealthCondition>>,

    /// name is the CSI driver name, matching CSINodeDriver.name.
    pub name: std::string::String,
}

impl crate::DeepMerge for StorageHealth {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.health_conditions, other.health_conditions);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageHealth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_health_conditions,
            Key_name,
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
                            "healthConditions" => Field::Key_health_conditions,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageHealth;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageHealth")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_health_conditions: Option<std::vec::Vec<crate::api::storage::v1::StorageHealthCondition>> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_health_conditions => value_health_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageHealth {
                    health_conditions: value_health_conditions,
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageHealth",
            &[
                "healthConditions",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageHealth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageHealth",
            1 +
            self.health_conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.health_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "healthConditions", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageHealth {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.StorageHealth".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "StorageHealth contains storage backend health reported by a CSI driver on a node.",
            "type": "object",
            "properties": {
                "healthConditions": {
                    "description": "healthConditions are the adverse storage backend conditions reported by the CSI driver. At most 16 conditions may be reported.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::storage::v1::StorageHealthCondition>()),
                },
                "name": {
                    "description": "name is the CSI driver name, matching CSINodeDriver.name.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for StorageHealth {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.StorageHealth".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("StorageHealth contains storage backend health reported by a CSI driver on a node.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "healthConditions".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("healthConditions are the adverse storage backend conditions reported by the CSI driver. At most 16 conditions may be reported.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::storage::v1::StorageHealthCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("name is the CSI driver name, matching CSINodeDriver.name.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
