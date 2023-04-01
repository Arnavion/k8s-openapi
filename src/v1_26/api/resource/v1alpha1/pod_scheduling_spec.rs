// Generated from definition io.k8s.api.resource.v1alpha1.PodSchedulingSpec

/// PodSchedulingSpec describes where resources for the Pod are needed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSchedulingSpec {
    /// PotentialNodes lists nodes where the Pod might be able to run.
    ///
    /// The size of this field is limited to 128. This is large enough for many clusters. Larger clusters may need more attempts to find a node that suits all pending resources. This may get increased in the future, but not reduced.
    pub potential_nodes: Option<Vec<String>>,

    /// SelectedNode is the node for which allocation of ResourceClaims that are referenced by the Pod and that use "WaitForFirstConsumer" allocation is to be attempted.
    pub selected_node: Option<String>,
}

impl crate::DeepMerge for PodSchedulingSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::set(&mut self.potential_nodes, other.potential_nodes);
        crate::DeepMerge::merge_from(&mut self.selected_node, other.selected_node);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodSchedulingSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_potential_nodes,
            Key_selected_node,
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
                            "potentialNodes" => Field::Key_potential_nodes,
                            "selectedNode" => Field::Key_selected_node,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodSchedulingSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSchedulingSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_potential_nodes: Option<Vec<String>> = None;
                let mut value_selected_node: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_potential_nodes => value_potential_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selected_node => value_selected_node = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSchedulingSpec {
                    potential_nodes: value_potential_nodes,
                    selected_node: value_selected_node,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSchedulingSpec",
            &[
                "potentialNodes",
                "selectedNode",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodSchedulingSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSchedulingSpec",
            self.potential_nodes.as_ref().map_or(0, |_| 1) +
            self.selected_node.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.potential_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "potentialNodes", value)?;
        }
        if let Some(value) = &self.selected_node {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectedNode", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodSchedulingSpec {
    fn schema_name() -> String {
        "io.k8s.api.resource.v1alpha1.PodSchedulingSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodSchedulingSpec describes where resources for the Pod are needed.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "potentialNodes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PotentialNodes lists nodes where the Pod might be able to run.\n\nThe size of this field is limited to 128. This is large enough for many clusters. Larger clusters may need more attempts to find a node that suits all pending resources. This may get increased in the future, but not reduced.".to_owned()),
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
                    (
                        "selectedNode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SelectedNode is the node for which allocation of ResourceClaims that are referenced by the Pod and that use \"WaitForFirstConsumer\" allocation is to be attempted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
