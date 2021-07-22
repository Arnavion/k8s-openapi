// Generated from definition io.k8s.api.flowcontrol.v1beta1.FlowSchemaStatus

/// FlowSchemaStatus represents the current state of a FlowSchema.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowSchemaStatus {
    /// `conditions` is a list of the current states of FlowSchema.
    pub conditions: Vec<crate::api::flowcontrol::v1beta1::FlowSchemaCondition>,
}

impl<'de> crate::serde::Deserialize<'de> for FlowSchemaStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlowSchemaStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlowSchemaStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::flowcontrol::v1beta1::FlowSchemaCondition>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlowSchemaStatus {
                    conditions: value_conditions.unwrap_or_default(),
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

impl crate::serde::Serialize for FlowSchemaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlowSchemaStatus",
            usize::from(!self.conditions.is_empty()),
        )?;
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for FlowSchemaStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "FlowSchemaStatus represents the current state of a FlowSchema.",
          "properties": {
            "conditions": {
              "description": "`conditions` is a list of the current states of FlowSchema.",
              "items": crate::api::flowcontrol::v1beta1::FlowSchemaCondition::schema(),
              "x-kubernetes-list-map-keys": [
                "type"
              ],
              "x-kubernetes-list-type": "map",
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
