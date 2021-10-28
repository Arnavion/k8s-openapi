// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Initializers

/// Initializers tracks the progress of initialization.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Initializers {
    /// Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.
    pub pending: Vec<crate::apimachinery::pkg::apis::meta::v1::Initializer>,

    /// If result is set with the Failure field, the object will be persisted to storage and then deleted, ensuring that other clients can observe the deletion.
    pub result: Option<crate::apimachinery::pkg::apis::meta::v1::Status>,
}

impl<'de> crate::serde::Deserialize<'de> for Initializers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pending,
            Key_result,
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
                            "pending" => Field::Key_pending,
                            "result" => Field::Key_result,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Initializers;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Initializers")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_pending: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::Initializer>> = None;
                let mut value_result: Option<crate::apimachinery::pkg::apis::meta::v1::Status> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pending => value_pending = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_result => value_result = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Initializers {
                    pending: value_pending.unwrap_or_default(),
                    result: value_result,
                })
            }
        }

        deserializer.deserialize_struct(
            "Initializers",
            &[
                "pending",
                "result",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Initializers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Initializers",
            1 +
            self.result.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pending", &self.pending)?;
        if let Some(value) = &self.result {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "result", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Initializers {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.Initializers".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Initializers tracks the progress of initialization.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: std::array::IntoIter::new([
                    (
                        "pending".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Initializer>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "result".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Status>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If result is set with the Failure field, the object will be persisted to storage and then deleted, ensuring that other clients can observe the deletion.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: std::array::IntoIter::new([
                    "pending",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
