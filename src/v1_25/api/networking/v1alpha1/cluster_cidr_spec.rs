// Generated from definition io.k8s.api.networking.v1alpha1.ClusterCIDRSpec

/// ClusterCIDRSpec defines the desired state of ClusterCIDR.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterCIDRSpec {
    /// IPv4 defines an IPv4 IP block in CIDR notation(e.g. "10.0.0.0/8"). At least one of IPv4 and IPv6 must be specified. This field is immutable.
    pub ipv4: Option<String>,

    /// IPv6 defines an IPv6 IP block in CIDR notation(e.g. "fd12:3456:789a:1::/64"). At least one of IPv4 and IPv6 must be specified. This field is immutable.
    pub ipv6: Option<String>,

    /// NodeSelector defines which nodes the config is applicable to. An empty or nil NodeSelector selects all nodes. This field is immutable.
    pub node_selector: Option<crate::api::core::v1::NodeSelector>,

    /// PerNodeHostBits defines the number of host bits to be configured per node. A subnet mask determines how much of the address is used for network bits and host bits. For example an IPv4 address of 192.168.0.0/24, splits the address into 24 bits for the network portion and 8 bits for the host portion. To allocate 256 IPs, set this field to 8 (a /24 mask for IPv4 or a /120 for IPv6). Minimum value is 4 (16 IPs). This field is immutable.
    pub per_node_host_bits: i32,
}

impl crate::DeepMerge for ClusterCIDRSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ipv4, other.ipv4);
        crate::DeepMerge::merge_from(&mut self.ipv6, other.ipv6);
        crate::DeepMerge::merge_from(&mut self.node_selector, other.node_selector);
        crate::DeepMerge::merge_from(&mut self.per_node_host_bits, other.per_node_host_bits);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ClusterCIDRSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ipv4,
            Key_ipv6,
            Key_node_selector,
            Key_per_node_host_bits,
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
                            "ipv4" => Field::Key_ipv4,
                            "ipv6" => Field::Key_ipv6,
                            "nodeSelector" => Field::Key_node_selector,
                            "perNodeHostBits" => Field::Key_per_node_host_bits,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterCIDRSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClusterCIDRSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ipv4: Option<String> = None;
                let mut value_ipv6: Option<String> = None;
                let mut value_node_selector: Option<crate::api::core::v1::NodeSelector> = None;
                let mut value_per_node_host_bits: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ipv4 => value_ipv4 = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ipv6 => value_ipv6 = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_per_node_host_bits => value_per_node_host_bits = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterCIDRSpec {
                    ipv4: value_ipv4,
                    ipv6: value_ipv6,
                    node_selector: value_node_selector,
                    per_node_host_bits: value_per_node_host_bits.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterCIDRSpec",
            &[
                "ipv4",
                "ipv6",
                "nodeSelector",
                "perNodeHostBits",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ClusterCIDRSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterCIDRSpec",
            1 +
            self.ipv4.as_ref().map_or(0, |_| 1) +
            self.ipv6.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ipv4 {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipv4", value)?;
        }
        if let Some(value) = &self.ipv6 {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipv6", value)?;
        }
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "perNodeHostBits", &self.per_node_host_bits)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ClusterCIDRSpec {
    fn schema_name() -> String {
        "io.k8s.api.networking.v1alpha1.ClusterCIDRSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ClusterCIDRSpec defines the desired state of ClusterCIDR.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ipv4".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPv4 defines an IPv4 IP block in CIDR notation(e.g. \"10.0.0.0/8\"). At least one of IPv4 and IPv6 must be specified. This field is immutable.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipv6".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPv6 defines an IPv6 IP block in CIDR notation(e.g. \"fd12:3456:789a:1::/64\"). At least one of IPv4 and IPv6 must be specified. This field is immutable.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeSelector defines which nodes the config is applicable to. An empty or nil NodeSelector selects all nodes. This field is immutable.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "perNodeHostBits".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PerNodeHostBits defines the number of host bits to be configured per node. A subnet mask determines how much of the address is used for network bits and host bits. For example an IPv4 address of 192.168.0.0/24, splits the address into 24 bits for the network portion and 8 bits for the host portion. To allocate 256 IPs, set this field to 8 (a /24 mask for IPv4 or a /120 for IPv6). Minimum value is 4 (16 IPs). This field is immutable.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "perNodeHostBits".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
