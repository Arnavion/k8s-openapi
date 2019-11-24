// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicyPort

/// DEPRECATED 1.9 - This group version of NetworkPolicyPort is deprecated by networking/v1/NetworkPolicyPort.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyPort {
    /// If specified, the port on the given protocol.  This can either be a numerical or named port on a pod.  If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    pub port: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// Optional.  The protocol (TCP or UDP) which traffic must match. If not specified, this field defaults to TCP.
    pub protocol: Option<String>,
}

impl<'de> serde::Deserialize<'de> for NetworkPolicyPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
            type Value = NetworkPolicyPort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NetworkPolicyPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_port => value_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for NetworkPolicyPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyPort",
            self.port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.protocol {
            serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
