// Generated from definition io.k8s.api.auditregistration.v1alpha1.Policy

/// Policy defines the configuration of how audit events are logged
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Policy {
    /// The Level that all requests are recorded at. available options: None, Metadata, Request, RequestResponse required
    pub level: String,

    /// Stages is a list of stages for which events are created.
    pub stages: Option<Vec<String>>,
}

impl<'de> crate::serde::Deserialize<'de> for Policy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_level,
            Key_stages,
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
                            "level" => Field::Key_level,
                            "stages" => Field::Key_stages,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Policy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Policy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_level: Option<String> = None;
                let mut value_stages: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_level => value_level = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_stages => value_stages = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Policy {
                    level: value_level.ok_or_else(|| crate::serde::de::Error::missing_field("level"))?,
                    stages: value_stages,
                })
            }
        }

        deserializer.deserialize_struct(
            "Policy",
            &[
                "level",
                "stages",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Policy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Policy",
            1 +
            self.stages.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "level", &self.level)?;
        if let Some(value) = &self.stages {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stages", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
