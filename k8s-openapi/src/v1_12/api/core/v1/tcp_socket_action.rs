// Generated from definition io.k8s.api.core.v1.TCPSocketAction

/// TCPSocketAction describes an action based on opening a socket
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TCPSocketAction {
    /// Optional: Host name to connect to, defaults to the pod IP.
    pub host: Option<String>,

    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: ::v1_12::apimachinery::pkg::util::intstr::IntOrString,
}

impl<'de> ::serde::Deserialize<'de> for TCPSocketAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_host,
            Key_port,
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
                            "host" => Field::Key_host,
                            "port" => Field::Key_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TCPSocketAction;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct TCPSocketAction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_host: Option<String> = None;
                let mut value_port: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_host => value_host = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TCPSocketAction {
                    host: value_host,
                    port: value_port.ok_or_else(|| ::serde::de::Error::missing_field("port"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TCPSocketAction",
            &[
                "host",
                "port",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for TCPSocketAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TCPSocketAction",
            0 +
            self.host.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.host {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
