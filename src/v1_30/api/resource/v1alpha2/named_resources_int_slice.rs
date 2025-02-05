// Generated from definition io.k8s.api.resource.v1alpha2.NamedResourcesIntSlice

/// NamedResourcesIntSlice contains a slice of 64-bit integers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamedResourcesIntSlice {
    /// Ints is the slice of 64-bit integers.
    pub ints: std::vec::Vec<i64>,
}

impl crate::DeepMerge for NamedResourcesIntSlice {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.ints, other.ints);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NamedResourcesIntSlice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ints,
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
                            "ints" => Field::Key_ints,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamedResourcesIntSlice;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NamedResourcesIntSlice")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ints: Option<std::vec::Vec<i64>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ints => value_ints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamedResourcesIntSlice {
                    ints: value_ints.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NamedResourcesIntSlice",
            &[
                "ints",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamedResourcesIntSlice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamedResourcesIntSlice",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ints", &self.ints)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamedResourcesIntSlice {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.NamedResourcesIntSlice".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("NamedResourcesIntSlice contains a slice of 64-bit integers.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ints".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Ints is the slice of 64-bit integers.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                                        format: Some("int64".into()),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "ints".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
