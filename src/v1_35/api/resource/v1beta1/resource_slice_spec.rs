// Generated from definition io.k8s.api.resource.v1beta1.ResourceSliceSpec

/// ResourceSliceSpec contains the information published by the driver in one ResourceSlice.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceSliceSpec {
    /// AllNodes indicates that all nodes have access to the resources in the pool.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    pub all_nodes: Option<bool>,

    /// Devices lists some or all of the devices in this pool.
    ///
    /// Must not have more than 128 entries. If any device uses taints or consumes counters the limit is 64.
    ///
    /// Only one of Devices and SharedCounters can be set in a ResourceSlice.
    pub devices: Option<std::vec::Vec<crate::api::resource::v1beta1::Device>>,

    /// Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. It should use only lower case characters. This field is immutable.
    pub driver: std::string::String,

    /// NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.
    ///
    /// This field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set. This field is immutable.
    pub node_name: Option<std::string::String>,

    /// NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.
    ///
    /// Must use exactly one term.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    pub node_selector: Option<crate::api::core::v1::NodeSelector>,

    /// PerDeviceNodeSelection defines whether the access from nodes to resources in the pool is set on the ResourceSlice level or on each device. If it is set to true, every device defined the ResourceSlice must specify this individually.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    pub per_device_node_selection: Option<bool>,

    /// Pool describes the pool that this ResourceSlice belongs to.
    pub pool: crate::api::resource::v1beta1::ResourcePool,

    /// SharedCounters defines a list of counter sets, each of which has a name and a list of counters available.
    ///
    /// The names of the counter sets must be unique in the ResourcePool.
    ///
    /// Only one of Devices and SharedCounters can be set in a ResourceSlice.
    ///
    /// The maximum number of counter sets is 8.
    pub shared_counters: Option<std::vec::Vec<crate::api::resource::v1beta1::CounterSet>>,
}

impl crate::DeepMerge for ResourceSliceSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.all_nodes, other.all_nodes);
        crate::merge_strategies::list::atomic(&mut self.devices, other.devices);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.node_selector, other.node_selector);
        crate::DeepMerge::merge_from(&mut self.per_device_node_selection, other.per_device_node_selection);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
        crate::merge_strategies::list::atomic(&mut self.shared_counters, other.shared_counters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceSliceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_all_nodes,
            Key_devices,
            Key_driver,
            Key_node_name,
            Key_node_selector,
            Key_per_device_node_selection,
            Key_pool,
            Key_shared_counters,
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
                            "allNodes" => Field::Key_all_nodes,
                            "devices" => Field::Key_devices,
                            "driver" => Field::Key_driver,
                            "nodeName" => Field::Key_node_name,
                            "nodeSelector" => Field::Key_node_selector,
                            "perDeviceNodeSelection" => Field::Key_per_device_node_selection,
                            "pool" => Field::Key_pool,
                            "sharedCounters" => Field::Key_shared_counters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceSliceSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceSliceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_all_nodes: Option<bool> = None;
                let mut value_devices: Option<std::vec::Vec<crate::api::resource::v1beta1::Device>> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_node_selector: Option<crate::api::core::v1::NodeSelector> = None;
                let mut value_per_device_node_selection: Option<bool> = None;
                let mut value_pool: Option<crate::api::resource::v1beta1::ResourcePool> = None;
                let mut value_shared_counters: Option<std::vec::Vec<crate::api::resource::v1beta1::CounterSet>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all_nodes => value_all_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_devices => value_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_per_device_node_selection => value_per_device_node_selection = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_shared_counters => value_shared_counters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceSliceSpec {
                    all_nodes: value_all_nodes,
                    devices: value_devices,
                    driver: value_driver.unwrap_or_default(),
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    per_device_node_selection: value_per_device_node_selection,
                    pool: value_pool.unwrap_or_default(),
                    shared_counters: value_shared_counters,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceSliceSpec",
            &[
                "allNodes",
                "devices",
                "driver",
                "nodeName",
                "nodeSelector",
                "perDeviceNodeSelection",
                "pool",
                "sharedCounters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceSliceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceSliceSpec",
            2 +
            self.all_nodes.as_ref().map_or(0, |_| 1) +
            self.devices.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.per_device_node_selection.as_ref().map_or(0, |_| 1) +
            self.shared_counters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.all_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allNodes", value)?;
        }
        if let Some(value) = &self.devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devices", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.per_device_node_selection {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "perDeviceNodeSelection", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", &self.pool)?;
        if let Some(value) = &self.shared_counters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sharedCounters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceSliceSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.ResourceSliceSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceSliceSpec contains the information published by the driver in one ResourceSlice.",
            "type": "object",
            "properties": {
                "allNodes": {
                    "description": "AllNodes indicates that all nodes have access to the resources in the pool.\n\nExactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.",
                    "type": "boolean",
                },
                "devices": {
                    "description": "Devices lists some or all of the devices in this pool.\n\nMust not have more than 128 entries. If any device uses taints or consumes counters the limit is 64.\n\nOnly one of Devices and SharedCounters can be set in a ResourceSlice.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1beta1::Device>()),
                },
                "driver": {
                    "description": "Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. It should use only lower case characters. This field is immutable.",
                    "type": "string",
                },
                "nodeName": {
                    "description": "NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.\n\nThis field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.\n\nExactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set. This field is immutable.",
                    "type": "string",
                },
                "nodeSelector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>();
                    schema_obj.ensure_object().insert("description".into(), "NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.\n\nMust use exactly one term.\n\nExactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.".into());
                    schema_obj
                }),
                "perDeviceNodeSelection": {
                    "description": "PerDeviceNodeSelection defines whether the access from nodes to resources in the pool is set on the ResourceSlice level or on each device. If it is set to true, every device defined the ResourceSlice must specify this individually.\n\nExactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.",
                    "type": "boolean",
                },
                "pool": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::ResourcePool>();
                    schema_obj.ensure_object().insert("description".into(), "Pool describes the pool that this ResourceSlice belongs to.".into());
                    schema_obj
                }),
                "sharedCounters": {
                    "description": "SharedCounters defines a list of counter sets, each of which has a name and a list of counters available.\n\nThe names of the counter sets must be unique in the ResourcePool.\n\nOnly one of Devices and SharedCounters can be set in a ResourceSlice.\n\nThe maximum number of counter sets is 8.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1beta1::CounterSet>()),
                },
            },
            "required": [
                "driver",
                "pool",
            ],
        })
    }
}
