// Generated from definition io.k8s.api.core.v1.PodReadinessGate

/// PodReadinessGate contains the reference to a pod condition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodReadinessGate {
    /// ConditionType refers to a condition in the pod's condition list with matching type.
    pub condition_type: std::string::String,
}

impl crate::DeepMerge for PodReadinessGate {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.condition_type, other.condition_type);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodReadinessGate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_condition_type,
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
                            "conditionType" => Field::Key_condition_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodReadinessGate;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodReadinessGate")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_condition_type: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_condition_type => value_condition_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodReadinessGate {
                    condition_type: value_condition_type.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodReadinessGate",
            &[
                "conditionType",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodReadinessGate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodReadinessGate",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditionType", &self.condition_type)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodReadinessGate {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.PodReadinessGate".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodReadinessGate contains the reference to a pod condition",
            "type": "object",
            "properties": {
                "conditionType": {
                    "description": "ConditionType refers to a condition in the pod's condition list with matching type.",
                    "type": "string",
                },
            },
            "required": [
                "conditionType",
            ],
        })
    }
}
