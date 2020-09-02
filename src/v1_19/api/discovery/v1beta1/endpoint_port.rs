// Generated from definition io.k8s.api.discovery.v1beta1.EndpointPort

/// EndpointPort represents a Port used by an EndpointSlice
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointPort {
    /// The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol.
    pub app_protocol: Option<String>,

    /// The name of this port. All ports in an EndpointSlice must have a unique name. If the EndpointSlice is dervied from a Kubernetes service, this corresponds to the Service.ports\[\].name. Name must either be an empty string or pass DNS_LABEL validation: * must be no more than 63 characters long. * must consist of lower case alphanumeric characters or '-'. * must start and end with an alphanumeric character. Default is empty string.
    pub name: Option<String>,

    /// The port number of the endpoint. If this is not specified, ports are not restricted and must be interpreted in the context of the specific consumer.
    pub port: Option<i32>,

    /// The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.
    pub protocol: Option<String>,
}

impl<'de> serde::Deserialize<'de> for EndpointPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_app_protocol,
            Key_name,
            Key_port,
            Key_protocol,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "appProtocol" => Field::Key_app_protocol,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EndpointPort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_app_protocol: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_app_protocol => value_app_protocol = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointPort {
                    app_protocol: value_app_protocol,
                    name: value_name,
                    port: value_port,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointPort",
            &[
                "appProtocol",
                "name",
                "port",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EndpointPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointPort",
            self.app_protocol.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.app_protocol {
            serde::ser::SerializeStruct::serialize_field(&mut state, "appProtocol", value)?;
        }
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.protocol {
            serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
