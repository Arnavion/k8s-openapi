// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupSchedulingConstraints

/// WorkloadPodGroupSchedulingConstraints defines leaf-level scheduling constraints, such as topology.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadPodGroupSchedulingConstraints {
    /// topology specifies desired topological placements for all pods within the pod group. If unset, no topology placement is requested.
    pub topology: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::TopologyConstraint>>,
}

impl crate::DeepMerge for WorkloadPodGroupSchedulingConstraints {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.topology, other.topology);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadPodGroupSchedulingConstraints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_topology,
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
                            "topology" => Field::Key_topology,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadPodGroupSchedulingConstraints;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadPodGroupSchedulingConstraints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_topology: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::TopologyConstraint>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_topology => value_topology = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadPodGroupSchedulingConstraints {
                    topology: value_topology,
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadPodGroupSchedulingConstraints",
            &[
                "topology",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadPodGroupSchedulingConstraints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadPodGroupSchedulingConstraints",
            self.topology.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.topology {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topology", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadPodGroupSchedulingConstraints {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupSchedulingConstraints".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadPodGroupSchedulingConstraints defines leaf-level scheduling constraints, such as topology.",
            "type": "object",
            "properties": {
                "topology": {
                    "description": "topology specifies desired topological placements for all pods within the pod group. If unset, no topology placement is requested.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha3::TopologyConstraint>()),
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for WorkloadPodGroupSchedulingConstraints {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.WorkloadPodGroupSchedulingConstraints".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("WorkloadPodGroupSchedulingConstraints defines leaf-level scheduling constraints, such as topology.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "topology".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("topology specifies desired topological placements for all pods within the pod group. If unset, no topology placement is requested.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::scheduling::v1alpha3::TopologyConstraint>()))),
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
