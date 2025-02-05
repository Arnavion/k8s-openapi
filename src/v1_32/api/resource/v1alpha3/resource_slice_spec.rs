// Generated from definition io.k8s.api.resource.v1alpha3.ResourceSliceSpec

/// ResourceSliceSpec contains the information published by the driver in one ResourceSlice.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceSliceSpec {
    /// AllNodes indicates that all nodes have access to the resources in the pool.
    ///
    /// Exactly one of NodeName, NodeSelector and AllNodes must be set.
    pub all_nodes: Option<bool>,

    /// Devices lists some or all of the devices in this pool.
    ///
    /// Must not have more than 128 entries.
    pub devices: Option<std::vec::Vec<crate::api::resource::v1alpha3::Device>>,

    /// Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. This field is immutable.
    pub driver: std::string::String,

    /// NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.
    ///
    /// This field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.
    ///
    /// Exactly one of NodeName, NodeSelector and AllNodes must be set. This field is immutable.
    pub node_name: Option<std::string::String>,

    /// NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.
    ///
    /// Must use exactly one term.
    ///
    /// Exactly one of NodeName, NodeSelector and AllNodes must be set.
    pub node_selector: Option<crate::api::core::v1::NodeSelector>,

    /// Pool describes the pool that this ResourceSlice belongs to.
    pub pool: crate::api::resource::v1alpha3::ResourcePool,
}

impl crate::DeepMerge for ResourceSliceSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.all_nodes, other.all_nodes);
        crate::merge_strategies::list::atomic(&mut self.devices, other.devices);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.node_selector, other.node_selector);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
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
            Key_pool,
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
                            "pool" => Field::Key_pool,
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
                let mut value_devices: Option<std::vec::Vec<crate::api::resource::v1alpha3::Device>> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_node_selector: Option<crate::api::core::v1::NodeSelector> = None;
                let mut value_pool: Option<crate::api::resource::v1alpha3::ResourcePool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all_nodes => value_all_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_devices => value_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceSliceSpec {
                    all_nodes: value_all_nodes,
                    devices: value_devices,
                    driver: value_driver.unwrap_or_default(),
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    pool: value_pool.unwrap_or_default(),
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
                "pool",
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
            self.node_selector.as_ref().map_or(0, |_| 1),
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
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", &self.pool)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceSliceSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.ResourceSliceSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceSliceSpec contains the information published by the driver in one ResourceSlice.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allNodes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllNodes indicates that all nodes have access to the resources in the pool.\n\nExactly one of NodeName, NodeSelector and AllNodes must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "devices".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Devices lists some or all of the devices in this pool.\n\nMust not have more than 128 entries.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::Device>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "driver".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.\n\nThis field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.\n\nExactly one of NodeName, NodeSelector and AllNodes must be set. This field is immutable.".into()),
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
                                description: Some("NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.\n\nMust use exactly one term.\n\nExactly one of NodeName, NodeSelector and AllNodes must be set.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "pool".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha3::ResourcePool>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Pool describes the pool that this ResourceSlice belongs to.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "driver".into(),
                    "pool".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
