// Generated from definition io.k8s.api.resource.v1.DeviceTaint

/// The device this taint is attached to has the "effect" on any claim which does not tolerate the taint and, through the claim, to pods using the claim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceTaint {
    /// The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them. Valid effects are NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here.
    pub effect: std::string::String,

    /// The taint key to be applied to a device. Must be a label name.
    pub key: std::string::String,

    /// TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.
    pub time_added: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// The taint value corresponding to the taint key. Must be a label value.
    pub value: Option<std::string::String>,
}

impl crate::DeepMerge for DeviceTaint {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.effect, other.effect);
        crate::DeepMerge::merge_from(&mut self.key, other.key);
        crate::DeepMerge::merge_from(&mut self.time_added, other.time_added);
        crate::DeepMerge::merge_from(&mut self.value, other.value);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceTaint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_effect,
            Key_key,
            Key_time_added,
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
                            "effect" => Field::Key_effect,
                            "key" => Field::Key_key,
                            "timeAdded" => Field::Key_time_added,
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
            type Value = DeviceTaint;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceTaint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_effect: Option<std::string::String> = None;
                let mut value_key: Option<std::string::String> = None;
                let mut value_time_added: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_value: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_effect => value_effect = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time_added => value_time_added = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceTaint {
                    effect: value_effect.unwrap_or_default(),
                    key: value_key.unwrap_or_default(),
                    time_added: value_time_added,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceTaint",
            &[
                "effect",
                "key",
                "timeAdded",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceTaint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceTaint",
            2 +
            self.time_added.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "effect", &self.effect)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.time_added {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeAdded", value)?;
        }
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceTaint {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1.DeviceTaint".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "The device this taint is attached to has the \"effect\" on any claim which does not tolerate the taint and, through the claim, to pods using the claim.",
            "type": "object",
            "properties": {
                "effect": {
                    "description": "The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them. Valid effects are NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here.",
                    "type": "string",
                },
                "key": {
                    "description": "The taint key to be applied to a device. Must be a label name.",
                    "type": "string",
                },
                "timeAdded": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.".into());
                    schema_obj
                }),
                "value": {
                    "description": "The taint value corresponding to the taint key. Must be a label value.",
                    "type": "string",
                },
            },
            "required": [
                "effect",
                "key",
            ],
        })
    }
}
