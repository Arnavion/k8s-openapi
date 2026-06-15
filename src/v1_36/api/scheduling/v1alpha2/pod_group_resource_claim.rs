// Generated from definition io.k8s.api.scheduling.v1alpha2.PodGroupResourceClaim

/// PodGroupResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the PodGroup.
///
/// It adds a name to it that uniquely identifies the ResourceClaim inside the PodGroup. Pods that need access to the ResourceClaim define a matching reference in its own Spec.ResourceClaims. The Pod's claim must match all fields of the PodGroup's claim exactly.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodGroupResourceClaim {
    /// Name uniquely identifies this resource claim inside the PodGroup. This must be a DNS_LABEL.
    pub name: std::string::String,

    /// ResourceClaimName is the name of a ResourceClaim object in the same namespace as this PodGroup. The ResourceClaim will be reserved for the PodGroup instead of its individual pods.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    pub resource_claim_name: Option<std::string::String>,

    /// ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this PodGroup.
    ///
    /// The template will be used to create a new ResourceClaim, which will be bound to this PodGroup. When this PodGroup is deleted, the ResourceClaim will also be deleted. The PodGroup name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in podgroup.status.resourceClaimStatuses.
    ///
    /// This field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    pub resource_claim_template_name: Option<std::string::String>,
}

impl crate::DeepMerge for PodGroupResourceClaim {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.resource_claim_name, other.resource_claim_name);
        crate::DeepMerge::merge_from(&mut self.resource_claim_template_name, other.resource_claim_template_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodGroupResourceClaim {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_resource_claim_name,
            Key_resource_claim_template_name,
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
                            "resourceClaimName" => Field::Key_resource_claim_name,
                            "resourceClaimTemplateName" => Field::Key_resource_claim_template_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodGroupResourceClaim;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodGroupResourceClaim")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_resource_claim_name: Option<std::string::String> = None;
                let mut value_resource_claim_template_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claim_name => value_resource_claim_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claim_template_name => value_resource_claim_template_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodGroupResourceClaim {
                    name: value_name.unwrap_or_default(),
                    resource_claim_name: value_resource_claim_name,
                    resource_claim_template_name: value_resource_claim_template_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodGroupResourceClaim",
            &[
                "name",
                "resourceClaimName",
                "resourceClaimTemplateName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodGroupResourceClaim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodGroupResourceClaim",
            1 +
            self.resource_claim_name.as_ref().map_or(0, |_| 1) +
            self.resource_claim_template_name.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.resource_claim_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaimName", value)?;
        }
        if let Some(value) = &self.resource_claim_template_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaimTemplateName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodGroupResourceClaim {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha2.PodGroupResourceClaim".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodGroupResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the PodGroup.\n\nIt adds a name to it that uniquely identifies the ResourceClaim inside the PodGroup. Pods that need access to the ResourceClaim define a matching reference in its own Spec.ResourceClaims. The Pod's claim must match all fields of the PodGroup's claim exactly.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "Name uniquely identifies this resource claim inside the PodGroup. This must be a DNS_LABEL.",
                    "type": "string",
                },
                "resourceClaimName": {
                    "description": "ResourceClaimName is the name of a ResourceClaim object in the same namespace as this PodGroup. The ResourceClaim will be reserved for the PodGroup instead of its individual pods.\n\nExactly one of ResourceClaimName and ResourceClaimTemplateName must be set.",
                    "type": "string",
                },
                "resourceClaimTemplateName": {
                    "description": "ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this PodGroup.\n\nThe template will be used to create a new ResourceClaim, which will be bound to this PodGroup. When this PodGroup is deleted, the ResourceClaim will also be deleted. The PodGroup name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in podgroup.status.resourceClaimStatuses.\n\nThis field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.\n\nExactly one of ResourceClaimName and ResourceClaimTemplateName must be set.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for PodGroupResourceClaim {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha2.PodGroupResourceClaim".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("PodGroupResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the PodGroup.\n\nIt adds a name to it that uniquely identifies the ResourceClaim inside the PodGroup. Pods that need access to the ResourceClaim define a matching reference in its own Spec.ResourceClaims. The Pod's claim must match all fields of the PodGroup's claim exactly.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Name uniquely identifies this resource claim inside the PodGroup. This must be a DNS_LABEL.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceClaimName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("ResourceClaimName is the name of a ResourceClaim object in the same namespace as this PodGroup. The ResourceClaim will be reserved for the PodGroup instead of its individual pods.\n\nExactly one of ResourceClaimName and ResourceClaimTemplateName must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceClaimTemplateName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this PodGroup.\n\nThe template will be used to create a new ResourceClaim, which will be bound to this PodGroup. When this PodGroup is deleted, the ResourceClaim will also be deleted. The PodGroup name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in podgroup.status.resourceClaimStatuses.\n\nThis field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.\n\nExactly one of ResourceClaimName and ResourceClaimTemplateName must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
