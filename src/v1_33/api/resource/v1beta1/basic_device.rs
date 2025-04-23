// Generated from definition io.k8s.api.resource.v1beta1.BasicDevice

/// BasicDevice defines one device instance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BasicDevice {
    /// AllNodes indicates that all nodes have access to the device.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    pub all_nodes: Option<bool>,

    /// Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    pub attributes: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceAttribute>>,

    /// Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    pub capacity: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceCapacity>>,

    /// ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.
    ///
    /// There can only be a single entry per counterSet.
    ///
    /// The total number of device counter consumption entries must be \<= 32. In addition, the total number in the entire ResourceSlice must be \<= 1024 (for example, 64 devices with 16 counters each).
    pub consumes_counters: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceCounterConsumption>>,

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
    /// The maximum number of taints is 4.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    pub taints: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceTaint>>,
}

impl crate::DeepMerge for BasicDevice {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.all_nodes, other.all_nodes);
        crate::merge_strategies::map::granular(&mut self.attributes, other.attributes, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.capacity, other.capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.consumes_counters, other.consumes_counters);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.node_selector, other.node_selector);
        crate::merge_strategies::list::atomic(&mut self.taints, other.taints);
    }
}

impl<'de> crate::serde::Deserialize<'de> for BasicDevice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_all_nodes,
            Key_attributes,
            Key_capacity,
            Key_consumes_counters,
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
                            "attributes" => Field::Key_attributes,
                            "capacity" => Field::Key_capacity,
                            "consumesCounters" => Field::Key_consumes_counters,
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
            type Value = BasicDevice;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("BasicDevice")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_all_nodes: Option<bool> = None;
                let mut value_attributes: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceAttribute>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceCapacity>> = None;
                let mut value_consumes_counters: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceCounterConsumption>> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_node_selector: Option<crate::api::core::v1::NodeSelector> = None;
                let mut value_taints: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceTaint>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all_nodes => value_all_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attributes => value_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_consumes_counters => value_consumes_counters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taints => value_taints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BasicDevice {
                    all_nodes: value_all_nodes,
                    attributes: value_attributes,
                    capacity: value_capacity,
                    consumes_counters: value_consumes_counters,
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    taints: value_taints,
                })
            }
        }

        deserializer.deserialize_struct(
            "BasicDevice",
            &[
                "allNodes",
                "attributes",
                "capacity",
                "consumesCounters",
                "nodeName",
                "nodeSelector",
                "taints",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for BasicDevice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BasicDevice",
            self.all_nodes.as_ref().map_or(0, |_| 1) +
            self.attributes.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
            self.consumes_counters.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.taints.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.all_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allNodes", value)?;
        }
        if let Some(value) = &self.attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attributes", value)?;
        }
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.consumes_counters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "consumesCounters", value)?;
        }
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
impl crate::schemars::JsonSchema for BasicDevice {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta1.BasicDevice".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("BasicDevice defines one device instance.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allNodes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllNodes indicates that all nodes have access to the device.\n\nMust only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "attributes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.\n\nThe maximum number of attributes and capacities combined is 32.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceAttribute>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacity".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.\n\nThe maximum number of attributes and capacities combined is 32.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceCapacity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "consumesCounters".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.\n\nThere can only be a single entry per counterSet.\n\nThe total number of device counter consumption entries must be <= 32. In addition, the total number in the entire ResourceSlice must be <= 1024 (for example, 64 devices with 16 counters each).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceCounterConsumption>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeName identifies the node where the device is available.\n\nMust only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeSelector defines the nodes where the device is available.\n\nMust use exactly one term.\n\nMust only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "taints".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, these are the driver-defined taints.\n\nThe maximum number of taints is 4.\n\nThis is an alpha field and requires enabling the DRADeviceTaints feature gate.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceTaint>()))),
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
