// Generated from definition io.k8s.api.core.v1.SleepAction

/// SleepAction describes a "sleep" action.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SleepAction {
    /// Seconds is the number of seconds to sleep.
    pub seconds: i64,
}

impl crate::DeepMerge for SleepAction {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.seconds, other.seconds);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SleepAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_seconds,
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
                            "seconds" => Field::Key_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SleepAction;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SleepAction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_seconds: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_seconds => value_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SleepAction {
                    seconds: value_seconds.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "SleepAction",
            &[
                "seconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SleepAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SleepAction",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "seconds", &self.seconds)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SleepAction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.SleepAction".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "SleepAction describes a \"sleep\" action.",
            "type": "object",
            "properties": {
                "seconds": {
                    "description": "Seconds is the number of seconds to sleep.",
                    "type": "integer",
                    "format": "int64",
                },
            },
            "required": [
                "seconds",
            ],
        })
    }
}
