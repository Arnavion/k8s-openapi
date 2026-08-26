// Generated from definition io.k8s.api.core.v1.NodePodPreemptionPolicy

/// NodePodPreemptionPolicy defines the node-level policies governing preemption for pods on this node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodePodPreemptionPolicy {
    /// DisableResizePreemption lists the owners (e.g., autoscalers, operators, administrators) that have requested to disable scheduler and Kubelet preemption for in-place pod resize on this node. If this list is non-empty, resize-induced preemption is disabled on this node. This is an alpha field and requires enabling the InPlacePodVerticalScalingSchedulerPreemption feature gate.
    pub disable_resize_preemption: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for NodePodPreemptionPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::set(&mut self.disable_resize_preemption, other.disable_resize_preemption);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodePodPreemptionPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_disable_resize_preemption,
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
                            "disableResizePreemption" => Field::Key_disable_resize_preemption,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodePodPreemptionPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodePodPreemptionPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_disable_resize_preemption: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_disable_resize_preemption => value_disable_resize_preemption = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodePodPreemptionPolicy {
                    disable_resize_preemption: value_disable_resize_preemption,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodePodPreemptionPolicy",
            &[
                "disableResizePreemption",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodePodPreemptionPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodePodPreemptionPolicy",
            self.disable_resize_preemption.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.disable_resize_preemption {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "disableResizePreemption", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodePodPreemptionPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NodePodPreemptionPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodePodPreemptionPolicy defines the node-level policies governing preemption for pods on this node.",
            "type": "object",
            "properties": {
                "disableResizePreemption": {
                    "description": "DisableResizePreemption lists the owners (e.g., autoscalers, operators, administrators) that have requested to disable scheduler and Kubelet preemption for in-place pod resize on this node. If this list is non-empty, resize-induced preemption is disabled on this node. This is an alpha field and requires enabling the InPlacePodVerticalScalingSchedulerPreemption feature gate.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodePodPreemptionPolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.NodePodPreemptionPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodePodPreemptionPolicy defines the node-level policies governing preemption for pods on this node.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "disableResizePreemption".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("DisableResizePreemption lists the owners (e.g., autoscalers, operators, administrators) that have requested to disable scheduler and Kubelet preemption for in-place pod resize on this node. If this list is non-empty, resize-induced preemption is disabled on this node. This is an alpha field and requires enabling the InPlacePodVerticalScalingSchedulerPreemption feature gate.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                                        instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
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
