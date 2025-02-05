// Generated from definition io.k8s.api.resource.v1alpha2.ResourceClaimSpec

/// ResourceClaimSpec defines how a resource is to be allocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimSpec {
    /// Allocation can start immediately or when a Pod wants to use the resource. "WaitForFirstConsumer" is the default.
    pub allocation_mode: Option<std::string::String>,

    /// ParametersRef references a separate object with arbitrary parameters that will be used by the driver when allocating a resource for the claim.
    ///
    /// The object must be in the same namespace as the ResourceClaim.
    pub parameters_ref: Option<crate::api::resource::v1alpha2::ResourceClaimParametersReference>,

    /// ResourceClassName references the driver and additional parameters via the name of a ResourceClass that was created as part of the driver deployment.
    pub resource_class_name: std::string::String,
}

impl crate::DeepMerge for ResourceClaimSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocation_mode, other.allocation_mode);
        crate::DeepMerge::merge_from(&mut self.parameters_ref, other.parameters_ref);
        crate::DeepMerge::merge_from(&mut self.resource_class_name, other.resource_class_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocation_mode,
            Key_parameters_ref,
            Key_resource_class_name,
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
                            "allocationMode" => Field::Key_allocation_mode,
                            "parametersRef" => Field::Key_parameters_ref,
                            "resourceClassName" => Field::Key_resource_class_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocation_mode: Option<std::string::String> = None;
                let mut value_parameters_ref: Option<crate::api::resource::v1alpha2::ResourceClaimParametersReference> = None;
                let mut value_resource_class_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocation_mode => value_allocation_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters_ref => value_parameters_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_class_name => value_resource_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimSpec {
                    allocation_mode: value_allocation_mode,
                    parameters_ref: value_parameters_ref,
                    resource_class_name: value_resource_class_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimSpec",
            &[
                "allocationMode",
                "parametersRef",
                "resourceClassName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimSpec",
            1 +
            self.allocation_mode.as_ref().map_or(0, |_| 1) +
            self.parameters_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocation_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocationMode", value)?;
        }
        if let Some(value) = &self.parameters_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parametersRef", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClassName", &self.resource_class_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceClaimSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClaimSpec defines how a resource is to be allocated.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocationMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Allocation can start immediately or when a Pod wants to use the resource. \"WaitForFirstConsumer\" is the default.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "parametersRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::ResourceClaimParametersReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ParametersRef references a separate object with arbitrary parameters that will be used by the driver when allocating a resource for the claim.\n\nThe object must be in the same namespace as the ResourceClaim.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceClassName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceClassName references the driver and additional parameters via the name of a ResourceClass that was created as part of the driver deployment.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "resourceClassName".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
