// Generated from definition io.k8s.api.extensions.v1beta1.RollbackConfig

/// DEPRECATED.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollbackConfig {
    /// The revision to rollback to. If set to 0, rollback to the last revision.
    pub revision: Option<i64>,
}

impl<'de> crate::serde::Deserialize<'de> for RollbackConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_revision,
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
                            "revision" => Field::Key_revision,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RollbackConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RollbackConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_revision: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_revision => value_revision = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RollbackConfig {
                    revision: value_revision,
                })
            }
        }

        deserializer.deserialize_struct(
            "RollbackConfig",
            &[
                "revision",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RollbackConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollbackConfig",
            self.revision.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.revision {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "revision", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
