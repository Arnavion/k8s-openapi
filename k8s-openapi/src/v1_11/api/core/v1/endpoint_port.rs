// Generated from definition io.k8s.api.core.v1.EndpointPort

/// EndpointPort is a tuple that describes a single port.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointPort {
    /// The name of this port (corresponds to ServicePort.Name). Must be a DNS_LABEL. Optional only if one port is defined.
    pub name: Option<String>,

    /// The port number of the endpoint.
    pub port: i32,

    /// The IP protocol for this port. Must be UDP or TCP. Default is TCP.
    pub protocol: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for EndpointPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_port,
            Key_protocol,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointPort;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct EndpointPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_protocol => value_protocol = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointPort {
                    name: value_name,
                    port: value_port.ok_or_else(|| ::serde::de::Error::missing_field("port"))?,
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

impl ::serde::Serialize for EndpointPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointPort",
            0 +
            self.name.as_ref().map_or(0, |_| 1) +
            1 +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        if let Some(value) = &self.protocol {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
