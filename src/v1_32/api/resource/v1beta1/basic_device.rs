// Generated from definition io.k8s.api.resource.v1beta1.BasicDevice

/// BasicDevice defines one device instance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BasicDevice {
    /// Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    pub attributes: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceAttribute>>,

    /// Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    pub capacity: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceCapacity>>,
}

impl crate::DeepMerge for BasicDevice {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.attributes, other.attributes, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.capacity, other.capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for BasicDevice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attributes,
            Key_capacity,
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
                            "attributes" => Field::Key_attributes,
                            "capacity" => Field::Key_capacity,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = BasicDevice;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("BasicDevice")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_attributes: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceAttribute>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::DeviceCapacity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attributes => value_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BasicDevice {
                    attributes: value_attributes,
                    capacity: value_capacity,
                })
            }
        }

        deserializer.deserialize_struct(
            "BasicDevice",
            &[
                "attributes",
                "capacity",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for BasicDevice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BasicDevice",
            self.attributes.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attributes", value)?;
        }
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BasicDevice {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta1.BasicDevice".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("BasicDevice defines one device instance.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "attributes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.\n\nThe maximum number of attributes and capacities combined is 32.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceAttribute>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacity".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.\n\nThe maximum number of attributes and capacities combined is 32.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta1::DeviceCapacity>())),
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
