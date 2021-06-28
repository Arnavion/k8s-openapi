// Generated from definition io.k8s.api.core.v1.PodAffinity

/// Pod affinity is a group of inter pod affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
    pub preferred_during_scheduling_ignored_during_execution: Vec<crate::api::core::v1::WeightedPodAffinityTerm>,

    /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
    pub required_during_scheduling_ignored_during_execution: Vec<crate::api::core::v1::PodAffinityTerm>,
}

impl<'de> crate::serde::Deserialize<'de> for PodAffinity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_preferred_during_scheduling_ignored_during_execution,
            Key_required_during_scheduling_ignored_during_execution,
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
                            "preferredDuringSchedulingIgnoredDuringExecution" => Field::Key_preferred_during_scheduling_ignored_during_execution,
                            "requiredDuringSchedulingIgnoredDuringExecution" => Field::Key_required_during_scheduling_ignored_during_execution,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodAffinity;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodAffinity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_preferred_during_scheduling_ignored_during_execution: Option<Vec<crate::api::core::v1::WeightedPodAffinityTerm>> = None;
                let mut value_required_during_scheduling_ignored_during_execution: Option<Vec<crate::api::core::v1::PodAffinityTerm>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_preferred_during_scheduling_ignored_during_execution => value_preferred_during_scheduling_ignored_during_execution = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required_during_scheduling_ignored_during_execution => value_required_during_scheduling_ignored_during_execution = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodAffinity {
                    preferred_during_scheduling_ignored_during_execution: value_preferred_during_scheduling_ignored_during_execution.unwrap_or_default(),
                    required_during_scheduling_ignored_during_execution: value_required_during_scheduling_ignored_during_execution.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodAffinity",
            &[
                "preferredDuringSchedulingIgnoredDuringExecution",
                "requiredDuringSchedulingIgnoredDuringExecution",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodAffinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodAffinity",
            usize::from(!self.preferred_during_scheduling_ignored_during_execution.is_empty()) +
            usize::from(!self.required_during_scheduling_ignored_during_execution.is_empty()),
        )?;
        if !self.preferred_during_scheduling_ignored_during_execution.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preferredDuringSchedulingIgnoredDuringExecution", &self.preferred_during_scheduling_ignored_during_execution)?;
        }
        if !self.required_during_scheduling_ignored_during_execution.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requiredDuringSchedulingIgnoredDuringExecution", &self.required_during_scheduling_ignored_during_execution)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PodAffinity {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Pod affinity is a group of inter pod affinity scheduling rules.",
          "properties": {
            "preferredDuringSchedulingIgnoredDuringExecution": {
              "description": "The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding \"weight\" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.",
              "items": crate::api::core::v1::WeightedPodAffinityTerm::schema(),
              "type": "array"
            },
            "requiredDuringSchedulingIgnoredDuringExecution": {
              "description": "If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.",
              "items": crate::api::core::v1::PodAffinityTerm::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
