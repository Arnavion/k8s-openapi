// Generated from definition io.k8s.api.resource.v1beta2.CapacityRequirements

/// CapacityRequirements defines the capacity requirements for a specific device request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapacityRequirements {
    /// Requests represent individual device resource requests for distinct resources, all of which must be provided by the device.
    ///
    /// This value is used as an additional filtering condition against the available capacity on the device. This is semantically equivalent to a CEL selector with `device.capacity\[\<domain\>\].\<name\>.compareTo(quantity(\<request quantity\>)) \>= 0`. For example, device.capacity\['test-driver.cdi.k8s.io'\].counters.compareTo(quantity('2')) \>= 0.
    ///
    /// When a requestPolicy is defined, the requested amount is adjusted upward to the nearest valid value based on the policy. If the requested amount cannot be adjusted to a valid value—because it exceeds what the requestPolicy allows— the device is considered ineligible for allocation.
    ///
    /// For any capacity that is not explicitly requested: - If no requestPolicy is set, the default consumed capacity is equal to the full device capacity
    ///   (i.e., the whole device is claimed).
    /// - If a requestPolicy is set, the default consumed capacity is determined according to that policy.
    ///
    /// If the device allows multiple allocation, the aggregated amount across all requests must not exceed the capacity value. The consumed capacity, which may be adjusted based on the requestPolicy if defined, is recorded in the resource claim’s status.devices\[*\].consumedCapacity field.
    pub requests: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,
}

impl crate::DeepMerge for CapacityRequirements {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.requests, other.requests, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for CapacityRequirements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
            type Value = CapacityRequirements;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CapacityRequirements")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_requests: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CapacityRequirements {
                    requests: value_requests,
                })
            }
        }

        deserializer.deserialize_struct(
            "CapacityRequirements",
            &[
                "requests",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CapacityRequirements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CapacityRequirements",
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CapacityRequirements {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.CapacityRequirements".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CapacityRequirements defines the capacity requirements for a specific device request.",
            "type": "object",
            "properties": {
                "requests": {
                    "description": "Requests represent individual device resource requests for distinct resources, all of which must be provided by the device.\n\nThis value is used as an additional filtering condition against the available capacity on the device. This is semantically equivalent to a CEL selector with `device.capacity[<domain>].<name>.compareTo(quantity(<request quantity>)) >= 0`. For example, device.capacity['test-driver.cdi.k8s.io'].counters.compareTo(quantity('2')) >= 0.\n\nWhen a requestPolicy is defined, the requested amount is adjusted upward to the nearest valid value based on the policy. If the requested amount cannot be adjusted to a valid value—because it exceeds what the requestPolicy allows— the device is considered ineligible for allocation.\n\nFor any capacity that is not explicitly requested: - If no requestPolicy is set, the default consumed capacity is equal to the full device capacity\n  (i.e., the whole device is claimed).\n- If a requestPolicy is set, the default consumed capacity is determined according to that policy.\n\nIf the device allows multiple allocation, the aggregated amount across all requests must not exceed the capacity value. The consumed capacity, which may be adjusted based on the requestPolicy if defined, is recorded in the resource claim’s status.devices[*].consumedCapacity field.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
            },
        })
    }
}
