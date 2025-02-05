// Generated from definition io.k8s.api.resource.v1alpha2.ResourceClaimStatus

/// ResourceClaimStatus tracks whether the resource has been allocated and what the resulting attributes are.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimStatus {
    /// Allocation is set by the resource driver once a resource or set of resources has been allocated successfully. If this is not specified, the resources have not been allocated yet.
    pub allocation: Option<crate::api::resource::v1alpha2::AllocationResult>,

    /// DeallocationRequested indicates that a ResourceClaim is to be deallocated.
    ///
    /// The driver then must deallocate this claim and reset the field together with clearing the Allocation field.
    ///
    /// While DeallocationRequested is set, no new consumers may be added to ReservedFor.
    pub deallocation_requested: Option<bool>,

    /// DriverName is a copy of the driver name from the ResourceClass at the time when allocation started.
    pub driver_name: Option<std::string::String>,

    /// ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started.
    ///
    /// There can be at most 32 such reservations. This may get increased in the future, but not reduced.
    pub reserved_for: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceClaimConsumerReference>>,
}

impl crate::DeepMerge for ResourceClaimStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocation, other.allocation);
        crate::DeepMerge::merge_from(&mut self.deallocation_requested, other.deallocation_requested);
        crate::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        crate::merge_strategies::list::map(
            &mut self.reserved_for,
            other.reserved_for,
            &[|lhs, rhs| lhs.uid == rhs.uid],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocation,
            Key_deallocation_requested,
            Key_driver_name,
            Key_reserved_for,
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
                            "allocation" => Field::Key_allocation,
                            "deallocationRequested" => Field::Key_deallocation_requested,
                            "driverName" => Field::Key_driver_name,
                            "reservedFor" => Field::Key_reserved_for,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocation: Option<crate::api::resource::v1alpha2::AllocationResult> = None;
                let mut value_deallocation_requested: Option<bool> = None;
                let mut value_driver_name: Option<std::string::String> = None;
                let mut value_reserved_for: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceClaimConsumerReference>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocation => value_allocation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deallocation_requested => value_deallocation_requested = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver_name => value_driver_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reserved_for => value_reserved_for = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimStatus {
                    allocation: value_allocation,
                    deallocation_requested: value_deallocation_requested,
                    driver_name: value_driver_name,
                    reserved_for: value_reserved_for,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimStatus",
            &[
                "allocation",
                "deallocationRequested",
                "driverName",
                "reservedFor",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimStatus",
            self.allocation.as_ref().map_or(0, |_| 1) +
            self.deallocation_requested.as_ref().map_or(0, |_| 1) +
            self.driver_name.as_ref().map_or(0, |_| 1) +
            self.reserved_for.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocation", value)?;
        }
        if let Some(value) = &self.deallocation_requested {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deallocationRequested", value)?;
        }
        if let Some(value) = &self.driver_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverName", value)?;
        }
        if let Some(value) = &self.reserved_for {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reservedFor", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceClaimStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClaimStatus tracks whether the resource has been allocated and what the resulting attributes are.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocation".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::AllocationResult>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Allocation is set by the resource driver once a resource or set of resources has been allocated successfully. If this is not specified, the resources have not been allocated yet.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "deallocationRequested".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DeallocationRequested indicates that a ResourceClaim is to be deallocated.\n\nThe driver then must deallocate this claim and reset the field together with clearing the Allocation field.\n\nWhile DeallocationRequested is set, no new consumers may be added to ReservedFor.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "driverName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverName is a copy of the driver name from the ResourceClass at the time when allocation started.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reservedFor".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started.\n\nThere can be at most 32 such reservations. This may get increased in the future, but not reduced.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::ResourceClaimConsumerReference>()))),
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
