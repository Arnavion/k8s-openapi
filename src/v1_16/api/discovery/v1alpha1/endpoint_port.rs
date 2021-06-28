// Generated from definition io.k8s.api.discovery.v1alpha1.EndpointPort

/// EndpointPort represents a Port used by an EndpointSlice
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointPort {
    /// The name of this port. All ports in an EndpointSlice must have a unique name. If the EndpointSlice is dervied from a Kubernetes service, this corresponds to the Service.ports\[\].name. Name must either be an empty string or pass IANA_SVC_NAME validation: * must be no more than 15 characters long * may contain only \[-a-z0-9\] * must contain at least one letter \[a-z\] * it must not start or end with a hyphen, nor contain adjacent hyphens Default is empty string.
    pub name: Option<String>,

    /// The port number of the endpoint. If this is not specified, ports are not restricted and must be interpreted in the context of the specific consumer.
    pub port: Option<i32>,

    /// The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.
    pub protocol: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for EndpointPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_port,
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
                            "name" => Field::Key_name,
                            "port" => Field::Key_port,
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
            type Value = EndpointPort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointPort {
                    name: value_name,
                    port: value_port,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointPort",
            &[
                "name",
                "port",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointPort",
            self.name.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.protocol {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for EndpointPort {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "EndpointPort represents a Port used by an EndpointSlice",
          "properties": {
            "name": {
              "description": "The name of this port. All ports in an EndpointSlice must have a unique name. If the EndpointSlice is dervied from a Kubernetes service, this corresponds to the Service.ports[].name. Name must either be an empty string or pass IANA_SVC_NAME validation: * must be no more than 15 characters long * may contain only [-a-z0-9] * must contain at least one letter [a-z] * it must not start or end with a hyphen, nor contain adjacent hyphens Default is empty string.",
              "type": "string"
            },
            "port": {
              "description": "The port number of the endpoint. If this is not specified, ports are not restricted and must be interpreted in the context of the specific consumer.",
              "format": "int32",
              "type": "integer"
            },
            "protocol": {
              "description": "The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
