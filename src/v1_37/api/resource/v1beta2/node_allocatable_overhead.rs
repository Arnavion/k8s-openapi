// Generated from definition io.k8s.api.resource.v1beta2.NodeAllocatableOverhead

/// NodeAllocatableOverhead defines auxiliary resource overheads incurred when allocating a device. Overheads can be specified as a fixed cost per pod referencing the claim, a variable cost per container reference, or both. Kubelet accounts for this overhead by adding it to both the pod-level and container-level cgroups of referencing containers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAllocatableOverhead {
    /// PerContainer is applied per container reference to the claim. This models overhead scaling linearly with the number of containers actively using the device. When both PerPod and PerContainer are specified, the total overhead allocated for each pod referencing the claim is computed as: Quantity = PerPod + (PerContainer * NumReferences) Kubelet accounts for this overhead in cgroups: - Pod-level cgroup (requests and limits): Kubelet adds PerPod + (PerContainer * NumReferences). - Container-level cgroup (limits only): Kubelet adds PerPod + PerContainer for each referencing container. This allows any single container to access the pod-level overhead, while the parent cgroup caps the total usage to account for PerPod exactly once.
    pub per_container: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// PerPod is overhead applied once per pod referencing the claim on this node. This is a flat overhead incurred for every pod referencing the claim.
    pub per_pod: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl crate::DeepMerge for NodeAllocatableOverhead {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.per_container, other.per_container);
        crate::DeepMerge::merge_from(&mut self.per_pod, other.per_pod);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeAllocatableOverhead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_per_container,
            Key_per_pod,
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
                            "perContainer" => Field::Key_per_container,
                            "perPod" => Field::Key_per_pod,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAllocatableOverhead;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeAllocatableOverhead")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_per_container: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_per_pod: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_per_container => value_per_container = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_per_pod => value_per_pod = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAllocatableOverhead {
                    per_container: value_per_container,
                    per_pod: value_per_pod,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAllocatableOverhead",
            &[
                "perContainer",
                "perPod",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAllocatableOverhead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAllocatableOverhead",
            self.per_container.as_ref().map_or(0, |_| 1) +
            self.per_pod.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.per_container {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "perContainer", value)?;
        }
        if let Some(value) = &self.per_pod {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "perPod", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAllocatableOverhead {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.NodeAllocatableOverhead".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeAllocatableOverhead defines auxiliary resource overheads incurred when allocating a device. Overheads can be specified as a fixed cost per pod referencing the claim, a variable cost per container reference, or both. Kubelet accounts for this overhead by adding it to both the pod-level and container-level cgroups of referencing containers.",
            "type": "object",
            "properties": {
                "perContainer": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "PerContainer is applied per container reference to the claim. This models overhead scaling linearly with the number of containers actively using the device. When both PerPod and PerContainer are specified, the total overhead allocated for each pod referencing the claim is computed as: Quantity = PerPod + (PerContainer * NumReferences) Kubelet accounts for this overhead in cgroups: - Pod-level cgroup (requests and limits): Kubelet adds PerPod + (PerContainer * NumReferences). - Container-level cgroup (limits only): Kubelet adds PerPod + PerContainer for each referencing container. This allows any single container to access the pod-level overhead, while the parent cgroup caps the total usage to account for PerPod exactly once.".into());
                    schema_obj
                }),
                "perPod": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "PerPod is overhead applied once per pod referencing the claim on this node. This is a flat overhead incurred for every pod referencing the claim.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodeAllocatableOverhead {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta2.NodeAllocatableOverhead".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodeAllocatableOverhead defines auxiliary resource overheads incurred when allocating a device. Overheads can be specified as a fixed cost per pod referencing the claim, a variable cost per container reference, or both. Kubelet accounts for this overhead by adding it to both the pod-level and container-level cgroups of referencing containers.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "perContainer".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("PerContainer is applied per container reference to the claim. This models overhead scaling linearly with the number of containers actively using the device. When both PerPod and PerContainer are specified, the total overhead allocated for each pod referencing the claim is computed as: Quantity = PerPod + (PerContainer * NumReferences) Kubelet accounts for this overhead in cgroups: - Pod-level cgroup (requests and limits): Kubelet adds PerPod + (PerContainer * NumReferences). - Container-level cgroup (limits only): Kubelet adds PerPod + PerContainer for each referencing container. This allows any single container to access the pod-level overhead, while the parent cgroup caps the total usage to account for PerPod exactly once.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "perPod".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("PerPod is overhead applied once per pod referencing the claim on this node. This is a flat overhead incurred for every pod referencing the claim.".into()),
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
