// Generated from definition io.k8s.api.storage.v1.CSINodeDriver

/// CSINodeDriver holds information about the specification of one CSI driver installed on a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeDriver {
    /// allocatable represents the volume resources of a node that are available for scheduling. This field is beta.
    pub allocatable: Option<crate::api::storage::v1::VolumeNodeResources>,

    /// This is the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.
    pub name: String,

    /// nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as "node1", but the storage system may refer to the same node as "nodeA". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. "nodeA" instead of "node1". This field is required.
    pub node_id: String,

    /// topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. "company.com/zone", "company.com/region"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.
    pub topology_keys: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl CSINodeDriver  {
    /// Set [`Self::allocatable`]
    pub  fn allocatable_set(&mut self, allocatable: impl Into<Option<crate::api::storage::v1::VolumeNodeResources>>) -> &mut Self {
        self.allocatable = allocatable.into(); self
    }

    pub  fn allocatable(&mut self) -> &mut crate::api::storage::v1::VolumeNodeResources {
        if self.allocatable.is_none() { self.allocatable = Some(Default::default()) }
        self.allocatable.as_mut().unwrap()
    }

    /// Modify [`Self::allocatable`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allocatable_with(&mut self, func: impl FnOnce(&mut crate::api::storage::v1::VolumeNodeResources)) -> &mut Self {
        if self.allocatable.is_none() { self.allocatable = Some(Default::default()) };
        func(self.allocatable.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::node_id`]
    pub  fn node_id_set(&mut self, node_id: impl Into<String>) -> &mut Self {
        self.node_id = node_id.into(); self
    }

    pub  fn node_id(&mut self) -> &mut String {
        &mut self.node_id
    }

    /// Modify [`Self::node_id`] with a `func`
    pub  fn node_id_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.node_id); self
    }


    /// Set [`Self::topology_keys`]
    pub  fn topology_keys_set(&mut self, topology_keys: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.topology_keys = topology_keys.into(); self
    }

    pub  fn topology_keys(&mut self) -> &mut Vec<String> {
        if self.topology_keys.is_none() { self.topology_keys = Some(Default::default()) }
        self.topology_keys.as_mut().unwrap()
    }

    /// Modify [`Self::topology_keys`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn topology_keys_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.topology_keys.is_none() { self.topology_keys = Some(Default::default()) };
        func(self.topology_keys.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::topology_keys`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn topology_keys_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.topology_keys.is_none() {
            self.topology_keys = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.topology_keys.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::topology_keys`]
    pub  fn topology_keys_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.topology_keys.is_none() { self.topology_keys = Some(Vec::new()); }
         let topology_keys = &mut self.topology_keys.as_mut().unwrap();
         for item in other.borrow() {
             topology_keys.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for CSINodeDriver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocatable,
            Key_name,
            Key_node_id,
            Key_topology_keys,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeDriver;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSINodeDriver")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocatable: Option<crate::api::storage::v1::VolumeNodeResources> = None;
                let mut value_name: Option<String> = None;
                let mut value_node_id: Option<String> = None;
                let mut value_topology_keys: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocatable => value_allocatable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_id => value_node_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_keys => value_topology_keys = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeDriver {
                    allocatable: value_allocatable,
                    name: value_name.unwrap_or_default(),
                    node_id: value_node_id.unwrap_or_default(),
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

impl crate::serde::Serialize for CSINodeDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeDriver",
            2 +
            self.allocatable.as_ref().map_or(0, |_| 1) +
            self.topology_keys.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocatable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatable", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeID", &self.node_id)?;
        if let Some(value) = &self.topology_keys {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKeys", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSINodeDriver {
    fn schema_name() -> String {
        "io.k8s.api.storage.v1.CSINodeDriver".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CSINodeDriver holds information about the specification of one CSI driver installed on a node".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocatable".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::storage::v1::VolumeNodeResources>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allocatable represents the volume resources of a node that are available for scheduling. This field is beta.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This is the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as \"node1\", but the storage system may refer to the same node as \"nodeA\". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. \"nodeA\" instead of \"node1\". This field is required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "topologyKeys".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. \"company.com/zone\", \"company.com/region\"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.".to_owned()),
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
                required: [
                    "name".to_owned(),
                    "nodeID".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
