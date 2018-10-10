// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicyIngressRule

/// DEPRECATED 1.9 - This group version of NetworkPolicyIngressRule is deprecated by networking/v1/NetworkPolicyIngressRule. This NetworkPolicyIngressRule matches traffic if and only if the traffic matches both ports AND from.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyIngressRule {
    /// List of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least on item, this rule allows traffic only if the traffic matches at least one item in the from list.
    pub from: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPeer>>,

    /// List of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    pub ports: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPort>>,
}

impl<'de> ::serde::Deserialize<'de> for NetworkPolicyIngressRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
            Key_ports,
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
                            "from" => Field::Key_from,
                            "ports" => Field::Key_ports,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicyIngressRule;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NetworkPolicyIngressRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_from: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPeer>> = None;
                let mut value_ports: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyPort>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyIngressRule {
                    from: value_from,
                    ports: value_ports,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyIngressRule",
            &[
                "from",
                "ports",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NetworkPolicyIngressRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyIngressRule",
            0 +
            self.from.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.from {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "from", value)?;
        }
        if let Some(value) = &self.ports {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
