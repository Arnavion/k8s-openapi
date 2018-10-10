// Generated from definition io.k8s.api.core.v1.NodeStatus

/// NodeStatus is information about the current status of a node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeStatus {
    /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses
    pub addresses: Option<Vec<::v1_12::api::core::v1::NodeAddress>>,

    /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    pub allocatable: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>>,

    /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    pub capacity: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>>,

    /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition
    pub conditions: Option<Vec<::v1_12::api::core::v1::NodeCondition>>,

    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    pub config: Option<::v1_12::api::core::v1::NodeConfigStatus>,

    /// Endpoints of daemons running on the Node.
    pub daemon_endpoints: Option<::v1_12::api::core::v1::NodeDaemonEndpoints>,

    /// List of container images on this node
    pub images: Option<Vec<::v1_12::api::core::v1::ContainerImage>>,

    /// Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#info
    pub node_info: Option<::v1_12::api::core::v1::NodeSystemInfo>,

    /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
    pub phase: Option<String>,

    /// List of volumes that are attached to the node.
    pub volumes_attached: Option<Vec<::v1_12::api::core::v1::AttachedVolume>>,

    /// List of attachable volumes in use (mounted) by the node.
    pub volumes_in_use: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for NodeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_addresses,
            Key_allocatable,
            Key_capacity,
            Key_conditions,
            Key_config,
            Key_daemon_endpoints,
            Key_images,
            Key_node_info,
            Key_phase,
            Key_volumes_attached,
            Key_volumes_in_use,
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
                            "addresses" => Field::Key_addresses,
                            "allocatable" => Field::Key_allocatable,
                            "capacity" => Field::Key_capacity,
                            "conditions" => Field::Key_conditions,
                            "config" => Field::Key_config,
                            "daemonEndpoints" => Field::Key_daemon_endpoints,
                            "images" => Field::Key_images,
                            "nodeInfo" => Field::Key_node_info,
                            "phase" => Field::Key_phase,
                            "volumesAttached" => Field::Key_volumes_attached,
                            "volumesInUse" => Field::Key_volumes_in_use,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_addresses: Option<Vec<::v1_12::api::core::v1::NodeAddress>> = None;
                let mut value_allocatable: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_capacity: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_conditions: Option<Vec<::v1_12::api::core::v1::NodeCondition>> = None;
                let mut value_config: Option<::v1_12::api::core::v1::NodeConfigStatus> = None;
                let mut value_daemon_endpoints: Option<::v1_12::api::core::v1::NodeDaemonEndpoints> = None;
                let mut value_images: Option<Vec<::v1_12::api::core::v1::ContainerImage>> = None;
                let mut value_node_info: Option<::v1_12::api::core::v1::NodeSystemInfo> = None;
                let mut value_phase: Option<String> = None;
                let mut value_volumes_attached: Option<Vec<::v1_12::api::core::v1::AttachedVolume>> = None;
                let mut value_volumes_in_use: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocatable => value_allocatable = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config => value_config = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_daemon_endpoints => value_daemon_endpoints = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_images => value_images = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_info => value_node_info = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes_attached => value_volumes_attached = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes_in_use => value_volumes_in_use = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeStatus {
                    addresses: value_addresses,
                    allocatable: value_allocatable,
                    capacity: value_capacity,
                    conditions: value_conditions,
                    config: value_config,
                    daemon_endpoints: value_daemon_endpoints,
                    images: value_images,
                    node_info: value_node_info,
                    phase: value_phase,
                    volumes_attached: value_volumes_attached,
                    volumes_in_use: value_volumes_in_use,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeStatus",
            &[
                "addresses",
                "allocatable",
                "capacity",
                "conditions",
                "config",
                "daemonEndpoints",
                "images",
                "nodeInfo",
                "phase",
                "volumesAttached",
                "volumesInUse",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NodeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeStatus",
            0 +
            self.addresses.as_ref().map_or(0, |_| 1) +
            self.allocatable.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.config.as_ref().map_or(0, |_| 1) +
            self.daemon_endpoints.as_ref().map_or(0, |_| 1) +
            self.images.as_ref().map_or(0, |_| 1) +
            self.node_info.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.volumes_attached.as_ref().map_or(0, |_| 1) +
            self.volumes_in_use.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.addresses {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "addresses", value)?;
        }
        if let Some(value) = &self.allocatable {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatable", value)?;
        }
        if let Some(value) = &self.capacity {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.config {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.daemon_endpoints {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "daemonEndpoints", value)?;
        }
        if let Some(value) = &self.images {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "images", value)?;
        }
        if let Some(value) = &self.node_info {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeInfo", value)?;
        }
        if let Some(value) = &self.phase {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.volumes_attached {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumesAttached", value)?;
        }
        if let Some(value) = &self.volumes_in_use {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumesInUse", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
