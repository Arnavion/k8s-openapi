// Generated from definition io.k8s.api.core.v1.NodeStatus

/// NodeStatus is information about the current status of a node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeStatus {
    /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See http://pr.k8s.io/79391 for an example.
    pub addresses: Option<Vec<crate::api::core::v1::NodeAddress>>,

    /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    pub allocatable: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    pub capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition
    pub conditions: Option<Vec<crate::api::core::v1::NodeCondition>>,

    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    pub config: Option<crate::api::core::v1::NodeConfigStatus>,

    /// Endpoints of daemons running on the Node.
    pub daemon_endpoints: Option<crate::api::core::v1::NodeDaemonEndpoints>,

    /// List of container images on this node
    pub images: Option<Vec<crate::api::core::v1::ContainerImage>>,

    /// Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#info
    pub node_info: Option<crate::api::core::v1::NodeSystemInfo>,

    /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
    pub phase: Option<String>,

    /// List of volumes that are attached to the node.
    pub volumes_attached: Option<Vec<crate::api::core::v1::AttachedVolume>>,

    /// List of attachable volumes in use (mounted) by the node.
    pub volumes_in_use: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl NodeStatus  {
    /// Set [`Self::addresses`]
    pub  fn addresses_set(&mut self, addresses: impl Into<Option<Vec<crate::api::core::v1::NodeAddress>>>) -> &mut Self {
        self.addresses = addresses.into(); self
    }

    pub  fn addresses(&mut self) -> &mut Vec<crate::api::core::v1::NodeAddress> {
        if self.addresses.is_none() { self.addresses = Some(Default::default()) }
        self.addresses.as_mut().unwrap()
    }

    /// Modify [`Self::addresses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn addresses_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::NodeAddress>)) -> &mut Self {
        if self.addresses.is_none() { self.addresses = Some(Default::default()) };
        func(self.addresses.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::addresses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn addresses_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::NodeAddress)) -> &mut Self {
        if self.addresses.is_none() {
            self.addresses = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.addresses.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::addresses`]
    pub  fn addresses_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::NodeAddress]>) -> &mut Self {
         if self.addresses.is_none() { self.addresses = Some(Vec::new()); }
         let addresses = &mut self.addresses.as_mut().unwrap();
         for item in other.borrow() {
             addresses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allocatable`]
    pub  fn allocatable_set(&mut self, allocatable: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.allocatable = allocatable.into(); self
    }

    pub  fn allocatable(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.allocatable.is_none() { self.allocatable = Some(Default::default()) }
        self.allocatable.as_mut().unwrap()
    }

    /// Modify [`Self::allocatable`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allocatable_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.allocatable.is_none() { self.allocatable = Some(Default::default()) };
        func(self.allocatable.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::allocatable`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn allocatable_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.allocatable.is_none() {
            self.allocatable = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.allocatable.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::allocatable`]
    pub  fn allocatable_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.allocatable.is_none() { self.allocatable = Some(std::collections::BTreeMap::new()); }
         let allocatable = &mut self.allocatable.as_mut().unwrap();
         for (name, value) in other.borrow() {
             allocatable.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::capacity`]
    pub  fn capacity_set(&mut self, capacity: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.capacity = capacity.into(); self
    }

    pub  fn capacity(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.capacity.is_none() { self.capacity = Some(Default::default()) }
        self.capacity.as_mut().unwrap()
    }

    /// Modify [`Self::capacity`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn capacity_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.capacity.is_none() { self.capacity = Some(Default::default()) };
        func(self.capacity.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::capacity`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn capacity_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.capacity.is_none() {
            self.capacity = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.capacity.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::capacity`]
    pub  fn capacity_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.capacity.is_none() { self.capacity = Some(std::collections::BTreeMap::new()); }
         let capacity = &mut self.capacity.as_mut().unwrap();
         for (name, value) in other.borrow() {
             capacity.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::core::v1::NodeCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::core::v1::NodeCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::NodeCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::NodeCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::NodeCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::config`]
    pub  fn config_set(&mut self, config: impl Into<Option<crate::api::core::v1::NodeConfigStatus>>) -> &mut Self {
        self.config = config.into(); self
    }

    pub  fn config(&mut self) -> &mut crate::api::core::v1::NodeConfigStatus {
        if self.config.is_none() { self.config = Some(Default::default()) }
        self.config.as_mut().unwrap()
    }

    /// Modify [`Self::config`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn config_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::NodeConfigStatus)) -> &mut Self {
        if self.config.is_none() { self.config = Some(Default::default()) };
        func(self.config.as_mut().unwrap()); self
    }


    /// Set [`Self::daemon_endpoints`]
    pub  fn daemon_endpoints_set(&mut self, daemon_endpoints: impl Into<Option<crate::api::core::v1::NodeDaemonEndpoints>>) -> &mut Self {
        self.daemon_endpoints = daemon_endpoints.into(); self
    }

    pub  fn daemon_endpoints(&mut self) -> &mut crate::api::core::v1::NodeDaemonEndpoints {
        if self.daemon_endpoints.is_none() { self.daemon_endpoints = Some(Default::default()) }
        self.daemon_endpoints.as_mut().unwrap()
    }

    /// Modify [`Self::daemon_endpoints`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn daemon_endpoints_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::NodeDaemonEndpoints)) -> &mut Self {
        if self.daemon_endpoints.is_none() { self.daemon_endpoints = Some(Default::default()) };
        func(self.daemon_endpoints.as_mut().unwrap()); self
    }


    /// Set [`Self::images`]
    pub  fn images_set(&mut self, images: impl Into<Option<Vec<crate::api::core::v1::ContainerImage>>>) -> &mut Self {
        self.images = images.into(); self
    }

    pub  fn images(&mut self) -> &mut Vec<crate::api::core::v1::ContainerImage> {
        if self.images.is_none() { self.images = Some(Default::default()) }
        self.images.as_mut().unwrap()
    }

    /// Modify [`Self::images`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn images_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::ContainerImage>)) -> &mut Self {
        if self.images.is_none() { self.images = Some(Default::default()) };
        func(self.images.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::images`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn images_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ContainerImage)) -> &mut Self {
        if self.images.is_none() {
            self.images = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.images.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::images`]
    pub  fn images_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::ContainerImage]>) -> &mut Self {
         if self.images.is_none() { self.images = Some(Vec::new()); }
         let images = &mut self.images.as_mut().unwrap();
         for item in other.borrow() {
             images.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::node_info`]
    pub  fn node_info_set(&mut self, node_info: impl Into<Option<crate::api::core::v1::NodeSystemInfo>>) -> &mut Self {
        self.node_info = node_info.into(); self
    }

    pub  fn node_info(&mut self) -> &mut crate::api::core::v1::NodeSystemInfo {
        if self.node_info.is_none() { self.node_info = Some(Default::default()) }
        self.node_info.as_mut().unwrap()
    }

    /// Modify [`Self::node_info`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_info_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::NodeSystemInfo)) -> &mut Self {
        if self.node_info.is_none() { self.node_info = Some(Default::default()) };
        func(self.node_info.as_mut().unwrap()); self
    }


    /// Set [`Self::phase`]
    pub  fn phase_set(&mut self, phase: impl Into<Option<String>>) -> &mut Self {
        self.phase = phase.into(); self
    }

    pub  fn phase(&mut self) -> &mut String {
        if self.phase.is_none() { self.phase = Some(Default::default()) }
        self.phase.as_mut().unwrap()
    }

    /// Modify [`Self::phase`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn phase_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.phase.is_none() { self.phase = Some(Default::default()) };
        func(self.phase.as_mut().unwrap()); self
    }


    /// Set [`Self::volumes_attached`]
    pub  fn volumes_attached_set(&mut self, volumes_attached: impl Into<Option<Vec<crate::api::core::v1::AttachedVolume>>>) -> &mut Self {
        self.volumes_attached = volumes_attached.into(); self
    }

    pub  fn volumes_attached(&mut self) -> &mut Vec<crate::api::core::v1::AttachedVolume> {
        if self.volumes_attached.is_none() { self.volumes_attached = Some(Default::default()) }
        self.volumes_attached.as_mut().unwrap()
    }

    /// Modify [`Self::volumes_attached`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volumes_attached_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::AttachedVolume>)) -> &mut Self {
        if self.volumes_attached.is_none() { self.volumes_attached = Some(Default::default()) };
        func(self.volumes_attached.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volumes_attached`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volumes_attached_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::AttachedVolume)) -> &mut Self {
        if self.volumes_attached.is_none() {
            self.volumes_attached = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volumes_attached.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volumes_attached`]
    pub  fn volumes_attached_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::AttachedVolume]>) -> &mut Self {
         if self.volumes_attached.is_none() { self.volumes_attached = Some(Vec::new()); }
         let volumes_attached = &mut self.volumes_attached.as_mut().unwrap();
         for item in other.borrow() {
             volumes_attached.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::volumes_in_use`]
    pub  fn volumes_in_use_set(&mut self, volumes_in_use: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.volumes_in_use = volumes_in_use.into(); self
    }

    pub  fn volumes_in_use(&mut self) -> &mut Vec<String> {
        if self.volumes_in_use.is_none() { self.volumes_in_use = Some(Default::default()) }
        self.volumes_in_use.as_mut().unwrap()
    }

    /// Modify [`Self::volumes_in_use`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volumes_in_use_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.volumes_in_use.is_none() { self.volumes_in_use = Some(Default::default()) };
        func(self.volumes_in_use.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volumes_in_use`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volumes_in_use_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volumes_in_use.is_none() {
            self.volumes_in_use = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volumes_in_use.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volumes_in_use`]
    pub  fn volumes_in_use_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.volumes_in_use.is_none() { self.volumes_in_use = Some(Vec::new()); }
         let volumes_in_use = &mut self.volumes_in_use.as_mut().unwrap();
         for item in other.borrow() {
             volumes_in_use.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for NodeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_addresses: Option<Vec<crate::api::core::v1::NodeAddress>> = None;
                let mut value_allocatable: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_conditions: Option<Vec<crate::api::core::v1::NodeCondition>> = None;
                let mut value_config: Option<crate::api::core::v1::NodeConfigStatus> = None;
                let mut value_daemon_endpoints: Option<crate::api::core::v1::NodeDaemonEndpoints> = None;
                let mut value_images: Option<Vec<crate::api::core::v1::ContainerImage>> = None;
                let mut value_node_info: Option<crate::api::core::v1::NodeSystemInfo> = None;
                let mut value_phase: Option<String> = None;
                let mut value_volumes_attached: Option<Vec<crate::api::core::v1::AttachedVolume>> = None;
                let mut value_volumes_in_use: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocatable => value_allocatable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config => value_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_daemon_endpoints => value_daemon_endpoints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_images => value_images = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_info => value_node_info = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes_attached => value_volumes_attached = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes_in_use => value_volumes_in_use = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for NodeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeStatus",
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
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "addresses", value)?;
        }
        if let Some(value) = &self.allocatable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatable", value)?;
        }
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.daemon_endpoints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "daemonEndpoints", value)?;
        }
        if let Some(value) = &self.images {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "images", value)?;
        }
        if let Some(value) = &self.node_info {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeInfo", value)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.volumes_attached {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumesAttached", value)?;
        }
        if let Some(value) = &self.volumes_in_use {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumesInUse", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NodeStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeStatus is information about the current status of a node.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addresses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See http://pr.k8s.io/79391 for an example.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::NodeAddress>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allocatable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacity".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::NodeCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "config".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeConfigStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status of the config assigned to the node via the dynamic Kubelet config feature.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "daemonEndpoints".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeDaemonEndpoints>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Endpoints of daemons running on the Node.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "images".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of container images on this node".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerImage>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeInfo".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSystemInfo>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#info".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "phase".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumesAttached".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of volumes that are attached to the node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::AttachedVolume>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumesInUse".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of attachable volumes in use (mounted) by the node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
