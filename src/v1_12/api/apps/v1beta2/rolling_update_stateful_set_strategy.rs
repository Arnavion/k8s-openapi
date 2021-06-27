// Generated from definition io.k8s.api.apps.v1beta2.RollingUpdateStatefulSetStrategy

/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be partitioned. Default value is 0.
    pub partition: Option<i32>,
}

impl<'de> crate::serde::Deserialize<'de> for RollingUpdateStatefulSetStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_partition,
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
                            "partition" => Field::Key_partition,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RollingUpdateStatefulSetStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RollingUpdateStatefulSetStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_partition: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_partition => value_partition = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RollingUpdateStatefulSetStrategy {
                    partition: value_partition,
                })
            }
        }

        deserializer.deserialize_struct(
            "RollingUpdateStatefulSetStrategy",
            &[
                "partition",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RollingUpdateStatefulSetStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollingUpdateStatefulSetStrategy",
            self.partition.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.partition {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "partition", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl RollingUpdateStatefulSetStrategy {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.",
          "properties": {
            "partition": {
              "description": "Partition indicates the ordinal at which the StatefulSet should be partitioned. Default value is 0.",
              "format": "int32",
              "type": "integer"
            }
          },
          "type": "object"
        })
    }
}
