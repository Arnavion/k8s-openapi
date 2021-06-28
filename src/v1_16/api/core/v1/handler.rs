// Generated from definition io.k8s.api.core.v1.Handler

/// Handler defines a specific action that should be taken
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Handler {
    /// One and only one of the following should be specified. Exec specifies the action to take.
    pub exec: Option<crate::api::core::v1::ExecAction>,

    /// HTTPGet specifies the http request to perform.
    pub http_get: Option<crate::api::core::v1::HTTPGetAction>,

    /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
    pub tcp_socket: Option<crate::api::core::v1::TCPSocketAction>,
}

impl<'de> crate::serde::Deserialize<'de> for Handler {
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
            type Value = Handler;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Handler")
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

                Ok(Handler {
                    exec: value_exec,
                    http_get: value_http_get,
                    tcp_socket: value_tcp_socket,
                })
            }
        }

        deserializer.deserialize_struct(
            "Handler",
            &[
                "exec",
                "httpGet",
                "tcpSocket",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Handler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Handler",
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

#[cfg(feature = "schema")]
impl crate::Schema for Handler {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Handler defines a specific action that should be taken",
          "properties": {
            "exec": crate::schema_ref_with_description(crate::api::core::v1::ExecAction::schema(), "One and only one of the following should be specified. Exec specifies the action to take."),
            "httpGet": crate::schema_ref_with_description(crate::api::core::v1::HTTPGetAction::schema(), "HTTPGet specifies the http request to perform."),
            "tcpSocket": crate::schema_ref_with_description(crate::api::core::v1::TCPSocketAction::schema(), "TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported")
          },
          "type": "object"
        })
    }
}
