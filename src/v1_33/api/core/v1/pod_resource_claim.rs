// Generated from definition io.k8s.api.core.v1.PodResourceClaim

/// PodResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the pod.
///
/// It adds a name to it that uniquely identifies the ResourceClaim inside the Pod. Containers that need access to the ResourceClaim reference it with this name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodResourceClaim {
    /// Name uniquely identifies this resource claim inside the pod. This must be a DNS_LABEL.
    pub name: std::string::String,

    /// ResourceClaimName is the name of a ResourceClaim object in the same namespace as this pod.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    pub resource_claim_name: Option<std::string::String>,

    /// ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this pod.
    ///
    /// The template will be used to create a new ResourceClaim, which will be bound to this pod. When this pod is deleted, the ResourceClaim will also be deleted. The pod name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in pod.status.resourceClaimStatuses.
    ///
    /// This field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    pub resource_claim_template_name: Option<std::string::String>,
}

impl crate::DeepMerge for PodResourceClaim {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.resource_claim_name, other.resource_claim_name);
        crate::DeepMerge::merge_from(&mut self.resource_claim_template_name, other.resource_claim_template_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodResourceClaim {
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
            type Value = PodResourceClaim;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodResourceClaim")
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

                Ok(PodResourceClaim {
                    name: value_name.unwrap_or_default(),
                    resource_claim_name: value_resource_claim_name,
                    resource_claim_template_name: value_resource_claim_template_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodResourceClaim",
            &[
                "name",
                "resourceClaimName",
                "resourceClaimTemplateName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodResourceClaim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodResourceClaim",
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
impl crate::schemars::JsonSchema for PodResourceClaim {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PodResourceClaim".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PodResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the pod.\n\nIt adds a name to it that uniquely identifies the ResourceClaim inside the Pod. Containers that need access to the ResourceClaim reference it with this name.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name uniquely identifies this resource claim inside the pod. This must be a DNS_LABEL.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceClaimName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceClaimName is the name of a ResourceClaim object in the same namespace as this pod.\n\nExactly one of ResourceClaimName and ResourceClaimTemplateName must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceClaimTemplateName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this pod.\n\nThe template will be used to create a new ResourceClaim, which will be bound to this pod. When this pod is deleted, the ResourceClaim will also be deleted. The pod name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in pod.status.resourceClaimStatuses.\n\nThis field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.\n\nExactly one of ResourceClaimName and ResourceClaimTemplateName must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
