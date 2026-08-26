// Generated from definition io.k8s.api.resource.v1beta2.NodeAllocatableResource

/// NodeAllocatableResource defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource. At least one of Mapping or Overhead must be specified. Not specifying either is an invalid configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAllocatableResource {
    /// Mapping is used when the device directly models a node allocatable resource like standard CPU or memory (e.g., with a CPU DRA driver). The calculated quantity is accounted for exactly once per claim instance on the node. To prevent node cgroup isolation friction, the scheduler explicitly blocks sharing mapped device claims across multiple pods.
    pub mapping: Option<crate::api::resource::v1beta2::NodeAllocatableMapping>,

    /// Overhead contains fields for modeling auxiliary overhead incurred on node allocatable resources when allocating devices that are not themselves modeling a node allocatable resource (e.g., host memory overhead for GPUs). Sharing overhead-mapped claims across multiple pods is allowed. The node allocatable overhead is accounted for individually for each pod referencing the claim. Overhead is always subtracted from the node's allocatable capacity for the resource, even when mapping is specified for the same resource. Eg: If a device models memory capacity per socket as a consumable capacity pool via Mapping (with CapacityKey), any overhead specified for the same resource will be subtracted from the node's general allocatable capacity and not from the per-socket capacity pool in Mapping.
    pub overhead: Option<crate::api::resource::v1beta2::NodeAllocatableOverhead>,
}

impl crate::DeepMerge for NodeAllocatableResource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.mapping, other.mapping);
        crate::DeepMerge::merge_from(&mut self.overhead, other.overhead);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeAllocatableResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_mapping,
            Key_overhead,
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
                            "mapping" => Field::Key_mapping,
                            "overhead" => Field::Key_overhead,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAllocatableResource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeAllocatableResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_mapping: Option<crate::api::resource::v1beta2::NodeAllocatableMapping> = None;
                let mut value_overhead: Option<crate::api::resource::v1beta2::NodeAllocatableOverhead> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mapping => value_mapping = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_overhead => value_overhead = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAllocatableResource {
                    mapping: value_mapping,
                    overhead: value_overhead,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAllocatableResource",
            &[
                "mapping",
                "overhead",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAllocatableResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAllocatableResource",
            self.mapping.as_ref().map_or(0, |_| 1) +
            self.overhead.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.mapping {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mapping", value)?;
        }
        if let Some(value) = &self.overhead {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "overhead", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAllocatableResource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.NodeAllocatableResource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeAllocatableResource defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource. At least one of Mapping or Overhead must be specified. Not specifying either is an invalid configuration.",
            "type": "object",
            "properties": {
                "mapping": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::NodeAllocatableMapping>();
                    schema_obj.ensure_object().insert("description".into(), "Mapping is used when the device directly models a node allocatable resource like standard CPU or memory (e.g., with a CPU DRA driver). The calculated quantity is accounted for exactly once per claim instance on the node. To prevent node cgroup isolation friction, the scheduler explicitly blocks sharing mapped device claims across multiple pods.".into());
                    schema_obj
                }),
                "overhead": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::NodeAllocatableOverhead>();
                    schema_obj.ensure_object().insert("description".into(), "Overhead contains fields for modeling auxiliary overhead incurred on node allocatable resources when allocating devices that are not themselves modeling a node allocatable resource (e.g., host memory overhead for GPUs). Sharing overhead-mapped claims across multiple pods is allowed. The node allocatable overhead is accounted for individually for each pod referencing the claim. Overhead is always subtracted from the node's allocatable capacity for the resource, even when mapping is specified for the same resource. Eg: If a device models memory capacity per socket as a consumable capacity pool via Mapping (with CapacityKey), any overhead specified for the same resource will be subtracted from the node's general allocatable capacity and not from the per-socket capacity pool in Mapping.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodeAllocatableResource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta2.NodeAllocatableResource".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodeAllocatableResource defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource. At least one of Mapping or Overhead must be specified. Not specifying either is an invalid configuration.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "mapping".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::NodeAllocatableMapping>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Mapping is used when the device directly models a node allocatable resource like standard CPU or memory (e.g., with a CPU DRA driver). The calculated quantity is accounted for exactly once per claim instance on the node. To prevent node cgroup isolation friction, the scheduler explicitly blocks sharing mapped device claims across multiple pods.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "overhead".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::NodeAllocatableOverhead>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Overhead contains fields for modeling auxiliary overhead incurred on node allocatable resources when allocating devices that are not themselves modeling a node allocatable resource (e.g., host memory overhead for GPUs). Sharing overhead-mapped claims across multiple pods is allowed. The node allocatable overhead is accounted for individually for each pod referencing the claim. Overhead is always subtracted from the node's allocatable capacity for the resource, even when mapping is specified for the same resource. Eg: If a device models memory capacity per socket as a consumable capacity pool via Mapping (with CapacityKey), any overhead specified for the same resource will be subtracted from the node's general allocatable capacity and not from the per-socket capacity pool in Mapping.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
