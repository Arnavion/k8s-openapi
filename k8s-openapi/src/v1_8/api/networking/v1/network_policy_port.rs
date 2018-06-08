// Generated from definition io.k8s.api.networking.v1.NetworkPolicyPort

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Debug, Default)]
pub struct NetworkPolicyPort {
    /// The port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers.
    pub port: Option<::v1_8::apimachinery::pkg::util::intstr::IntOrString>,

    /// The protocol (TCP or UDP) which traffic must match. If not specified, this field defaults to TCP.
    pub protocol: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for NetworkPolicyPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
            type Value = NetworkPolicyPort;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NetworkPolicyPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_port: Option<::v1_8::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_port => value_port = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyPort {
                    port: value_port,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyPort",
            &[
                "port",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NetworkPolicyPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyPort",
            0 +
            (if self.port.is_some() { 1 } else { 0 }) +
            (if self.protocol.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.port {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.protocol {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
