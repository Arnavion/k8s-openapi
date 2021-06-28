// Generated from definition io.k8s.api.node.v1.Scheduling

/// Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scheduling {
    /// nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.
    pub node_selector: std::collections::BTreeMap<String, String>,

    /// tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.
    pub tolerations: Vec<crate::api::core::v1::Toleration>,
}

impl<'de> crate::serde::Deserialize<'de> for Scheduling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_selector,
            Key_tolerations,
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
                            "nodeSelector" => Field::Key_node_selector,
                            "tolerations" => Field::Key_tolerations,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Scheduling;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Scheduling")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_tolerations: Option<Vec<crate::api::core::v1::Toleration>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scheduling {
                    node_selector: value_node_selector.unwrap_or_default(),
                    tolerations: value_tolerations.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Scheduling",
            &[
                "nodeSelector",
                "tolerations",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Scheduling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scheduling",
            usize::from(!self.node_selector.is_empty()) +
            usize::from(!self.tolerations.is_empty()),
        )?;
        if !self.node_selector.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", &self.node_selector)?;
        }
        if !self.tolerations.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", &self.tolerations)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Scheduling {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.",
          "properties": {
            "nodeSelector": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.",
              "type": "object"
            },
            "tolerations": {
              "description": "tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.",
              "items": crate::api::core::v1::Toleration::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
