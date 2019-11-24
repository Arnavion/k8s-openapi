// Generated from definition io.k8s.api.node.v1alpha1.Scheduling

/// Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scheduling {
    /// nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.
    pub node_selector: Option<std::collections::BTreeMap<String, String>>,

    /// tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.
    pub tolerations: Option<Vec<crate::api::core::v1::Toleration>>,
}

impl<'de> serde::Deserialize<'de> for Scheduling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_selector,
            Key_tolerations,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Scheduling;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Scheduling")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_tolerations: Option<Vec<crate::api::core::v1::Toleration>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_selector => value_node_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scheduling {
                    node_selector: value_node_selector,
                    tolerations: value_tolerations,
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

impl serde::Serialize for Scheduling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scheduling",
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.node_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.tolerations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
