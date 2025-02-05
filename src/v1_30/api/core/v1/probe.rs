// Generated from definition io.k8s.api.core.v1.Probe

/// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Probe {
    /// Exec specifies the action to take.
    pub exec: Option<crate::api::core::v1::ExecAction>,

    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    pub failure_threshold: Option<i32>,

    /// GRPC specifies an action involving a GRPC port.
    pub grpc: Option<crate::api::core::v1::GRPCAction>,

    /// HTTPGet specifies the http request to perform.
    pub http_get: Option<crate::api::core::v1::HTTPGetAction>,

    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub initial_delay_seconds: Option<i32>,

    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    pub period_seconds: Option<i32>,

    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    pub success_threshold: Option<i32>,

    /// TCPSocket specifies an action involving a TCP port.
    pub tcp_socket: Option<crate::api::core::v1::TCPSocketAction>,

    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    pub termination_grace_period_seconds: Option<i64>,

    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub timeout_seconds: Option<i32>,
}

impl crate::DeepMerge for Probe {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.exec, other.exec);
        crate::DeepMerge::merge_from(&mut self.failure_threshold, other.failure_threshold);
        crate::DeepMerge::merge_from(&mut self.grpc, other.grpc);
        crate::DeepMerge::merge_from(&mut self.http_get, other.http_get);
        crate::DeepMerge::merge_from(&mut self.initial_delay_seconds, other.initial_delay_seconds);
        crate::DeepMerge::merge_from(&mut self.period_seconds, other.period_seconds);
        crate::DeepMerge::merge_from(&mut self.success_threshold, other.success_threshold);
        crate::DeepMerge::merge_from(&mut self.tcp_socket, other.tcp_socket);
        crate::DeepMerge::merge_from(&mut self.termination_grace_period_seconds, other.termination_grace_period_seconds);
        crate::DeepMerge::merge_from(&mut self.timeout_seconds, other.timeout_seconds);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Probe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exec,
            Key_failure_threshold,
            Key_grpc,
            Key_http_get,
            Key_initial_delay_seconds,
            Key_period_seconds,
            Key_success_threshold,
            Key_tcp_socket,
            Key_termination_grace_period_seconds,
            Key_timeout_seconds,
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
                            "exec" => Field::Key_exec,
                            "failureThreshold" => Field::Key_failure_threshold,
                            "grpc" => Field::Key_grpc,
                            "httpGet" => Field::Key_http_get,
                            "initialDelaySeconds" => Field::Key_initial_delay_seconds,
                            "periodSeconds" => Field::Key_period_seconds,
                            "successThreshold" => Field::Key_success_threshold,
                            "tcpSocket" => Field::Key_tcp_socket,
                            "terminationGracePeriodSeconds" => Field::Key_termination_grace_period_seconds,
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Probe;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Probe")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_exec: Option<crate::api::core::v1::ExecAction> = None;
                let mut value_failure_threshold: Option<i32> = None;
                let mut value_grpc: Option<crate::api::core::v1::GRPCAction> = None;
                let mut value_http_get: Option<crate::api::core::v1::HTTPGetAction> = None;
                let mut value_initial_delay_seconds: Option<i32> = None;
                let mut value_period_seconds: Option<i32> = None;
                let mut value_success_threshold: Option<i32> = None;
                let mut value_tcp_socket: Option<crate::api::core::v1::TCPSocketAction> = None;
                let mut value_termination_grace_period_seconds: Option<i64> = None;
                let mut value_timeout_seconds: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_exec => value_exec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failure_threshold => value_failure_threshold = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_grpc => value_grpc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http_get => value_http_get = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_initial_delay_seconds => value_initial_delay_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_period_seconds => value_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_success_threshold => value_success_threshold = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tcp_socket => value_tcp_socket = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_grace_period_seconds => value_termination_grace_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_seconds => value_timeout_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Probe {
                    exec: value_exec,
                    failure_threshold: value_failure_threshold,
                    grpc: value_grpc,
                    http_get: value_http_get,
                    initial_delay_seconds: value_initial_delay_seconds,
                    period_seconds: value_period_seconds,
                    success_threshold: value_success_threshold,
                    tcp_socket: value_tcp_socket,
                    termination_grace_period_seconds: value_termination_grace_period_seconds,
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "Probe",
            &[
                "exec",
                "failureThreshold",
                "grpc",
                "httpGet",
                "initialDelaySeconds",
                "periodSeconds",
                "successThreshold",
                "tcpSocket",
                "terminationGracePeriodSeconds",
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Probe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Probe",
            self.exec.as_ref().map_or(0, |_| 1) +
            self.failure_threshold.as_ref().map_or(0, |_| 1) +
            self.grpc.as_ref().map_or(0, |_| 1) +
            self.http_get.as_ref().map_or(0, |_| 1) +
            self.initial_delay_seconds.as_ref().map_or(0, |_| 1) +
            self.period_seconds.as_ref().map_or(0, |_| 1) +
            self.success_threshold.as_ref().map_or(0, |_| 1) +
            self.tcp_socket.as_ref().map_or(0, |_| 1) +
            self.termination_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.exec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exec", value)?;
        }
        if let Some(value) = &self.failure_threshold {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failureThreshold", value)?;
        }
        if let Some(value) = &self.grpc {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "grpc", value)?;
        }
        if let Some(value) = &self.http_get {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "httpGet", value)?;
        }
        if let Some(value) = &self.initial_delay_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initialDelaySeconds", value)?;
        }
        if let Some(value) = &self.period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "periodSeconds", value)?;
        }
        if let Some(value) = &self.success_threshold {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "successThreshold", value)?;
        }
        if let Some(value) = &self.tcp_socket {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tcpSocket", value)?;
        }
        if let Some(value) = &self.termination_grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationGracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.timeout_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Probe {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.Probe".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "exec".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ExecAction>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Exec specifies the action to take.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "failureThreshold".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "grpc".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GRPCAction>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("GRPC specifies an action involving a GRPC port.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "httpGet".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::HTTPGetAction>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("HTTPGet specifies the http request to perform.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "initialDelaySeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "periodSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "successThreshold".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tcpSocket".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TCPSocketAction>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("TCPSocket specifies an action involving a TCP port.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "terminationGracePeriodSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "timeoutSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
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
