// Generated from definition io.k8s.api.resource.v1alpha3.ShareableSummaryStatus

/// ShareableSummaryStatus reports aggregate capacity for a pool that contains devices with AllowMultipleAllocations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShareableSummaryStatus {
    /// Capacity reports aggregate total, consumed, and available amounts per shareable capacity key across the pool.
    pub capacity: Option<std::vec::Vec<crate::api::resource::v1alpha3::ShareableCapacityStatus>>,

    /// FullyAvailableDevices is the number of shareable devices with no capacity consumed.
    pub fully_available_devices: i32,

    /// PartiallyAvailableDevices is the number of shareable devices with some but not all capacity consumed.
    pub partially_available_devices: i32,
}

impl crate::DeepMerge for ShareableSummaryStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.capacity, other.capacity);
        crate::DeepMerge::merge_from(&mut self.fully_available_devices, other.fully_available_devices);
        crate::DeepMerge::merge_from(&mut self.partially_available_devices, other.partially_available_devices);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ShareableSummaryStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_capacity,
            Key_fully_available_devices,
            Key_partially_available_devices,
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
                            "capacity" => Field::Key_capacity,
                            "fullyAvailableDevices" => Field::Key_fully_available_devices,
                            "partiallyAvailableDevices" => Field::Key_partially_available_devices,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ShareableSummaryStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ShareableSummaryStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_capacity: Option<std::vec::Vec<crate::api::resource::v1alpha3::ShareableCapacityStatus>> = None;
                let mut value_fully_available_devices: Option<i32> = None;
                let mut value_partially_available_devices: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fully_available_devices => value_fully_available_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_partially_available_devices => value_partially_available_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ShareableSummaryStatus {
                    capacity: value_capacity,
                    fully_available_devices: value_fully_available_devices.unwrap_or_default(),
                    partially_available_devices: value_partially_available_devices.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ShareableSummaryStatus",
            &[
                "capacity",
                "fullyAvailableDevices",
                "partiallyAvailableDevices",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ShareableSummaryStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ShareableSummaryStatus",
            2 +
            self.capacity.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fullyAvailableDevices", &self.fully_available_devices)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "partiallyAvailableDevices", &self.partially_available_devices)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ShareableSummaryStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.ShareableSummaryStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ShareableSummaryStatus reports aggregate capacity for a pool that contains devices with AllowMultipleAllocations.",
            "type": "object",
            "properties": {
                "capacity": {
                    "description": "Capacity reports aggregate total, consumed, and available amounts per shareable capacity key across the pool.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1alpha3::ShareableCapacityStatus>()),
                },
                "fullyAvailableDevices": {
                    "description": "FullyAvailableDevices is the number of shareable devices with no capacity consumed.",
                    "type": "integer",
                    "format": "int32",
                },
                "partiallyAvailableDevices": {
                    "description": "PartiallyAvailableDevices is the number of shareable devices with some but not all capacity consumed.",
                    "type": "integer",
                    "format": "int32",
                },
            },
            "required": [
                "fullyAvailableDevices",
                "partiallyAvailableDevices",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for ShareableSummaryStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.ShareableSummaryStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("ShareableSummaryStatus reports aggregate capacity for a pool that contains devices with AllowMultipleAllocations.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "capacity".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Capacity reports aggregate total, consumed, and available amounts per shareable capacity key across the pool.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::ShareableCapacityStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fullyAvailableDevices".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("FullyAvailableDevices is the number of shareable devices with no capacity consumed.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "partiallyAvailableDevices".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("PartiallyAvailableDevices is the number of shareable devices with some but not all capacity consumed.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "fullyAvailableDevices".into(),
                    "partiallyAvailableDevices".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
