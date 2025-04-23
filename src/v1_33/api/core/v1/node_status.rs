// Generated from definition io.k8s.api.core.v1.NodeStatus

/// NodeStatus is information about the current status of a node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeStatus {
    /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/reference/node/node-status/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
    pub addresses: Option<std::vec::Vec<crate::api::core::v1::NodeAddress>>,

    /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    pub allocatable: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity
    pub capacity: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/reference/node/node-status/#condition
    pub conditions: Option<std::vec::Vec<crate::api::core::v1::NodeCondition>>,

    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    pub config: Option<crate::api::core::v1::NodeConfigStatus>,

    /// Endpoints of daemons running on the Node.
    pub daemon_endpoints: Option<crate::api::core::v1::NodeDaemonEndpoints>,

    /// Features describes the set of features implemented by the CRI implementation.
    pub features: Option<crate::api::core::v1::NodeFeatures>,

    /// List of container images on this node
    pub images: Option<std::vec::Vec<crate::api::core::v1::ContainerImage>>,

    /// Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/reference/node/node-status/#info
    pub node_info: Option<crate::api::core::v1::NodeSystemInfo>,

    /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
    pub phase: Option<std::string::String>,

    /// The available runtime handlers.
    pub runtime_handlers: Option<std::vec::Vec<crate::api::core::v1::NodeRuntimeHandler>>,

    /// List of volumes that are attached to the node.
    pub volumes_attached: Option<std::vec::Vec<crate::api::core::v1::AttachedVolume>>,

    /// List of attachable volumes in use (mounted) by the node.
    pub volumes_in_use: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for NodeStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.addresses,
            other.addresses,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::map::granular(&mut self.allocatable, other.allocatable, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.capacity, other.capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.config, other.config);
        crate::DeepMerge::merge_from(&mut self.daemon_endpoints, other.daemon_endpoints);
        crate::DeepMerge::merge_from(&mut self.features, other.features);
        crate::merge_strategies::list::atomic(&mut self.images, other.images);
        crate::DeepMerge::merge_from(&mut self.node_info, other.node_info);
        crate::DeepMerge::merge_from(&mut self.phase, other.phase);
        crate::merge_strategies::list::atomic(&mut self.runtime_handlers, other.runtime_handlers);
        crate::merge_strategies::list::atomic(&mut self.volumes_attached, other.volumes_attached);
        crate::merge_strategies::list::atomic(&mut self.volumes_in_use, other.volumes_in_use);
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
            Key_features,
            Key_images,
            Key_node_info,
            Key_phase,
            Key_runtime_handlers,
            Key_volumes_attached,
            Key_volumes_in_use,
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
                            "addresses" => Field::Key_addresses,
                            "allocatable" => Field::Key_allocatable,
                            "capacity" => Field::Key_capacity,
                            "conditions" => Field::Key_conditions,
                            "config" => Field::Key_config,
                            "daemonEndpoints" => Field::Key_daemon_endpoints,
                            "features" => Field::Key_features,
                            "images" => Field::Key_images,
                            "nodeInfo" => Field::Key_node_info,
                            "phase" => Field::Key_phase,
                            "runtimeHandlers" => Field::Key_runtime_handlers,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_addresses: Option<std::vec::Vec<crate::api::core::v1::NodeAddress>> = None;
                let mut value_allocatable: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::core::v1::NodeCondition>> = None;
                let mut value_config: Option<crate::api::core::v1::NodeConfigStatus> = None;
                let mut value_daemon_endpoints: Option<crate::api::core::v1::NodeDaemonEndpoints> = None;
                let mut value_features: Option<crate::api::core::v1::NodeFeatures> = None;
                let mut value_images: Option<std::vec::Vec<crate::api::core::v1::ContainerImage>> = None;
                let mut value_node_info: Option<crate::api::core::v1::NodeSystemInfo> = None;
                let mut value_phase: Option<std::string::String> = None;
                let mut value_runtime_handlers: Option<std::vec::Vec<crate::api::core::v1::NodeRuntimeHandler>> = None;
                let mut value_volumes_attached: Option<std::vec::Vec<crate::api::core::v1::AttachedVolume>> = None;
                let mut value_volumes_in_use: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocatable => value_allocatable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config => value_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_daemon_endpoints => value_daemon_endpoints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_features => value_features = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_images => value_images = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_info => value_node_info = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_runtime_handlers => value_runtime_handlers = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    features: value_features,
                    images: value_images,
                    node_info: value_node_info,
                    phase: value_phase,
                    runtime_handlers: value_runtime_handlers,
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
                "features",
                "images",
                "nodeInfo",
                "phase",
                "runtimeHandlers",
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
            self.features.as_ref().map_or(0, |_| 1) +
            self.images.as_ref().map_or(0, |_| 1) +
            self.node_info.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.runtime_handlers.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.features {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "features", value)?;
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
        if let Some(value) = &self.runtime_handlers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runtimeHandlers", value)?;
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
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.NodeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeStatus is information about the current status of a node.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addresses".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/reference/node/node-status/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::NodeAddress>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allocatable".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacity".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/reference/node/node-status/#condition".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::NodeCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "config".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeConfigStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status of the config assigned to the node via the dynamic Kubelet config feature.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "daemonEndpoints".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeDaemonEndpoints>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Endpoints of daemons running on the Node.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "features".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeFeatures>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Features describes the set of features implemented by the CRI implementation.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "images".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of container images on this node".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerImage>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeInfo".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSystemInfo>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/reference/node/node-status/#info".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "phase".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runtimeHandlers".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The available runtime handlers.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::NodeRuntimeHandler>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumesAttached".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of volumes that are attached to the node.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::AttachedVolume>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumesInUse".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of attachable volumes in use (mounted) by the node.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
