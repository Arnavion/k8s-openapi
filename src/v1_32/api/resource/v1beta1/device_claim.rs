// Generated from definition io.k8s.api.resource.v1beta1.DeviceClaim

/// DeviceClaim defines how to request devices with a ResourceClaim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceClaim {
    /// This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim.
    pub config: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceClaimConfiguration>>,

    /// These constraints must be satisfied by the set of devices that get allocated for the claim.
    pub constraints: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceConstraint>>,

    /// Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated.
    pub requests: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceRequest>>,
}

impl crate::DeepMerge for DeviceClaim {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.config, other.config);
        crate::merge_strategies::list::atomic(&mut self.constraints, other.constraints);
        crate::merge_strategies::list::atomic(&mut self.requests, other.requests);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceClaim {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config,
            Key_constraints,
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
                            "config" => Field::Key_config,
                            "constraints" => Field::Key_constraints,
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
            type Value = DeviceClaim;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceClaim")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceClaimConfiguration>> = None;
                let mut value_constraints: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceConstraint>> = None;
                let mut value_requests: Option<std::vec::Vec<crate::api::resource::v1beta1::DeviceRequest>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config => value_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_constraints => value_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceClaim {
                    config: value_config,
                    constraints: value_constraints,
                    requests: value_requests,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceClaim",
            &[
                "config",
                "constraints",
                "requests",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceClaim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceClaim",
            self.config.as_ref().map_or(0, |_| 1) +
            self.constraints.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "constraints", value)?;
        }
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceClaim {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta1.DeviceClaim".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceClaim defines how to request devices with a ResourceClaim.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "config".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceClaimConfiguration>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "constraints".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("These constraints must be satisfied by the set of devices that get allocated for the claim.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceConstraint>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "requests".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceRequest>()))),
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
