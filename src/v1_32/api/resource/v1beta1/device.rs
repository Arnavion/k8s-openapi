// Generated from definition io.k8s.api.resource.v1beta1.Device

/// Device represents one individual hardware instance that can be selected based on its attributes. Besides the name, exactly one field must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Device {
    /// Basic defines one device instance.
    pub basic: Option<crate::api::resource::v1beta1::BasicDevice>,

    /// Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.
    pub name: std::string::String,
}

impl crate::DeepMerge for Device {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.basic, other.basic);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Device {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_basic,
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
                            "basic" => Field::Key_basic,
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
            type Value = Device;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Device")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_basic: Option<crate::api::resource::v1beta1::BasicDevice> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_basic => value_basic = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Device {
                    basic: value_basic,
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Device",
            &[
                "basic",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Device",
            1 +
            self.basic.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.basic {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "basic", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Device {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.Device".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Device represents one individual hardware instance that can be selected based on its attributes. Besides the name, exactly one field must be set.",
            "type": "object",
            "properties": {
                "basic": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::BasicDevice>();
                    schema_obj.ensure_object().insert("description".into(), "Basic defines one device instance.".into());
                    schema_obj
                }),
                "name": {
                    "description": "Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
