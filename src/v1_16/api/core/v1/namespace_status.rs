// Generated from definition io.k8s.api.core.v1.NamespaceStatus

/// NamespaceStatus is information about the current status of a Namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamespaceStatus {
    /// Represents the latest available observations of a namespace's current state.
    pub conditions: Option<Vec<crate::api::core::v1::NamespaceCondition>>,

    /// Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    pub phase: Option<String>,
}

impl<'de> serde::Deserialize<'de> for NamespaceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_phase,
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
                            "phase" => Field::Key_phase,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NamespaceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::core::v1::NamespaceCondition>> = None;
                let mut value_phase: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamespaceStatus {
                    conditions: value_conditions,
                    phase: value_phase,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamespaceStatus",
            &[
                "conditions",
                "phase",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for NamespaceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamespaceStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.phase {
            serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
