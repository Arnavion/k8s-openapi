// Generated from definition io.k8s.api.core.v1.DaemonEndpoint

/// DaemonEndpoint contains information about a single Daemon endpoint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonEndpoint {
    /// Port number of the given endpoint.
    pub port: i32,
}

impl<'de> ::serde::Deserialize<'de> for DaemonEndpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
                            "Port" => Field::Key_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonEndpoint;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DaemonEndpoint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_port: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_port => value_port = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonEndpoint {
                    port: value_port.ok_or_else(|| ::serde::de::Error::missing_field("Port"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonEndpoint",
            &[
                "Port",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for DaemonEndpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonEndpoint",
            0 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Port", &self.port)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
