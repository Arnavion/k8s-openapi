// Generated from definition io.k8s.api.resource.v1beta1.CapacityRequestPolicyRange

/// CapacityRequestPolicyRange defines a valid range for consumable capacity values.
///
///   - If the requested amount is less than Min, it is rounded up to the Min value.
///   - If Step is set and the requested amount is between Min and Max but not aligned with Step,
///     it will be rounded up to the next value equal to Min + (n * Step).
///   - If Step is not set, the requested amount is used as-is if it falls within the range Min to Max (if set).
///   - If the requested or rounded amount exceeds Max (if set), the request does not satisfy the policy,
///     and the device cannot be allocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapacityRequestPolicyRange {
    /// Max defines the upper limit for capacity that can be requested.
    ///
    /// Max must be less than or equal to the capacity value. Min and requestPolicy.default must be less than or equal to the maximum.
    pub max: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// Min specifies the minimum capacity allowed for a consumption request.
    ///
    /// Min must be greater than or equal to zero, and less than or equal to the capacity value. requestPolicy.default must be more than or equal to the minimum.
    pub min: crate::apimachinery::pkg::api::resource::Quantity,

    /// Step defines the step size between valid capacity amounts within the range.
    ///
    /// Max (if set) and requestPolicy.default must be a multiple of Step. Min + Step must be less than or equal to the capacity value.
    pub step: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl crate::DeepMerge for CapacityRequestPolicyRange {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.max, other.max);
        crate::DeepMerge::merge_from(&mut self.min, other.min);
        crate::DeepMerge::merge_from(&mut self.step, other.step);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CapacityRequestPolicyRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max,
            Key_min,
            Key_step,
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
                            "max" => Field::Key_max,
                            "min" => Field::Key_min,
                            "step" => Field::Key_step,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CapacityRequestPolicyRange;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CapacityRequestPolicyRange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_min: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_step: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max => value_max = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min => value_min = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_step => value_step = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CapacityRequestPolicyRange {
                    max: value_max,
                    min: value_min.unwrap_or_default(),
                    step: value_step,
                })
            }
        }

        deserializer.deserialize_struct(
            "CapacityRequestPolicyRange",
            &[
                "max",
                "min",
                "step",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CapacityRequestPolicyRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CapacityRequestPolicyRange",
            1 +
            self.max.as_ref().map_or(0, |_| 1) +
            self.step.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "max", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "min", &self.min)?;
        if let Some(value) = &self.step {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "step", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CapacityRequestPolicyRange {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.CapacityRequestPolicyRange".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CapacityRequestPolicyRange defines a valid range for consumable capacity values.\n\n  - If the requested amount is less than Min, it is rounded up to the Min value.\n  - If Step is set and the requested amount is between Min and Max but not aligned with Step,\n    it will be rounded up to the next value equal to Min + (n * Step).\n  - If Step is not set, the requested amount is used as-is if it falls within the range Min to Max (if set).\n  - If the requested or rounded amount exceeds Max (if set), the request does not satisfy the policy,\n    and the device cannot be allocated.",
            "type": "object",
            "properties": {
                "max": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Max defines the upper limit for capacity that can be requested.\n\nMax must be less than or equal to the capacity value. Min and requestPolicy.default must be less than or equal to the maximum.".into());
                    schema_obj
                }),
                "min": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Min specifies the minimum capacity allowed for a consumption request.\n\nMin must be greater than or equal to zero, and less than or equal to the capacity value. requestPolicy.default must be more than or equal to the minimum.".into());
                    schema_obj
                }),
                "step": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Step defines the step size between valid capacity amounts within the range.\n\nMax (if set) and requestPolicy.default must be a multiple of Step. Min + Step must be less than or equal to the capacity value.".into());
                    schema_obj
                }),
            },
            "required": [
                "min",
            ],
        })
    }
}
