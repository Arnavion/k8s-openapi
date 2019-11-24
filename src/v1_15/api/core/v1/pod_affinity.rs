// Generated from definition io.k8s.api.core.v1.PodAffinity

/// Pod affinity is a group of inter pod affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<crate::api::core::v1::WeightedPodAffinityTerm>>,

    /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
    pub required_during_scheduling_ignored_during_execution: Option<Vec<crate::api::core::v1::PodAffinityTerm>>,
}

impl<'de> serde::Deserialize<'de> for PodAffinity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_preferred_during_scheduling_ignored_during_execution,
            Key_required_during_scheduling_ignored_during_execution,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodAffinity;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodAffinity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_preferred_during_scheduling_ignored_during_execution: Option<Vec<crate::api::core::v1::WeightedPodAffinityTerm>> = None;
                let mut value_required_during_scheduling_ignored_during_execution: Option<Vec<crate::api::core::v1::PodAffinityTerm>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_preferred_during_scheduling_ignored_during_execution => value_preferred_during_scheduling_ignored_during_execution = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required_during_scheduling_ignored_during_execution => value_required_during_scheduling_ignored_during_execution = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodAffinity {
                    preferred_during_scheduling_ignored_during_execution: value_preferred_during_scheduling_ignored_during_execution,
                    required_during_scheduling_ignored_during_execution: value_required_during_scheduling_ignored_during_execution,
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

impl serde::Serialize for PodAffinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodAffinity",
            self.preferred_during_scheduling_ignored_during_execution.as_ref().map_or(0, |_| 1) +
            self.required_during_scheduling_ignored_during_execution.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.preferred_during_scheduling_ignored_during_execution {
            serde::ser::SerializeStruct::serialize_field(&mut state, "preferredDuringSchedulingIgnoredDuringExecution", value)?;
        }
        if let Some(value) = &self.required_during_scheduling_ignored_during_execution {
            serde::ser::SerializeStruct::serialize_field(&mut state, "requiredDuringSchedulingIgnoredDuringExecution", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
