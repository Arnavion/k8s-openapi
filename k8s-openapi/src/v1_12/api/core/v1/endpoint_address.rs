// Generated from definition io.k8s.api.core.v1.EndpointAddress

/// EndpointAddress is a tuple that describes single IP address.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointAddress {
    /// The Hostname of this endpoint
    pub hostname: Option<String>,

    /// The IP of this endpoint. May not be loopback (127.0.0.0/8), link-local (169.254.0.0/16), or link-local multicast ((224.0.0.0/24). IPv6 is also accepted but not fully supported on all platforms. Also, certain kubernetes components, like kube-proxy, are not IPv6 ready.
    pub ip: String,

    /// Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
    pub node_name: Option<String>,

    /// Reference to object providing the endpoint.
    pub target_ref: Option<::v1_12::api::core::v1::ObjectReference>,
}

impl<'de> ::serde::Deserialize<'de> for EndpointAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostname,
            Key_ip,
            Key_node_name,
            Key_target_ref,
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
                            "hostname" => Field::Key_hostname,
                            "ip" => Field::Key_ip,
                            "nodeName" => Field::Key_node_name,
                            "targetRef" => Field::Key_target_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointAddress;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct EndpointAddress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_hostname: Option<String> = None;
                let mut value_ip: Option<String> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_target_ref: Option<::v1_12::api::core::v1::ObjectReference> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostname => value_hostname = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_node_name => value_node_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_ref => value_target_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointAddress {
                    hostname: value_hostname,
                    ip: value_ip.ok_or_else(|| ::serde::de::Error::missing_field("ip"))?,
                    node_name: value_node_name,
                    target_ref: value_target_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointAddress",
            &[
                "hostname",
                "ip",
                "nodeName",
                "targetRef",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for EndpointAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointAddress",
            0 +
            self.hostname.as_ref().map_or(0, |_| 1) +
            1 +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.target_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hostname {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", &self.ip)?;
        if let Some(value) = &self.node_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.target_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "targetRef", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
