// Generated from definition io.k8s.api.core.v1.NodeAddress

/// NodeAddress contains information for the node's address.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAddress {
    /// The node address.
    pub address: String,

    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    ///
    /// Possible enum values:
    ///  - `"ExternalDNS"` identifies a DNS name which resolves to an IP address which has the characteristics of a NodeExternalIP. The IP it resolves to may or may not be a listed NodeExternalIP address.
    ///  - `"ExternalIP"` identifies an IP address which is, in some way, intended to be more usable from outside the cluster then an internal IP, though no specific semantics are defined. It may be a globally routable IP, though it is not required to be. External IPs may be assigned directly to an interface on the node, like a NodeInternalIP, or alternatively, packets sent to the external IP may be NAT'ed to an internal node IP rather than being delivered directly (making the IP less efficient for node-to-node traffic than a NodeInternalIP).
    ///  - `"Hostname"` identifies a name of the node. Although every node can be assumed to have a NodeAddress of this type, its exact syntax and semantics are not defined, and are not consistent between different clusters.
    ///  - `"InternalDNS"` identifies a DNS name which resolves to an IP address which has the characteristics of a NodeInternalIP. The IP it resolves to may or may not be a listed NodeInternalIP address.
    ///  - `"InternalIP"` identifies an IP address which is assigned to one of the node's network interfaces. Every node should have at least one address of this type. An internal IP is normally expected to be reachable from every other node, but may not be visible to hosts outside the cluster. By default it is assumed that kube-apiserver can reach node internal IPs, though it is possible to configure clusters where this is not the case. NodeInternalIP is the default type of node IP, and does not necessarily imply that the IP is ONLY reachable internally. If a node has multiple internal IPs, no specific semantics are assigned to the additional IPs.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for NodeAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address,
            Key_type_,
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
                            "address" => Field::Key_address,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAddress;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeAddress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_address: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address => value_address = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAddress {
                    address: value_address.unwrap_or_default(),
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAddress",
            &[
                "address",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAddress",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "address", &self.address)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAddress {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NodeAddress".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeAddress contains information for the node's address.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "address".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The node address.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Node address type, one of Hostname, ExternalIP or InternalIP.\n\nPossible enum values:\n - `\"ExternalDNS\"` identifies a DNS name which resolves to an IP address which has the characteristics of a NodeExternalIP. The IP it resolves to may or may not be a listed NodeExternalIP address.\n - `\"ExternalIP\"` identifies an IP address which is, in some way, intended to be more usable from outside the cluster then an internal IP, though no specific semantics are defined. It may be a globally routable IP, though it is not required to be. External IPs may be assigned directly to an interface on the node, like a NodeInternalIP, or alternatively, packets sent to the external IP may be NAT'ed to an internal node IP rather than being delivered directly (making the IP less efficient for node-to-node traffic than a NodeInternalIP).\n - `\"Hostname\"` identifies a name of the node. Although every node can be assumed to have a NodeAddress of this type, its exact syntax and semantics are not defined, and are not consistent between different clusters.\n - `\"InternalDNS\"` identifies a DNS name which resolves to an IP address which has the characteristics of a NodeInternalIP. The IP it resolves to may or may not be a listed NodeInternalIP address.\n - `\"InternalIP\"` identifies an IP address which is assigned to one of the node's network interfaces. Every node should have at least one address of this type. An internal IP is normally expected to be reachable from every other node, but may not be visible to hosts outside the cluster. By default it is assumed that kube-apiserver can reach node internal IPs, though it is possible to configure clusters where this is not the case. NodeInternalIP is the default type of node IP, and does not necessarily imply that the IP is ONLY reachable internally. If a node has multiple internal IPs, no specific semantics are assigned to the additional IPs.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ]).collect(),
                required: IntoIterator::into_iter([
                    "address",
                    "type",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
