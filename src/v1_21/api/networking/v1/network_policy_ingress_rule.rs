// Generated from definition io.k8s.api.networking.v1.NetworkPolicyIngressRule

/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyIngressRule {
    /// List of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the from list.
    pub from: Vec<crate::api::networking::v1::NetworkPolicyPeer>,

    /// List of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    pub ports: Vec<crate::api::networking::v1::NetworkPolicyPort>,
}

impl<'de> crate::serde::Deserialize<'de> for NetworkPolicyIngressRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
            Key_ports,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicyIngressRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NetworkPolicyIngressRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_from: Option<Vec<crate::api::networking::v1::NetworkPolicyPeer>> = None;
                let mut value_ports: Option<Vec<crate::api::networking::v1::NetworkPolicyPort>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyIngressRule {
                    from: value_from.unwrap_or_default(),
                    ports: value_ports.unwrap_or_default(),
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

impl crate::serde::Serialize for NetworkPolicyIngressRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyIngressRule",
            usize::from(!self.from.is_empty()) +
            usize::from(!self.ports.is_empty()),
        )?;
        if !self.from.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        }
        if !self.ports.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", &self.ports)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl NetworkPolicyIngressRule {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.",
          "properties": {
            "from": {
              "description": "List of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the from list.",
              "items": crate::api::networking::v1::NetworkPolicyPeer::schema(),
              "type": "array"
            },
            "ports": {
              "description": "List of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.",
              "items": crate::api::networking::v1::NetworkPolicyPort::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}