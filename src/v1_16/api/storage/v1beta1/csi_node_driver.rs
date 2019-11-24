// Generated from definition io.k8s.api.storage.v1beta1.CSINodeDriver

/// CSINodeDriver holds information about the specification of one CSI driver installed on a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeDriver {
    /// allocatable represents the volume resources of a node that are available for scheduling.
    pub allocatable: Option<crate::api::storage::v1beta1::VolumeNodeResources>,

    /// This is the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.
    pub name: String,

    /// nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as "node1", but the storage system may refer to the same node as "nodeA". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. "nodeA" instead of "node1". This field is required.
    pub node_id: String,

    /// topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. "company.com/zone", "company.com/region"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.
    pub topology_keys: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for CSINodeDriver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocatable,
            Key_name,
            Key_node_id,
            Key_topology_keys,
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
                            "allocatable" => Field::Key_allocatable,
                            "name" => Field::Key_name,
                            "nodeID" => Field::Key_node_id,
                            "topologyKeys" => Field::Key_topology_keys,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeDriver;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSINodeDriver")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allocatable: Option<crate::api::storage::v1beta1::VolumeNodeResources> = None;
                let mut value_name: Option<String> = None;
                let mut value_node_id: Option<String> = None;
                let mut value_topology_keys: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocatable => value_allocatable = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_node_id => value_node_id = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_topology_keys => value_topology_keys = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeDriver {
                    allocatable: value_allocatable,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    node_id: value_node_id.ok_or_else(|| serde::de::Error::missing_field("nodeID"))?,
                    topology_keys: value_topology_keys,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINodeDriver",
            &[
                "allocatable",
                "name",
                "nodeID",
                "topologyKeys",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CSINodeDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeDriver",
            2 +
            self.allocatable.as_ref().map_or(0, |_| 1) +
            self.topology_keys.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocatable {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allocatable", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "nodeID", &self.node_id)?;
        if let Some(value) = &self.topology_keys {
            serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKeys", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
