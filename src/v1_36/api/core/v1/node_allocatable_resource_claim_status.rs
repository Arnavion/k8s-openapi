// Generated from definition io.k8s.api.core.v1.NodeAllocatableResourceClaimStatus

/// NodeAllocatableResourceClaimStatus describes the status of node allocatable resources allocated via DRA.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAllocatableResourceClaimStatus {
    /// Containers lists the names of all containers in this pod that reference the claim.
    pub containers: Option<std::vec::Vec<std::string::String>>,

    /// ResourceClaimName is the resource claim referenced by the pod that resulted in this node allocatable resource allocation.
    pub resource_claim_name: std::string::String,

    /// Resources is a map of the node-allocatable resource name to the aggregate quantity allocated to the claim.
    pub resources: std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>,
}

impl crate::DeepMerge for NodeAllocatableResourceClaimStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::set(&mut self.containers, other.containers);
        crate::DeepMerge::merge_from(&mut self.resource_claim_name, other.resource_claim_name);
        crate::merge_strategies::map::granular(&mut self.resources, other.resources, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeAllocatableResourceClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_containers,
            Key_resource_claim_name,
            Key_resources,
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
                            "containers" => Field::Key_containers,
                            "resourceClaimName" => Field::Key_resource_claim_name,
                            "resources" => Field::Key_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAllocatableResourceClaimStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeAllocatableResourceClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_containers: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_resource_claim_name: Option<std::string::String> = None;
                let mut value_resources: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_containers => value_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claim_name => value_resource_claim_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAllocatableResourceClaimStatus {
                    containers: value_containers,
                    resource_claim_name: value_resource_claim_name.unwrap_or_default(),
                    resources: value_resources.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAllocatableResourceClaimStatus",
            &[
                "containers",
                "resourceClaimName",
                "resources",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAllocatableResourceClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAllocatableResourceClaimStatus",
            2 +
            self.containers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.containers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containers", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaimName", &self.resource_claim_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAllocatableResourceClaimStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NodeAllocatableResourceClaimStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeAllocatableResourceClaimStatus describes the status of node allocatable resources allocated via DRA.",
            "type": "object",
            "properties": {
                "containers": {
                    "description": "Containers lists the names of all containers in this pod that reference the claim.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "resourceClaimName": {
                    "description": "ResourceClaimName is the resource claim referenced by the pod that resulted in this node allocatable resource allocation.",
                    "type": "string",
                },
                "resources": {
                    "description": "Resources is a map of the node-allocatable resource name to the aggregate quantity allocated to the claim.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
            },
            "required": [
                "resourceClaimName",
                "resources",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodeAllocatableResourceClaimStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.NodeAllocatableResourceClaimStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodeAllocatableResourceClaimStatus describes the status of node allocatable resources allocated via DRA.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "containers".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Containers lists the names of all containers in this pod that reference the claim.".into()),
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
                    (
                        "resourceClaimName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("ResourceClaimName is the resource claim referenced by the pod that resulted in this node allocatable resource allocation.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resources".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Resources is a map of the node-allocatable resource name to the aggregate quantity allocated to the claim.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "resourceClaimName".into(),
                    "resources".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
