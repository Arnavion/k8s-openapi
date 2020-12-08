// Generated from definition io.k8s.api.flowcontrol.v1beta1.FlowSchemaStatus

/// FlowSchemaStatus represents the current state of a FlowSchema.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowSchemaStatus {
    /// `conditions` is a list of the current states of FlowSchema.
    pub conditions: Option<Vec<crate::api::flowcontrol::v1beta1::FlowSchemaCondition>>,
}

impl<'de> serde::Deserialize<'de> for FlowSchemaStatus {
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
            type Value = FlowSchemaStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlowSchemaStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::flowcontrol::v1beta1::FlowSchemaCondition>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlowSchemaStatus {
                    conditions: value_conditions,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlowSchemaStatus",
            &[
                "conditions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for FlowSchemaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlowSchemaStatus",
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
