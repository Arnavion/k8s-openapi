// Generated from definition io.k8s.api.resource.v1beta1.CapacityRequestPolicy

/// CapacityRequestPolicy defines how requests consume device capacity.
///
/// Must not set more than one ValidRequestValues.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapacityRequestPolicy {
    /// Default specifies how much of this capacity is consumed by a request that does not contain an entry for it in DeviceRequest's Capacity.
    pub default: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// ValidRange defines an acceptable quantity value range in consuming requests.
    ///
    /// If this field is set, Default must be defined and it must fall within the defined ValidRange.
    ///
    /// If the requested amount does not fall within the defined range, the request violates the policy, and this device cannot be allocated.
    ///
    /// If the request doesn't contain this capacity entry, Default value is used.
    pub valid_range: Option<crate::api::resource::v1beta1::CapacityRequestPolicyRange>,

    /// ValidValues defines a set of acceptable quantity values in consuming requests.
    ///
    /// Must not contain more than 10 entries. Must be sorted in ascending order.
    ///
    /// If this field is set, Default must be defined and it must be included in ValidValues list.
    ///
    /// If the requested amount does not match any valid value but smaller than some valid values, the scheduler calculates the smallest valid value that is greater than or equal to the request. That is: min(ceil(requestedValue) ∈ validValues), where requestedValue ≤ max(validValues).
    ///
    /// If the requested amount exceeds all valid values, the request violates the policy, and this device cannot be allocated.
    pub valid_values: Option<std::vec::Vec<crate::apimachinery::pkg::api::resource::Quantity>>,
}

impl crate::DeepMerge for CapacityRequestPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.default, other.default);
        crate::DeepMerge::merge_from(&mut self.valid_range, other.valid_range);
        crate::merge_strategies::list::atomic(&mut self.valid_values, other.valid_values);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CapacityRequestPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default,
            Key_valid_range,
            Key_valid_values,
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
                            "default" => Field::Key_default,
                            "validRange" => Field::Key_valid_range,
                            "validValues" => Field::Key_valid_values,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CapacityRequestPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CapacityRequestPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_valid_range: Option<crate::api::resource::v1beta1::CapacityRequestPolicyRange> = None;
                let mut value_valid_values: Option<std::vec::Vec<crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default => value_default = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_valid_range => value_valid_range = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_valid_values => value_valid_values = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CapacityRequestPolicy {
                    default: value_default,
                    valid_range: value_valid_range,
                    valid_values: value_valid_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "CapacityRequestPolicy",
            &[
                "default",
                "validRange",
                "validValues",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CapacityRequestPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CapacityRequestPolicy",
            self.default.as_ref().map_or(0, |_| 1) +
            self.valid_range.as_ref().map_or(0, |_| 1) +
            self.valid_values.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "default", value)?;
        }
        if let Some(value) = &self.valid_range {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validRange", value)?;
        }
        if let Some(value) = &self.valid_values {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validValues", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CapacityRequestPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.CapacityRequestPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CapacityRequestPolicy defines how requests consume device capacity.\n\nMust not set more than one ValidRequestValues.",
            "type": "object",
            "properties": {
                "default": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Default specifies how much of this capacity is consumed by a request that does not contain an entry for it in DeviceRequest's Capacity.".into());
                    schema_obj
                }),
                "validRange": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::CapacityRequestPolicyRange>();
                    schema_obj.ensure_object().insert("description".into(), "ValidRange defines an acceptable quantity value range in consuming requests.\n\nIf this field is set, Default must be defined and it must fall within the defined ValidRange.\n\nIf the requested amount does not fall within the defined range, the request violates the policy, and this device cannot be allocated.\n\nIf the request doesn't contain this capacity entry, Default value is used.".into());
                    schema_obj
                }),
                "validValues": {
                    "description": "ValidValues defines a set of acceptable quantity values in consuming requests.\n\nMust not contain more than 10 entries. Must be sorted in ascending order.\n\nIf this field is set, Default must be defined and it must be included in ValidValues list.\n\nIf the requested amount does not match any valid value but smaller than some valid values, the scheduler calculates the smallest valid value that is greater than or equal to the request. That is: min(ceil(requestedValue) ∈ validValues), where requestedValue ≤ max(validValues).\n\nIf the requested amount exceeds all valid values, the request violates the policy, and this device cannot be allocated.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
            },
        })
    }
}
