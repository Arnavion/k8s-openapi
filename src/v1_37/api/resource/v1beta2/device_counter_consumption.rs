// Generated from definition io.k8s.api.resource.v1beta2.DeviceCounterConsumption

/// DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceCounterConsumption {
    /// CompatibilityGroups is a list of opaque group names for this counter set consumption.
    ///
    /// Devices that consume counters from the same counter set may only be allocated at the same time ("co-allocated") if they all share at least one common group: the intersection of the CompatibilityGroups of all co-allocated devices on that counter set must be non-empty. Devices that consume from different counter sets are never compared via this field.
    ///
    /// An unset field, an explicit nil, and an empty list are equivalent and mean "no groups": such a device is only co-allocatable with sibling devices on the same counter set that also have no groups, and is never co-allocatable with a device that declares one or more groups.
    ///
    /// Group names are opaque and meaningful only within the publishing driver's pool.
    ///
    /// The maximum number of groups is 2, and the names must be unique.
    pub compatibility_groups: Option<std::vec::Vec<std::string::String>>,

    /// CounterSet is the name of the set from which the counters defined will be consumed.
    pub counter_set: std::string::String,

    /// Counters defines the counters that will be consumed by the device.
    ///
    /// The maximum number of counters is 32.
    pub counters: std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta2::Counter>,
}

impl crate::DeepMerge for DeviceCounterConsumption {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.compatibility_groups, other.compatibility_groups);
        crate::DeepMerge::merge_from(&mut self.counter_set, other.counter_set);
        crate::merge_strategies::map::granular(&mut self.counters, other.counters, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceCounterConsumption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_compatibility_groups,
            Key_counter_set,
            Key_counters,
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
                            "compatibilityGroups" => Field::Key_compatibility_groups,
                            "counterSet" => Field::Key_counter_set,
                            "counters" => Field::Key_counters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceCounterConsumption;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceCounterConsumption")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_compatibility_groups: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_counter_set: Option<std::string::String> = None;
                let mut value_counters: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta2::Counter>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_compatibility_groups => value_compatibility_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_counter_set => value_counter_set = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_counters => value_counters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceCounterConsumption {
                    compatibility_groups: value_compatibility_groups,
                    counter_set: value_counter_set.unwrap_or_default(),
                    counters: value_counters.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceCounterConsumption",
            &[
                "compatibilityGroups",
                "counterSet",
                "counters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceCounterConsumption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceCounterConsumption",
            2 +
            self.compatibility_groups.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.compatibility_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "compatibilityGroups", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "counterSet", &self.counter_set)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "counters", &self.counters)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceCounterConsumption {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceCounterConsumption".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.",
            "type": "object",
            "properties": {
                "compatibilityGroups": {
                    "description": "CompatibilityGroups is a list of opaque group names for this counter set consumption.\n\nDevices that consume counters from the same counter set may only be allocated at the same time (\"co-allocated\") if they all share at least one common group: the intersection of the CompatibilityGroups of all co-allocated devices on that counter set must be non-empty. Devices that consume from different counter sets are never compared via this field.\n\nAn unset field, an explicit nil, and an empty list are equivalent and mean \"no groups\": such a device is only co-allocatable with sibling devices on the same counter set that also have no groups, and is never co-allocatable with a device that declares one or more groups.\n\nGroup names are opaque and meaningful only within the publishing driver's pool.\n\nThe maximum number of groups is 2, and the names must be unique.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "counterSet": {
                    "description": "CounterSet is the name of the set from which the counters defined will be consumed.",
                    "type": "string",
                },
                "counters": {
                    "description": "Counters defines the counters that will be consumed by the device.\n\nThe maximum number of counters is 32.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::api::resource::v1beta2::Counter>()),
                },
            },
            "required": [
                "counterSet",
                "counters",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for DeviceCounterConsumption {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta2.DeviceCounterConsumption".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "compatibilityGroups".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("CompatibilityGroups is a list of opaque group names for this counter set consumption.\n\nDevices that consume counters from the same counter set may only be allocated at the same time (\"co-allocated\") if they all share at least one common group: the intersection of the CompatibilityGroups of all co-allocated devices on that counter set must be non-empty. Devices that consume from different counter sets are never compared via this field.\n\nAn unset field, an explicit nil, and an empty list are equivalent and mean \"no groups\": such a device is only co-allocatable with sibling devices on the same counter set that also have no groups, and is never co-allocatable with a device that declares one or more groups.\n\nGroup names are opaque and meaningful only within the publishing driver's pool.\n\nThe maximum number of groups is 2, and the names must be unique.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                                        instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "counterSet".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("CounterSet is the name of the set from which the counters defined will be consumed.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "counters".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Counters defines the counters that will be consumed by the device.\n\nThe maximum number of counters is 32.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1beta2::Counter>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "counterSet".into(),
                    "counters".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
