// Generated from definition io.k8s.api.scheduling.v1alpha3.CompositeDisruptionMode

/// CompositeDisruptionMode defines how individual entities within a composite pod group can be disrupted. Exactly one mode must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompositeDisruptionMode {
    /// all specifies that all children groups can only be disrupted together.
    pub all: Option<crate::api::scheduling::v1alpha3::AllCompositeDisruptionMode>,

    /// single specifies that children groups can be disrupted independently from each other.
    pub single: Option<crate::api::scheduling::v1alpha3::SingleCompositeDisruptionMode>,
}

impl crate::DeepMerge for CompositeDisruptionMode {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.all, other.all);
        crate::DeepMerge::merge_from(&mut self.single, other.single);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CompositeDisruptionMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_all,
            Key_single,
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
                            "all" => Field::Key_all,
                            "single" => Field::Key_single,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CompositeDisruptionMode;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CompositeDisruptionMode")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_all: Option<crate::api::scheduling::v1alpha3::AllCompositeDisruptionMode> = None;
                let mut value_single: Option<crate::api::scheduling::v1alpha3::SingleCompositeDisruptionMode> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all => value_all = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_single => value_single = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CompositeDisruptionMode {
                    all: value_all,
                    single: value_single,
                })
            }
        }

        deserializer.deserialize_struct(
            "CompositeDisruptionMode",
            &[
                "all",
                "single",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CompositeDisruptionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CompositeDisruptionMode",
            self.all.as_ref().map_or(0, |_| 1) +
            self.single.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.all {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "all", value)?;
        }
        if let Some(value) = &self.single {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "single", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CompositeDisruptionMode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.CompositeDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CompositeDisruptionMode defines how individual entities within a composite pod group can be disrupted. Exactly one mode must be set.",
            "type": "object",
            "properties": {
                "all": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::AllCompositeDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "all specifies that all children groups can only be disrupted together.".into());
                    schema_obj
                }),
                "single": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::SingleCompositeDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "single specifies that children groups can be disrupted independently from each other.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for CompositeDisruptionMode {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.CompositeDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("CompositeDisruptionMode defines how individual entities within a composite pod group can be disrupted. Exactly one mode must be set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "all".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::AllCompositeDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("all specifies that all children groups can only be disrupted together.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "single".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::SingleCompositeDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("single specifies that children groups can be disrupted independently from each other.".into()),
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
