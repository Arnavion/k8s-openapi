// Generated from definition io.k8s.api.resource.v1beta2.DeviceCapacity

/// DeviceCapacity describes a quantity associated with a device.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceCapacity {
    /// RequestPolicy defines how this DeviceCapacity must be consumed when the device is allowed to be shared by multiple allocations.
    ///
    /// The Device must have allowMultipleAllocations set to true in order to set a requestPolicy.
    ///
    /// If unset, capacity requests are unconstrained: requests can consume any amount of capacity, as long as the total consumed across all allocations does not exceed the device's defined capacity. If request is also unset, default is the full capacity value.
    pub request_policy: Option<crate::api::resource::v1beta2::CapacityRequestPolicy>,

    /// Value defines how much of a certain capacity that device has.
    ///
    /// This field reflects the fixed total capacity and does not change. The consumed amount is tracked separately by scheduler and does not affect this value.
    pub value: crate::apimachinery::pkg::api::resource::Quantity,
}

impl crate::DeepMerge for DeviceCapacity {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.request_policy, other.request_policy);
        crate::DeepMerge::merge_from(&mut self.value, other.value);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceCapacity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_request_policy,
            Key_value,
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
                            "requestPolicy" => Field::Key_request_policy,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceCapacity;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceCapacity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_request_policy: Option<crate::api::resource::v1beta2::CapacityRequestPolicy> = None;
                let mut value_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_request_policy => value_request_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceCapacity {
                    request_policy: value_request_policy,
                    value: value_value.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceCapacity",
            &[
                "requestPolicy",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceCapacity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceCapacity",
            1 +
            self.request_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.request_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requestPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceCapacity {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceCapacity".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceCapacity describes a quantity associated with a device.",
            "type": "object",
            "properties": {
                "requestPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::CapacityRequestPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "RequestPolicy defines how this DeviceCapacity must be consumed when the device is allowed to be shared by multiple allocations.\n\nThe Device must have allowMultipleAllocations set to true in order to set a requestPolicy.\n\nIf unset, capacity requests are unconstrained: requests can consume any amount of capacity, as long as the total consumed across all allocations does not exceed the device's defined capacity. If request is also unset, default is the full capacity value.".into());
                    schema_obj
                }),
                "value": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Value defines how much of a certain capacity that device has.\n\nThis field reflects the fixed total capacity and does not change. The consumed amount is tracked separately by scheduler and does not affect this value.".into());
                    schema_obj
                }),
            },
            "required": [
                "value",
            ],
        })
    }
}
