// Generated from definition io.k8s.api.core.v1.ContainerStateRunning

/// ContainerStateRunning is a running state of a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStateRunning {
    /// Time at which the container was last (re-)started
    pub started_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> crate::serde::Deserialize<'de> for ContainerStateRunning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_started_at,
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
                            "startedAt" => Field::Key_started_at,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStateRunning;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerStateRunning")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_started_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_started_at => value_started_at = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStateRunning {
                    started_at: value_started_at,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStateRunning",
            &[
                "startedAt",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerStateRunning {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStateRunning",
            self.started_at.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.started_at {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startedAt", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ContainerStateRunning {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ContainerStateRunning is a running state of a container.",
          "properties": {
            "startedAt": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "Time at which the container was last (re-)started")
          },
          "type": "object"
        })
    }
}
