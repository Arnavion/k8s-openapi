// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupGangSchedulingPolicy

/// WorkloadPodGroupGangSchedulingPolicy defines the parameters for gang (all-or-nothing) scheduling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadPodGroupGangSchedulingPolicy {
    /// minCount is the minimum number of pods that must be scheduled at the same time for the scheduler to admit the entire group. This field is optional. If it is not specified, the controller should inject a context-specific sane default (e.g., parallelism for a Job). If set, it must be a positive integer.
    pub min_count: Option<i32>,
}

impl crate::DeepMerge for WorkloadPodGroupGangSchedulingPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.min_count, other.min_count);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadPodGroupGangSchedulingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_count,
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
                            "minCount" => Field::Key_min_count,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadPodGroupGangSchedulingPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadPodGroupGangSchedulingPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_min_count: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_count => value_min_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadPodGroupGangSchedulingPolicy {
                    min_count: value_min_count,
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadPodGroupGangSchedulingPolicy",
            &[
                "minCount",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadPodGroupGangSchedulingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadPodGroupGangSchedulingPolicy",
            self.min_count.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minCount", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadPodGroupGangSchedulingPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupGangSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadPodGroupGangSchedulingPolicy defines the parameters for gang (all-or-nothing) scheduling.",
            "type": "object",
            "properties": {
                "minCount": {
                    "description": "minCount is the minimum number of pods that must be scheduled at the same time for the scheduler to admit the entire group. This field is optional. If it is not specified, the controller should inject a context-specific sane default (e.g., parallelism for a Job). If set, it must be a positive integer.",
                    "type": "integer",
                    "format": "int32",
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for WorkloadPodGroupGangSchedulingPolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupGangSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("WorkloadPodGroupGangSchedulingPolicy defines the parameters for gang (all-or-nothing) scheduling.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "minCount".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("minCount is the minimum number of pods that must be scheduled at the same time for the scheduler to admit the entire group. This field is optional. If it is not specified, the controller should inject a context-specific sane default (e.g., parallelism for a Job). If set, it must be a positive integer.".into()),
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
