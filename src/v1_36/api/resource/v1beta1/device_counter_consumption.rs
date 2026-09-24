// Generated from definition io.k8s.api.resource.v1beta1.DeviceCounterConsumption

/// DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceCounterConsumption {
    /// CounterSet is the name of the set from which the counters defined will be consumed.
    pub counter_set: std::string::String,

    /// Counters defines the counters that will be consumed by the device.
    ///
    /// The maximum number of counters is 32.
    pub counters: std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::Counter>,
}

impl crate::DeepMerge for DeviceCounterConsumption {
    fn merge_from(&mut self, other: Self) {
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
                let mut value_counter_set: Option<std::string::String> = None;
                let mut value_counters: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1beta1::Counter>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_counter_set => value_counter_set = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_counters => value_counters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceCounterConsumption {
                    counter_set: value_counter_set.unwrap_or_default(),
                    counters: value_counters.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceCounterConsumption",
            &[
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
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "counterSet", &self.counter_set)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "counters", &self.counters)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceCounterConsumption {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.DeviceCounterConsumption".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.",
            "type": "object",
            "properties": {
                "counterSet": {
                    "description": "CounterSet is the name of the set from which the counters defined will be consumed.",
                    "type": "string",
                },
                "counters": {
                    "description": "Counters defines the counters that will be consumed by the device.\n\nThe maximum number of counters is 32.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::api::resource::v1beta1::Counter>()),
                },
            },
            "required": [
                "counterSet",
                "counters",
            ],
        })
    }
}
