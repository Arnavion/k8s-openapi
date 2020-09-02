// Generated from definition io.k8s.api.flowcontrol.v1alpha1.PriorityLevelConfigurationStatus

/// PriorityLevelConfigurationStatus represents the current state of a "request-priority".
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriorityLevelConfigurationStatus {
    /// `conditions` is the current state of "request-priority".
    pub conditions: Option<Vec<crate::api::flowcontrol::v1alpha1::PriorityLevelConfigurationCondition>>,
}

impl<'de> serde::Deserialize<'de> for PriorityLevelConfigurationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "conditions" => Field::Key_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PriorityLevelConfigurationStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PriorityLevelConfigurationStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::flowcontrol::v1alpha1::PriorityLevelConfigurationCondition>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityLevelConfigurationStatus {
                    conditions: value_conditions,
                })
            }
        }

        deserializer.deserialize_struct(
            "PriorityLevelConfigurationStatus",
            &[
                "conditions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PriorityLevelConfigurationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PriorityLevelConfigurationStatus",
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
