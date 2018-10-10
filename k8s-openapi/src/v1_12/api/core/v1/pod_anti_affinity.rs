// Generated from definition io.k8s.api.core.v1.PodAntiAffinity

/// Pod anti affinity is a group of inter pod anti affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodAntiAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<::v1_12::api::core::v1::WeightedPodAffinityTerm>>,

    /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
    pub required_during_scheduling_ignored_during_execution: Option<Vec<::v1_12::api::core::v1::PodAffinityTerm>>,
}

impl<'de> ::serde::Deserialize<'de> for PodAntiAffinity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_preferred_during_scheduling_ignored_during_execution,
            Key_required_during_scheduling_ignored_during_execution,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodAntiAffinity;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodAntiAffinity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_preferred_during_scheduling_ignored_during_execution: Option<Vec<::v1_12::api::core::v1::WeightedPodAffinityTerm>> = None;
                let mut value_required_during_scheduling_ignored_during_execution: Option<Vec<::v1_12::api::core::v1::PodAffinityTerm>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_preferred_during_scheduling_ignored_during_execution => value_preferred_during_scheduling_ignored_during_execution = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required_during_scheduling_ignored_during_execution => value_required_during_scheduling_ignored_during_execution = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodAntiAffinity {
                    preferred_during_scheduling_ignored_during_execution: value_preferred_during_scheduling_ignored_during_execution,
                    required_during_scheduling_ignored_during_execution: value_required_during_scheduling_ignored_during_execution,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodAntiAffinity",
            &[
                "preferredDuringSchedulingIgnoredDuringExecution",
                "requiredDuringSchedulingIgnoredDuringExecution",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodAntiAffinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodAntiAffinity",
            0 +
            self.preferred_during_scheduling_ignored_during_execution.as_ref().map_or(0, |_| 1) +
            self.required_during_scheduling_ignored_during_execution.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.preferred_during_scheduling_ignored_during_execution {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "preferredDuringSchedulingIgnoredDuringExecution", value)?;
        }
        if let Some(value) = &self.required_during_scheduling_ignored_during_execution {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "requiredDuringSchedulingIgnoredDuringExecution", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
