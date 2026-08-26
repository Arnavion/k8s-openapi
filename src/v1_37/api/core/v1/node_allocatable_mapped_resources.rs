// Generated from definition io.k8s.api.core.v1.NodeAllocatableMappedResources

/// NodeAllocatableMappedResources describes mapped node allocatable resource allocations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAllocatableMappedResources {
    /// Name is the name of the resource (e.g., cpu, memory).
    pub name: std::string::String,

    /// Quantity is the total node allocatable resource capacity allocated for the claim. This claim's allocated devices is shared by all the containers referencing the claim. Kubelet adds this value to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.
    pub quantity: crate::apimachinery::pkg::api::resource::Quantity,
}

impl crate::DeepMerge for NodeAllocatableMappedResources {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.quantity, other.quantity);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeAllocatableMappedResources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_quantity,
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
                            "name" => Field::Key_name,
                            "quantity" => Field::Key_quantity,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAllocatableMappedResources;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeAllocatableMappedResources")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_quantity: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_quantity => value_quantity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAllocatableMappedResources {
                    name: value_name.unwrap_or_default(),
                    quantity: value_quantity.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAllocatableMappedResources",
            &[
                "name",
                "quantity",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAllocatableMappedResources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAllocatableMappedResources",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "quantity", &self.quantity)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAllocatableMappedResources {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NodeAllocatableMappedResources".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeAllocatableMappedResources describes mapped node allocatable resource allocations.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "Name is the name of the resource (e.g., cpu, memory).",
                    "type": "string",
                },
                "quantity": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Quantity is the total node allocatable resource capacity allocated for the claim. This claim's allocated devices is shared by all the containers referencing the claim. Kubelet adds this value to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.".into());
                    schema_obj
                }),
            },
            "required": [
                "name",
                "quantity",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodeAllocatableMappedResources {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.NodeAllocatableMappedResources".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodeAllocatableMappedResources describes mapped node allocatable resource allocations.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Name is the name of the resource (e.g., cpu, memory).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "quantity".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Quantity is the total node allocatable resource capacity allocated for the claim. This claim's allocated devices is shared by all the containers referencing the claim. Kubelet adds this value to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "quantity".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
