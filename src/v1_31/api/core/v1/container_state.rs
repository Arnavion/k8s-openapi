// Generated from definition io.k8s.api.core.v1.ContainerState

/// ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerState {
    /// Details about a running container
    pub running: Option<crate::api::core::v1::ContainerStateRunning>,

    /// Details about a terminated container
    pub terminated: Option<crate::api::core::v1::ContainerStateTerminated>,

    /// Details about a waiting container
    pub waiting: Option<crate::api::core::v1::ContainerStateWaiting>,
}

impl crate::DeepMerge for ContainerState {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.running, other.running);
        crate::DeepMerge::merge_from(&mut self.terminated, other.terminated);
        crate::DeepMerge::merge_from(&mut self.waiting, other.waiting);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_running,
            Key_terminated,
            Key_waiting,
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
                            "running" => Field::Key_running,
                            "terminated" => Field::Key_terminated,
                            "waiting" => Field::Key_waiting,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerState;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerState")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_running: Option<crate::api::core::v1::ContainerStateRunning> = None;
                let mut value_terminated: Option<crate::api::core::v1::ContainerStateTerminated> = None;
                let mut value_waiting: Option<crate::api::core::v1::ContainerStateWaiting> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_running => value_running = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_terminated => value_terminated = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_waiting => value_waiting = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerState {
                    running: value_running,
                    terminated: value_terminated,
                    waiting: value_waiting,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerState",
            &[
                "running",
                "terminated",
                "waiting",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerState",
            self.running.as_ref().map_or(0, |_| 1) +
            self.terminated.as_ref().map_or(0, |_| 1) +
            self.waiting.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.running {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "running", value)?;
        }
        if let Some(value) = &self.terminated {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminated", value)?;
        }
        if let Some(value) = &self.waiting {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "waiting", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerState {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ContainerState".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "running".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerStateRunning>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about a running container".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "terminated".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerStateTerminated>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about a terminated container".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "waiting".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerStateWaiting>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about a waiting container".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
