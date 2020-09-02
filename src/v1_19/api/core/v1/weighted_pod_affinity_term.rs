// Generated from definition io.k8s.api.core.v1.WeightedPodAffinityTerm

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeightedPodAffinityTerm {
    /// Required. A pod affinity term, associated with the corresponding weight.
    pub pod_affinity_term: crate::api::core::v1::PodAffinityTerm,

    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    pub weight: i32,
}

impl<'de> serde::Deserialize<'de> for WeightedPodAffinityTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_affinity_term,
            Key_weight,
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
                            "podAffinityTerm" => Field::Key_pod_affinity_term,
                            "weight" => Field::Key_weight,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WeightedPodAffinityTerm;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WeightedPodAffinityTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_pod_affinity_term: Option<crate::api::core::v1::PodAffinityTerm> = None;
                let mut value_weight: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_affinity_term => value_pod_affinity_term = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_weight => value_weight = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WeightedPodAffinityTerm {
                    pod_affinity_term: value_pod_affinity_term.ok_or_else(|| serde::de::Error::missing_field("podAffinityTerm"))?,
                    weight: value_weight.ok_or_else(|| serde::de::Error::missing_field("weight"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "WeightedPodAffinityTerm",
            &[
                "podAffinityTerm",
                "weight",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for WeightedPodAffinityTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WeightedPodAffinityTerm",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "podAffinityTerm", &self.pod_affinity_term)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "weight", &self.weight)?;
        serde::ser::SerializeStruct::end(state)
    }
}
