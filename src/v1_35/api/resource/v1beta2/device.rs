// Generated from definition io.k8s.api.resource.v1beta2.Device

/// Device represents one individual hardware instance that can be selected based on its attributes. Besides the name, exactly one field must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Device {
    /// AllNodes indicates that all nodes have access to the device.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    pub all_nodes: Option<bool>,

    /// AllowMultipleAllocations marks whether the device is allowed to be allocated to multiple DeviceRequests.
    ///
    /// If AllowMultipleAllocations is set to true, the device can be allocated more than once, and all of its capacity is consumable, regardless of whether the requestPolicy is defined or not.
    pub allow_multiple_allocations: Option<bool>,

    /// Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    pub attributes: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta2::DeviceAttribute>>,

    /// BindingConditions defines the conditions for proceeding with binding. All of these conditions must be set in the per-device status conditions with a value of True to proceed with binding the pod to the node while scheduling the pod.
    ///
    /// The maximum number of binding conditions is 4.
    ///
    /// The conditions must be a valid condition type string.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    pub binding_conditions: Option<std::vec::Vec<std::string::String>>,

    /// BindingFailureConditions defines the conditions for binding failure. They may be set in the per-device status conditions. If any is set to "True", a binding failure occurred.
    ///
    /// The maximum number of binding failure conditions is 4.
    ///
    /// The conditions must be a valid condition type string.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    pub binding_failure_conditions: Option<std::vec::Vec<std::string::String>>,

    /// BindsToNode indicates if the usage of an allocation involving this device has to be limited to exactly the node that was chosen when allocating the claim. If set to true, the scheduler will set the ResourceClaim.Status.Allocation.NodeSelector to match the node where the allocation was made.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    pub binds_to_node: Option<bool>,

    /// Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    pub capacity: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta2::DeviceCapacity>>,

    /// ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.
    ///
    /// There can only be a single entry per counterSet.
    ///
    /// The maximum number of device counter consumptions per device is 2.
    pub consumes_counters: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceCounterConsumption>>,

    /// Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.
    pub name: std::string::String,

    /// NodeName identifies the node where the device is available.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    pub node_name: Option<std::string::String>,

    /// NodeSelector defines the nodes where the device is available.
    ///
    /// Must use exactly one term.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    pub node_selector: Option<crate::api::core::v1::NodeSelector>,

    /// If specified, these are the driver-defined taints.
    ///
    /// The maximum number of taints is 16. If taints are set for any device in a ResourceSlice, then the maximum number of allowed devices per ResourceSlice is 64 instead of 128.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    pub taints: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceTaint>>,
}

