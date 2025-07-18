// Generated from definition io.k8s.api.resource.v1alpha3.CounterSet

/// CounterSet defines a named set of counters that are available to be used by devices defined in the ResourceSlice.
///
/// The counters are not allocatable by themselves, but can be referenced by devices. When a device is allocated, the portion of counters it uses will no longer be available for use by other devices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CounterSet {
    /// Counters defines the counters that will be consumed by the device. The name of each counter must be unique in that set and must be a DNS label.
    ///
    /// To ensure this uniqueness, capacities defined by the vendor must be listed without the driver name as domain prefix in their name. All others must be listed with their domain prefix.
    ///
    /// The maximum number of counters is 32.
    pub counters: std::collections::BTreeMap<std::string::String, crate::api::resource::v1alpha3::Counter>,

    /// CounterSet is the name of the set from which the counters defined will be consumed.
    pub name: std::string::String,
}

impl crate::DeepMerge for CounterSet {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.counters, other.counters, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CounterSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_counters,
            Key_name,
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
                            "counters" => Field::Key_counters,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CounterSet;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CounterSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_counters: Option<std::collections::BTreeMap<std::string::String, crate::api::resource::v1alpha3::Counter>> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_counters => value_counters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CounterSet {
                    counters: value_counters.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CounterSet",
            &[
                "counters",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CounterSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CounterSet",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "counters", &self.counters)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CounterSet {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.CounterSet".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CounterSet defines a named set of counters that are available to be used by devices defined in the ResourceSlice.\n\nThe counters are not allocatable by themselves, but can be referenced by devices. When a device is allocated, the portion of counters it uses will no longer be available for use by other devices.",
            "type": "object",
            "properties": {
                "counters": {
                    "description": "Counters defines the counters that will be consumed by the device. The name of each counter must be unique in that set and must be a DNS label.\n\nTo ensure this uniqueness, capacities defined by the vendor must be listed without the driver name as domain prefix in their name. All others must be listed with their domain prefix.\n\nThe maximum number of counters is 32.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::api::resource::v1alpha3::Counter>()),
                },
                "name": {
                    "description": "CounterSet is the name of the set from which the counters defined will be consumed.",
                    "type": "string",
                },
            },
            "required": [
                "counters",
                "name",
            ],
        })
    }
}
