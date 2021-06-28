// Generated from definition io.k8s.api.core.v1.ContainerPort

/// ContainerPort represents a network port in a single container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerPort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 \< x \< 65536.
    pub container_port: i32,

    /// What host IP to bind the external port to.
    pub host_ip: Option<String>,

    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 \< x \< 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    pub host_port: Option<i32>,

    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    pub name: Option<String>,

    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    pub protocol: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ContainerPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_port,
            Key_host_ip,
            Key_host_port,
            Key_name,
            Key_protocol,
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
                            "containerPort" => Field::Key_container_port,
                            "hostIP" => Field::Key_host_ip,
                            "hostPort" => Field::Key_host_port,
                            "name" => Field::Key_name,
                            "protocol" => Field::Key_protocol,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerPort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_port: Option<i32> = None;
                let mut value_host_ip: Option<String> = None;
                let mut value_host_port: Option<i32> = None;
                let mut value_name: Option<String> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_port => value_container_port = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_host_ip => value_host_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_port => value_host_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerPort {
                    container_port: value_container_port.ok_or_else(|| crate::serde::de::Error::missing_field("containerPort"))?,
                    host_ip: value_host_ip,
                    host_port: value_host_port,
                    name: value_name,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerPort",
            &[
                "containerPort",
                "hostIP",
                "hostPort",
                "name",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerPort",
            1 +
            self.host_ip.as_ref().map_or(0, |_| 1) +
            self.host_port.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerPort", &self.container_port)?;
        if let Some(value) = &self.host_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIP", value)?;
        }
        if let Some(value) = &self.host_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPort", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.protocol {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ContainerPort {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ContainerPort represents a network port in a single container.",
          "properties": {
            "containerPort": {
              "description": "Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.",
              "format": "int32",
              "type": "integer"
            },
            "hostIP": {
              "description": "What host IP to bind the external port to.",
              "type": "string"
            },
            "hostPort": {
              "description": "Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.",
              "format": "int32",
              "type": "integer"
            },
            "name": {
              "description": "If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.",
              "type": "string"
            },
            "protocol": {
              "description": "Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\".",
              "type": "string"
            }
          },
          "required": [
            "containerPort"
          ],
          "type": "object"
        })
    }
}
