// Generated from definition io.k8s.api.resource.v1beta2.DeviceCapacity

/// DeviceCapacity describes a quantity associated with a device.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceCapacity {
    /// Value defines how much of a certain device capacity is available.
    pub value: crate::apimachinery::pkg::api::resource::Quantity,
}

impl crate::DeepMerge for DeviceCapacity {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.value, other.value);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceCapacity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
                let mut value_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceCapacity {
                    value: value_value.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceCapacity",
            &[
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
            1,
        )?;
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
                "value": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Value defines how much of a certain device capacity is available.".into());
                    schema_obj
                }),
            },
            "required": [
                "value",
            ],
        })
    }
}
