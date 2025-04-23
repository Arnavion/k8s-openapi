// Generated from definition io.k8s.api.core.v1.Lifecycle

/// Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    pub post_start: Option<crate::api::core::v1::LifecycleHandler>,

    /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    pub pre_stop: Option<crate::api::core::v1::LifecycleHandler>,

    /// StopSignal defines which signal will be sent to a container when it is being stopped. If not specified, the default is defined by the container runtime in use. StopSignal can only be set for Pods with a non-empty .spec.os.name
    pub stop_signal: Option<std::string::String>,
}

impl crate::DeepMerge for Lifecycle {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.post_start, other.post_start);
        crate::DeepMerge::merge_from(&mut self.pre_stop, other.pre_stop);
        crate::DeepMerge::merge_from(&mut self.stop_signal, other.stop_signal);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Lifecycle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_post_start,
            Key_pre_stop,
            Key_stop_signal,
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
                            "postStart" => Field::Key_post_start,
                            "preStop" => Field::Key_pre_stop,
                            "stopSignal" => Field::Key_stop_signal,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Lifecycle;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Lifecycle")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_post_start: Option<crate::api::core::v1::LifecycleHandler> = None;
                let mut value_pre_stop: Option<crate::api::core::v1::LifecycleHandler> = None;
                let mut value_stop_signal: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_post_start => value_post_start = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pre_stop => value_pre_stop = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stop_signal => value_stop_signal = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Lifecycle {
                    post_start: value_post_start,
                    pre_stop: value_pre_stop,
                    stop_signal: value_stop_signal,
                })
            }
        }

        deserializer.deserialize_struct(
            "Lifecycle",
            &[
                "postStart",
                "preStop",
                "stopSignal",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Lifecycle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Lifecycle",
            self.post_start.as_ref().map_or(0, |_| 1) +
            self.pre_stop.as_ref().map_or(0, |_| 1) +
            self.stop_signal.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.post_start {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "postStart", value)?;
        }
        if let Some(value) = &self.pre_stop {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preStop", value)?;
        }
        if let Some(value) = &self.stop_signal {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stopSignal", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Lifecycle {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.Lifecycle".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "postStart".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LifecycleHandler>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "preStop".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LifecycleHandler>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "stopSignal".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("StopSignal defines which signal will be sent to a container when it is being stopped. If not specified, the default is defined by the container runtime in use. StopSignal can only be set for Pods with a non-empty .spec.os.name".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
