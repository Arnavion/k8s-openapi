// Generated from definition io.k8s.api.core.v1.LifecycleHandler

/// LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LifecycleHandler {
    /// Exec specifies the action to take.
    pub exec: Option<crate::api::core::v1::ExecAction>,

    /// HTTPGet specifies the http request to perform.
    pub http_get: Option<crate::api::core::v1::HTTPGetAction>,

    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    pub tcp_socket: Option<crate::api::core::v1::TCPSocketAction>,
}

impl crate::DeepMerge for LifecycleHandler {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.exec, other.exec);
        crate::DeepMerge::merge_from(&mut self.http_get, other.http_get);
        crate::DeepMerge::merge_from(&mut self.tcp_socket, other.tcp_socket);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LifecycleHandler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exec,
            Key_http_get,
            Key_tcp_socket,
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
                            "httpGet" => Field::Key_http_get,
                            "tcpSocket" => Field::Key_tcp_socket,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LifecycleHandler;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LifecycleHandler")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_exec: Option<crate::api::core::v1::ExecAction> = None;
                let mut value_http_get: Option<crate::api::core::v1::HTTPGetAction> = None;
                let mut value_tcp_socket: Option<crate::api::core::v1::TCPSocketAction> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_exec => value_exec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http_get => value_http_get = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tcp_socket => value_tcp_socket = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LifecycleHandler {
                    exec: value_exec,
                    http_get: value_http_get,
                    tcp_socket: value_tcp_socket,
                })
            }
        }

        deserializer.deserialize_struct(
            "LifecycleHandler",
            &[
                "exec",
                "httpGet",
                "tcpSocket",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LifecycleHandler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LifecycleHandler",
            self.exec.as_ref().map_or(0, |_| 1) +
            self.http_get.as_ref().map_or(0, |_| 1) +
            self.tcp_socket.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.exec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exec", value)?;
        }
        if let Some(value) = &self.http_get {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "httpGet", value)?;
        }
        if let Some(value) = &self.tcp_socket {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tcpSocket", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LifecycleHandler {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.LifecycleHandler".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.".to_owned()),
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
                                description: Some("Exec specifies the action to take.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
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
                        "tcpSocket".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TCPSocketAction>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.".to_owned()),
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
