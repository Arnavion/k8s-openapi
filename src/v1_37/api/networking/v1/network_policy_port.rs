// Generated from definition io.k8s.api.networking.v1.NetworkPolicyPort

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyPort {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
    pub end_port: Option<i32>,

    /// port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    pub port: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
    pub protocol: Option<std::string::String>,
}

impl crate::DeepMerge for NetworkPolicyPort {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.end_port, other.end_port);
        crate::DeepMerge::merge_from(&mut self.port, other.port);
        crate::DeepMerge::merge_from(&mut self.protocol, other.protocol);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NetworkPolicyPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_end_port,
            Key_port,
            Key_protocol,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "endPort" => Field::Key_end_port,
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
            type Value = NetworkPolicyPort;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NetworkPolicyPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_end_port: Option<i32> = None;
                let mut value_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_protocol: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_end_port => value_end_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyPort {
                    end_port: value_end_port,
                    port: value_port,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyPort",
            &[
                "endPort",
                "port",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NetworkPolicyPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyPort",
            self.end_port.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.end_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "endPort", value)?;
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NetworkPolicyPort {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.NetworkPolicyPort".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NetworkPolicyPort describes a port to allow traffic on",
            "type": "object",
            "properties": {
                "endPort": {
                    "description": "endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.",
                    "type": "integer",
                    "format": "int32",
                },
                "port": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>();
                    schema_obj.ensure_object().insert("description".into(), "port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.".into());
                    schema_obj
                }),
                "protocol": {
                    "description": "protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.",
                    "type": "string",
                },
            },
        })
    }
}
