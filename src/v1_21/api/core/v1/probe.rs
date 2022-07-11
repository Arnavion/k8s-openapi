// Generated from definition io.k8s.api.core.v1.Probe

/// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Probe {
    /// One and only one of the following should be specified. Exec specifies the action to take.
    pub exec: Option<crate::api::core::v1::ExecAction>,

    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    pub failure_threshold: Option<i32>,

    /// HTTPGet specifies the http request to perform.
    pub http_get: Option<crate::api::core::v1::HTTPGetAction>,

    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub initial_delay_seconds: Option<i32>,

    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    pub period_seconds: Option<i32>,

    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    pub success_threshold: Option<i32>,

    /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
    pub tcp_socket: Option<crate::api::core::v1::TCPSocketAction>,

    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is an alpha field and requires enabling ProbeTerminationGracePeriod feature gate.
    pub termination_grace_period_seconds: Option<i64>,

    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub timeout_seconds: Option<i32>,

}

#[cfg(feature = "dsl")]
impl Probe  {
    /// Set [`Self::exec`]
    pub  fn exec_set(&mut self, exec: impl Into<Option<crate::api::core::v1::ExecAction>>) -> &mut Self {
        self.exec = exec.into(); self
    }

    pub  fn exec(&mut self) -> &mut crate::api::core::v1::ExecAction {
        if self.exec.is_none() { self.exec = Some(Default::default()) }
        self.exec.as_mut().unwrap()
    }

    /// Modify [`Self::exec`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn exec_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ExecAction)) -> &mut Self {
        if self.exec.is_none() { self.exec = Some(Default::default()) };
        func(self.exec.as_mut().unwrap()); self
    }


    /// Set [`Self::failure_threshold`]
    pub  fn failure_threshold_set(&mut self, failure_threshold: impl Into<Option<i32>>) -> &mut Self {
        self.failure_threshold = failure_threshold.into(); self
    }

    pub  fn failure_threshold(&mut self) -> &mut i32 {
        if self.failure_threshold.is_none() { self.failure_threshold = Some(Default::default()) }
        self.failure_threshold.as_mut().unwrap()
    }

    /// Modify [`Self::failure_threshold`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn failure_threshold_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.failure_threshold.is_none() { self.failure_threshold = Some(Default::default()) };
        func(self.failure_threshold.as_mut().unwrap()); self
    }


    /// Set [`Self::http_get`]
    pub  fn http_get_set(&mut self, http_get: impl Into<Option<crate::api::core::v1::HTTPGetAction>>) -> &mut Self {
        self.http_get = http_get.into(); self
    }

    pub  fn http_get(&mut self) -> &mut crate::api::core::v1::HTTPGetAction {
        if self.http_get.is_none() { self.http_get = Some(Default::default()) }
        self.http_get.as_mut().unwrap()
    }

    /// Modify [`Self::http_get`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn http_get_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::HTTPGetAction)) -> &mut Self {
        if self.http_get.is_none() { self.http_get = Some(Default::default()) };
        func(self.http_get.as_mut().unwrap()); self
    }


    /// Set [`Self::initial_delay_seconds`]
    pub  fn initial_delay_seconds_set(&mut self, initial_delay_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.initial_delay_seconds = initial_delay_seconds.into(); self
    }

    pub  fn initial_delay_seconds(&mut self) -> &mut i32 {
        if self.initial_delay_seconds.is_none() { self.initial_delay_seconds = Some(Default::default()) }
        self.initial_delay_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::initial_delay_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn initial_delay_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.initial_delay_seconds.is_none() { self.initial_delay_seconds = Some(Default::default()) };
        func(self.initial_delay_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::period_seconds`]
    pub  fn period_seconds_set(&mut self, period_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.period_seconds = period_seconds.into(); self
    }

    pub  fn period_seconds(&mut self) -> &mut i32 {
        if self.period_seconds.is_none() { self.period_seconds = Some(Default::default()) }
        self.period_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::period_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn period_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.period_seconds.is_none() { self.period_seconds = Some(Default::default()) };
        func(self.period_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::success_threshold`]
    pub  fn success_threshold_set(&mut self, success_threshold: impl Into<Option<i32>>) -> &mut Self {
        self.success_threshold = success_threshold.into(); self
    }

    pub  fn success_threshold(&mut self) -> &mut i32 {
        if self.success_threshold.is_none() { self.success_threshold = Some(Default::default()) }
        self.success_threshold.as_mut().unwrap()
    }

    /// Modify [`Self::success_threshold`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn success_threshold_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.success_threshold.is_none() { self.success_threshold = Some(Default::default()) };
        func(self.success_threshold.as_mut().unwrap()); self
    }


    /// Set [`Self::tcp_socket`]
    pub  fn tcp_socket_set(&mut self, tcp_socket: impl Into<Option<crate::api::core::v1::TCPSocketAction>>) -> &mut Self {
        self.tcp_socket = tcp_socket.into(); self
    }

    pub  fn tcp_socket(&mut self) -> &mut crate::api::core::v1::TCPSocketAction {
        if self.tcp_socket.is_none() { self.tcp_socket = Some(Default::default()) }
        self.tcp_socket.as_mut().unwrap()
    }

    /// Modify [`Self::tcp_socket`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn tcp_socket_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::TCPSocketAction)) -> &mut Self {
        if self.tcp_socket.is_none() { self.tcp_socket = Some(Default::default()) };
        func(self.tcp_socket.as_mut().unwrap()); self
    }


    /// Set [`Self::termination_grace_period_seconds`]
    pub  fn termination_grace_period_seconds_set(&mut self, termination_grace_period_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.termination_grace_period_seconds = termination_grace_period_seconds.into(); self
    }

    pub  fn termination_grace_period_seconds(&mut self) -> &mut i64 {
        if self.termination_grace_period_seconds.is_none() { self.termination_grace_period_seconds = Some(Default::default()) }
        self.termination_grace_period_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::termination_grace_period_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn termination_grace_period_seconds_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.termination_grace_period_seconds.is_none() { self.termination_grace_period_seconds = Some(Default::default()) };
        func(self.termination_grace_period_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::timeout_seconds`]
    pub  fn timeout_seconds_set(&mut self, timeout_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.timeout_seconds = timeout_seconds.into(); self
    }

    pub  fn timeout_seconds(&mut self) -> &mut i32 {
        if self.timeout_seconds.is_none() { self.timeout_seconds = Some(Default::default()) }
        self.timeout_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::timeout_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn timeout_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.timeout_seconds.is_none() { self.timeout_seconds = Some(Default::default()) };
        func(self.timeout_seconds.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Probe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exec,
            Key_failure_threshold,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "exec" => Field::Key_exec,
                            "failureThreshold" => Field::Key_failure_threshold,
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Probe")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_exec: Option<crate::api::core::v1::ExecAction> = None;
                let mut value_failure_threshold: Option<i32> = None;
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
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Probe".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "exec".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ExecAction>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("One and only one of the following should be specified. Exec specifies the action to take.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "failureThreshold".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "httpGet".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::HTTPGetAction>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("HTTPGet specifies the http request to perform.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "initialDelaySeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "periodSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "successThreshold".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tcpSocket".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TCPSocketAction>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "terminationGracePeriodSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is an alpha field and requires enabling ProbeTerminationGracePeriod feature gate.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "timeoutSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
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
