// Generated from definition io.k8s.api.resource.v1beta2.DeviceAllocationResult

/// DeviceAllocationResult is the result of allocating devices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceAllocationResult {
    /// This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.
    ///
    /// This includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters.
    pub config: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceAllocationConfiguration>>,

    /// Results lists all allocated devices.
    pub results: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceRequestAllocationResult>>,
}

impl crate::DeepMerge for DeviceAllocationResult {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.config, other.config);
        crate::merge_strategies::list::atomic(&mut self.results, other.results);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceAllocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config,
            Key_results,
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
                            "results" => Field::Key_results,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceAllocationResult;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceAllocationResult")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceAllocationConfiguration>> = None;
                let mut value_results: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceRequestAllocationResult>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config => value_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_results => value_results = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceAllocationResult {
                    config: value_config,
                    results: value_results,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceAllocationResult",
            &[
                "config",
                "results",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceAllocationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceAllocationResult",
            self.config.as_ref().map_or(0, |_| 1) +
            self.results.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.results {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "results", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceAllocationResult {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta2.DeviceAllocationResult".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceAllocationResult is the result of allocating devices.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "config".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.\n\nThis includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta2::DeviceAllocationConfiguration>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "results".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Results lists all allocated devices.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta2::DeviceRequestAllocationResult>()))),
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
