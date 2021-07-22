// Generated from definition io.k8s.api.core.v1.NamespaceStatus

/// NamespaceStatus is information about the current status of a Namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamespaceStatus {
    /// Represents the latest available observations of a namespace's current state.
    pub conditions: Vec<crate::api::core::v1::NamespaceCondition>,

    /// Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    pub phase: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for NamespaceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_phase,
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
                            "phase" => Field::Key_phase,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NamespaceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::core::v1::NamespaceCondition>> = None;
                let mut value_phase: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamespaceStatus {
                    conditions: value_conditions.unwrap_or_default(),
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

impl crate::serde::Serialize for NamespaceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamespaceStatus",
            usize::from(!self.conditions.is_empty()) +
            self.phase.as_ref().map_or(0, |_| 1),
        )?;
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for NamespaceStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "NamespaceStatus is information about the current status of a Namespace.",
          "properties": {
            "conditions": {
              "description": "Represents the latest available observations of a namespace's current state.",
              "items": crate::api::core::v1::NamespaceCondition::schema(),
              "x-kubernetes-patch-merge-key": "type",
              "x-kubernetes-patch-strategy": "merge",
              "type": "array"
            },
            "phase": {
              "description": "Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