impl crate::DeepMerge for Device {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.all_nodes, other.all_nodes);
        crate::DeepMerge::merge_from(&mut self.allow_multiple_allocations, other.allow_multiple_allocations);
        crate::merge_strategies::map::granular(&mut self.attributes, other.attributes, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.binding_conditions, other.binding_conditions);
        crate::merge_strategies::list::atomic(&mut self.binding_failure_conditions, other.binding_failure_conditions);
        crate::DeepMerge::merge_from(&mut self.binds_to_node, other.binds_to_node);
        crate::merge_strategies::map::granular(&mut self.capacity, other.capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.consumes_counters, other.consumes_counters);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.node_selector, other.node_selector);
        crate::merge_strategies::list::atomic(&mut self.taints, other.taints);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Device {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_all_nodes,
            Key_allow_multiple_allocations,
            Key_attributes,
            Key_binding_conditions,
            Key_binding_failure_conditions,
            Key_binds_to_node,
            Key_capacity,
            Key_consumes_counters,
            Key_name,
            Key_node_name,
            Key_node_selector,
            Key_taints,
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
                            "allowMultipleAllocations" => Field::Key_allow_multiple_allocations,
                            "attributes" => Field::Key_attributes,
                            "bindingConditions" => Field::Key_binding_conditions,
                            "bindingFailureConditions" => Field::Key_binding_failure_conditions,
                            "bindsToNode" => Field::Key_binds_to_node,
                            "capacity" => Field::Key_capacity,
                            "consumesCounters" => Field::Key_consumes_counters,
                            "name" => Field::Key_name,
                            "nodeName" => Field::Key_node_name,
                            "nodeSelector" => Field::Key_node_selector,
                            "taints" => Field::Key_taints,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Device;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Device")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_all_nodes: Option<bool> = None;
                let mut value_allow_multiple_allocations: Option<bool> = None;
                let mut value_attributes: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta2::DeviceAttribute>> = None;
                let mut value_binding_conditions: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_binding_failure_conditions: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_binds_to_node: Option<bool> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta2::DeviceCapacity>> = None;
                let mut value_consumes_counters: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceCounterConsumption>> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_node_selector: Option<crate::api::core::v1::NodeSelector> = None;
                let mut value_taints: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceTaint>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all_nodes => value_all_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allow_multiple_allocations => value_allow_multiple_allocations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attributes => value_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_binding_conditions => value_binding_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_binding_failure_conditions => value_binding_failure_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_binds_to_node => value_binds_to_node = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_consumes_counters => value_consumes_counters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taints => value_taints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Device {
                    all_nodes: value_all_nodes,
                    allow_multiple_allocations: value_allow_multiple_allocations,
                    attributes: value_attributes,
                    binding_conditions: value_binding_conditions,
                    binding_failure_conditions: value_binding_failure_conditions,
                    binds_to_node: value_binds_to_node,
                    capacity: value_capacity,
                    consumes_counters: value_consumes_counters,
                    name: value_name.unwrap_or_default(),
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    taints: value_taints,
                })
            }
        }

        deserializer.deserialize_struct(
            "Device",
            &[
                "allNodes",
                "allowMultipleAllocations",
                "attributes",
                "bindingConditions",
                "bindingFailureConditions",
                "bindsToNode",
                "capacity",
                "consumesCounters",
                "name",
                "nodeName",
                "nodeSelector",
                "taints",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Device",
            1 +
            self.all_nodes.as_ref().map_or(0, |_| 1) +
            self.allow_multiple_allocations.as_ref().map_or(0, |_| 1) +
            self.attributes.as_ref().map_or(0, |_| 1) +
            self.binding_conditions.as_ref().map_or(0, |_| 1) +
            self.binding_failure_conditions.as_ref().map_or(0, |_| 1) +
            self.binds_to_node.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
            self.consumes_counters.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.taints.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.all_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allNodes", value)?;
        }
        if let Some(value) = &self.allow_multiple_allocations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowMultipleAllocations", value)?;
        }
        if let Some(value) = &self.attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attributes", value)?;
        }
        if let Some(value) = &self.binding_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bindingConditions", value)?;
        }
        if let Some(value) = &self.binding_failure_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bindingFailureConditions", value)?;
        }
        if let Some(value) = &self.binds_to_node {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bindsToNode", value)?;
        }
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.consumes_counters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "consumesCounters", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.taints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "taints", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Device {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.Device".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Device represents one individual hardware instance that can be selected based on its attributes. Besides the name, exactly one field must be set.",
            "type": "object",
            "properties": {
                "allNodes": {
                    "description": "AllNodes indicates that all nodes have access to the device.\n\nMust only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.",
                    "type": "boolean",
                },
                "allowMultipleAllocations": {
                    "description": "AllowMultipleAllocations marks whether the device is allowed to be allocated to multiple DeviceRequests.\n\nIf AllowMultipleAllocations is set to true, the device can be allocated more than once, and all of its capacity is consumable, regardless of whether the requestPolicy is defined or not.",
                    "type": "boolean",
                },
                "attributes": {
                    "description": "Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.\n\nThe maximum number of attributes and capacities combined is 32.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::api::resource::v1beta2::DeviceAttribute>()),
                },
                "bindingConditions": {
                    "description": "BindingConditions defines the conditions for proceeding with binding. All of these conditions must be set in the per-device status conditions with a value of True to proceed with binding the pod to the node while scheduling the pod.\n\nThe maximum number of binding conditions is 4.\n\nThe conditions must be a valid condition type string.\n\nThis is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "bindingFailureConditions": {
                    "description": "BindingFailureConditions defines the conditions for binding failure. They may be set in the per-device status conditions. If any is set to \"True\", a binding failure occurred.\n\nThe maximum number of binding failure conditions is 4.\n\nThe conditions must be a valid condition type string.\n\nThis is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "bindsToNode": {
                    "description": "BindsToNode indicates if the usage of an allocation involving this device has to be limited to exactly the node that was chosen when allocating the claim. If set to true, the scheduler will set the ResourceClaim.Status.Allocation.NodeSelector to match the node where the allocation was made.\n\nThis is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.",
                    "type": "boolean",
                },
                "capacity": {
                    "description": "Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.\n\nThe maximum number of attributes and capacities combined is 32.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::api::resource::v1beta2::DeviceCapacity>()),
                },
                "consumesCounters": {
                    "description": "ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.\n\nThere can only be a single entry per counterSet.\n\nThe maximum number of device counter consumptions per device is 2.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1beta2::DeviceCounterConsumption>()),
                },
                "name": {
                    "description": "Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.",
                    "type": "string",
                },
                "nodeName": {
                    "description": "NodeName identifies the node where the device is available.\n\nMust only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.",
                    "type": "string",
                },
                "nodeSelector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>();
                    schema_obj.ensure_object().insert("description".into(), "NodeSelector defines the nodes where the device is available.\n\nMust use exactly one term.\n\nMust only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.".into());
                    schema_obj
                }),
                "taints": {
                    "description": "If specified, these are the driver-defined taints.\n\nThe maximum number of taints is 16. If taints are set for any device in a ResourceSlice, then the maximum number of allowed devices per ResourceSlice is 64 instead of 128.\n\nThis is an alpha field and requires enabling the DRADeviceTaints feature gate.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1beta2::DeviceTaint>()),
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
