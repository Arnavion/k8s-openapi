// Generated from definition io.k8s.api.node.v1alpha1.Overhead

/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    pub pod_fixed: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,
}

impl<'de> serde::Deserialize<'de> for Overhead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_fixed,
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
                            "podFixed" => Field::Key_pod_fixed,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Overhead;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Overhead")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_pod_fixed: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_fixed => value_pod_fixed = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Overhead {
                    pod_fixed: value_pod_fixed,
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

impl serde::Serialize for Overhead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Overhead",
            self.pod_fixed.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.pod_fixed {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podFixed", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
