// Generated from definition io.k8s.api.core.v1.ResourceRequirements

/// ResourceRequirements describes the compute resource requirements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRequirements {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container.
    ///
    /// This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.
    ///
    /// This field is immutable. It can only be set for containers.
    pub claims: Option<std::vec::Vec<crate::api::core::v1::ResourceClaim>>,

    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    pub limits: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    pub requests: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,
}

impl crate::DeepMerge for ResourceRequirements {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.claims,
            other.claims,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::map::granular(&mut self.limits, other.limits, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.requests, other.requests, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceRequirements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_claims,
            Key_limits,
            Key_requests,
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
                            "claims" => Field::Key_claims,
                            "limits" => Field::Key_limits,
                            "requests" => Field::Key_requests,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceRequirements;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceRequirements")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_claims: Option<std::vec::Vec<crate::api::core::v1::ResourceClaim>> = None;
                let mut value_limits: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_requests: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_claims => value_claims = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_limits => value_limits = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRequirements {
                    claims: value_claims,
                    limits: value_limits,
                    requests: value_requests,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceRequirements",
            &[
                "claims",
                "limits",
                "requests",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceRequirements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceRequirements",
            self.claims.as_ref().map_or(0, |_| 1) +
            self.limits.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.claims {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "claims", value)?;
        }
        if let Some(value) = &self.limits {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "limits", value)?;
        }
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceRequirements {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ResourceRequirements".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceRequirements describes the compute resource requirements.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "claims".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container.\n\nThis is an alpha field and requires enabling the DynamicResourceAllocation feature gate.\n\nThis field is immutable. It can only be set for containers.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::ResourceClaim>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "limits".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "requests".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
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
