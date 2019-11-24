// Generated from definition io.k8s.api.apps.v1.RollingUpdateStatefulSetStrategy

/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be partitioned. Default value is 0.
    pub partition: Option<i32>,
}

impl<'de> serde::Deserialize<'de> for RollingUpdateStatefulSetStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_partition,
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
                            "partition" => Field::Key_partition,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RollingUpdateStatefulSetStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RollingUpdateStatefulSetStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_partition: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_partition => value_partition = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for RollingUpdateStatefulSetStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollingUpdateStatefulSetStrategy",
            self.partition.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.partition {
            serde::ser::SerializeStruct::serialize_field(&mut state, "partition", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
