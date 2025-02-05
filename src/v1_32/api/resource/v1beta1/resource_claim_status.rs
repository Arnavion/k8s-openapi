// Generated from definition io.k8s.api.resource.v1beta1.ResourceClaimStatus

/// ResourceClaimStatus tracks whether the resource has been allocated and what the result of that was.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimStatus {
    /// Allocation is set once the claim has been allocated successfully.
    pub allocation: Option<crate::api::resource::v1beta1::AllocationResult>,

    /// Devices contains the status of each device allocated for this claim, as reported by the driver. This can include driver-specific information. Entries are owned by their respective drivers.
    pub devices: Option<std::vec::Vec<crate::api::resource::v1beta1::AllocatedDeviceStatus>>,

    /// ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started. A claim that is in use or might be in use because it has been reserved must not get deallocated.
    ///
    /// In a cluster with multiple scheduler instances, two pods might get scheduled concurrently by different schedulers. When they reference the same ResourceClaim which already has reached its maximum number of consumers, only one pod can be scheduled.
    ///
    /// Both schedulers try to add their pod to the claim.status.reservedFor field, but only the update that reaches the API server first gets stored. The other one fails with an error and the scheduler which issued it knows that it must put the pod back into the queue, waiting for the ResourceClaim to become usable again.
    ///
    /// There can be at most 256 such reservations. This may get increased in the future, but not reduced.
    pub reserved_for: Option<std::vec::Vec<crate::api::resource::v1beta1::ResourceClaimConsumerReference>>,
}

impl crate::DeepMerge for ResourceClaimStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocation, other.allocation);
        crate::merge_strategies::list::map(
            &mut self.devices,
            other.devices,
            &[|lhs, rhs| lhs.driver == rhs.driver, |lhs, rhs| lhs.device == rhs.device, |lhs, rhs| lhs.pool == rhs.pool],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
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
            Key_devices,
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
                            "devices" => Field::Key_devices,
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
                let mut value_allocation: Option<crate::api::resource::v1beta1::AllocationResult> = None;
                let mut value_devices: Option<std::vec::Vec<crate::api::resource::v1beta1::AllocatedDeviceStatus>> = None;
                let mut value_reserved_for: Option<std::vec::Vec<crate::api::resource::v1beta1::ResourceClaimConsumerReference>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocation => value_allocation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_devices => value_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reserved_for => value_reserved_for = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimStatus {
                    allocation: value_allocation,
                    devices: value_devices,
                    reserved_for: value_reserved_for,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimStatus",
            &[
                "allocation",
                "devices",
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
            self.devices.as_ref().map_or(0, |_| 1) +
            self.reserved_for.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocation", value)?;
        }
        if let Some(value) = &self.devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devices", value)?;
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
        "io.k8s.api.resource.v1beta1.ResourceClaimStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClaimStatus tracks whether the resource has been allocated and what the result of that was.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocation".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::AllocationResult>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Allocation is set once the claim has been allocated successfully.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "devices".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Devices contains the status of each device allocated for this claim, as reported by the driver. This can include driver-specific information. Entries are owned by their respective drivers.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::AllocatedDeviceStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reservedFor".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started. A claim that is in use or might be in use because it has been reserved must not get deallocated.\n\nIn a cluster with multiple scheduler instances, two pods might get scheduled concurrently by different schedulers. When they reference the same ResourceClaim which already has reached its maximum number of consumers, only one pod can be scheduled.\n\nBoth schedulers try to add their pod to the claim.status.reservedFor field, but only the update that reaches the API server first gets stored. The other one fails with an error and the scheduler which issued it knows that it must put the pod back into the queue, waiting for the ResourceClaim to become usable again.\n\nThere can be at most 256 such reservations. This may get increased in the future, but not reduced.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::ResourceClaimConsumerReference>()))),
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
