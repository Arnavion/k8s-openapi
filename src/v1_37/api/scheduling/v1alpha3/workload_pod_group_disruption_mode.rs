// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupDisruptionMode

/// WorkloadPodGroupDisruptionMode defines how individual pods within a group can be disrupted. Exactly one mode must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadPodGroupDisruptionMode {
    /// all specifies that all pods in the group must be disrupted together.
    pub all: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupAllDisruptionMode>,

    /// single specifies that pods can be disrupted independently from each other.
    pub single: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupSingleDisruptionMode>,
}

impl crate::DeepMerge for WorkloadPodGroupDisruptionMode {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.all, other.all);
        crate::DeepMerge::merge_from(&mut self.single, other.single);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadPodGroupDisruptionMode {
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
            type Value = WorkloadPodGroupDisruptionMode;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadPodGroupDisruptionMode")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_all: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupAllDisruptionMode> = None;
                let mut value_single: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupSingleDisruptionMode> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all => value_all = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_single => value_single = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadPodGroupDisruptionMode {
                    all: value_all,
                    single: value_single,
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadPodGroupDisruptionMode",
            &[
                "all",
                "single",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadPodGroupDisruptionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadPodGroupDisruptionMode",
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
impl crate::schemars::JsonSchema for WorkloadPodGroupDisruptionMode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadPodGroupDisruptionMode defines how individual pods within a group can be disrupted. Exactly one mode must be set.",
            "type": "object",
            "properties": {
                "all": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupAllDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "all specifies that all pods in the group must be disrupted together.".into());
                    schema_obj
                }),
                "single": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupSingleDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "single specifies that pods can be disrupted independently from each other.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for WorkloadPodGroupDisruptionMode {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupDisruptionMode".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("WorkloadPodGroupDisruptionMode defines how individual pods within a group can be disrupted. Exactly one mode must be set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "all".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupAllDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("all specifies that all pods in the group must be disrupted together.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "single".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupSingleDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("single specifies that pods can be disrupted independently from each other.".into()),
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
