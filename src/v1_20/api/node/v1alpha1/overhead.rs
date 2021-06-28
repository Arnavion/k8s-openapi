// Generated from definition io.k8s.api.node.v1alpha1.Overhead

/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    pub pod_fixed: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> crate::serde::Deserialize<'de> for Overhead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_fixed,
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
                            "podFixed" => Field::Key_pod_fixed,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Overhead;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Overhead")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_pod_fixed: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_fixed => value_pod_fixed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Overhead {
                    pod_fixed: value_pod_fixed.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Overhead",
            &[
                "podFixed",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Overhead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Overhead",
            usize::from(!self.pod_fixed.is_empty()),
        )?;
        if !self.pod_fixed.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podFixed", &self.pod_fixed)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Overhead {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Overhead structure represents the resource overhead associated with running a pod.",
          "properties": {
            "podFixed": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "PodFixed represents the fixed resource overhead associated with running a pod.",
              "type": "object"
            }
          },
          "type": "object"
        })
    }
}
