// Generated from definition io.k8s.api.resource.v1alpha3.DeviceClassSpec

/// DeviceClassSpec is used in a \[DeviceClass\] to define what can be allocated and how to configure it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceClassSpec {
    /// Config defines configuration parameters that apply to each device that is claimed via this class. Some classses may potentially be satisfied by multiple drivers, so each instance of a vendor configuration applies to exactly one driver.
    ///
    /// They are passed to the driver, but are not considered while allocating the claim.
    pub config: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceClassConfiguration>>,

    /// Each selector must be satisfied by a device which is claimed via this class.
    pub selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>>,
}

impl crate::DeepMerge for DeviceClassSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.config, other.config);
        crate::merge_strategies::list::atomic(&mut self.selectors, other.selectors);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceClassSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config,
            Key_selectors,
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
                            "selectors" => Field::Key_selectors,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceClassSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceClassSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceClassConfiguration>> = None;
                let mut value_selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config => value_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectors => value_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceClassSpec {
                    config: value_config,
                    selectors: value_selectors,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceClassSpec",
            &[
                "config",
                "selectors",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceClassSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceClassSpec",
            self.config.as_ref().map_or(0, |_| 1) +
            self.selectors.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.selectors {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectors", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceClassSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.DeviceClassSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceClassSpec is used in a [DeviceClass] to define what can be allocated and how to configure it.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "config".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Config defines configuration parameters that apply to each device that is claimed via this class. Some classses may potentially be satisfied by multiple drivers, so each instance of a vendor configuration applies to exactly one driver.\n\nThey are passed to the driver, but are not considered while allocating the claim.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::DeviceClassConfiguration>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selectors".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Each selector must be satisfied by a device which is claimed via this class.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::DeviceSelector>()))),
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
