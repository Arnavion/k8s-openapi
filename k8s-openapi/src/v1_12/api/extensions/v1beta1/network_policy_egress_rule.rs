// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicyEgressRule

/// DEPRECATED 1.9 - This group version of NetworkPolicyEgressRule is deprecated by networking/v1/NetworkPolicyEgressRule. NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to. This type is beta-level in 1.8
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyEgressRule {
    /// List of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    pub ports: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPort>>,

    /// List of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list.
    pub to: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPeer>>,
}

impl<'de> ::serde::Deserialize<'de> for NetworkPolicyEgressRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ports,
            Key_to,
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
                            "ports" => Field::Key_ports,
                            "to" => Field::Key_to,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicyEgressRule;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NetworkPolicyEgressRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_ports: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPort>> = None;
                let mut value_to: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPeer>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ports => value_ports = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_to => value_to = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyEgressRule {
                    ports: value_ports,
                    to: value_to,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyEgressRule",
            &[
                "ports",
                "to",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NetworkPolicyEgressRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyEgressRule",
            0 +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.to.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ports {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.to {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "to", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
