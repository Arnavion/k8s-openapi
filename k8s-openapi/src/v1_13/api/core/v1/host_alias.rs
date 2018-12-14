// Generated from definition io.k8s.api.core.v1.HostAlias

/// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HostAlias {
    /// Hostnames for the above IP address.
    pub hostnames: Option<Vec<String>>,

    /// IP address of the host file entry.
    pub ip: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for HostAlias {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostnames,
            Key_ip,
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
                            "hostnames" => Field::Key_hostnames,
                            "ip" => Field::Key_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HostAlias;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct HostAlias")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_hostnames: Option<Vec<String>> = None;
                let mut value_ip: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostnames => value_hostnames = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HostAlias {
                    hostnames: value_hostnames,
                    ip: value_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "HostAlias",
            &[
                "hostnames",
                "ip",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for HostAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HostAlias",
            0 +
            self.hostnames.as_ref().map_or(0, |_| 1) +
            self.ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hostnames {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "hostnames", value)?;
        }
        if let Some(value) = &self.ip {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
