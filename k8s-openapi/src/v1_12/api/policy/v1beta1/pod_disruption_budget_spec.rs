// Generated from definition io.k8s.api.policy.v1beta1.PodDisruptionBudgetSpec

/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at most "maxUnavailable" pods selected by "selector" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with "minAvailable".
    pub max_unavailable: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString>,

    /// An eviction is allowed if at least "minAvailable" pods selected by "selector" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying "100%".
    pub min_available: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString>,

    /// Label query over pods whose evictions are managed by the disruption budget.
    pub selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> ::serde::Deserialize<'de> for PodDisruptionBudgetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_unavailable,
            Key_min_available,
            Key_selector,
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
                            "maxUnavailable" => Field::Key_max_unavailable,
                            "minAvailable" => Field::Key_min_available,
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodDisruptionBudgetSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodDisruptionBudgetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_max_unavailable: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_min_available: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_unavailable => value_max_unavailable = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_available => value_min_available = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodDisruptionBudgetSpec {
                    max_unavailable: value_max_unavailable,
                    min_available: value_min_available,
                    selector: value_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDisruptionBudgetSpec",
            &[
                "maxUnavailable",
                "minAvailable",
                "selector",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodDisruptionBudgetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDisruptionBudgetSpec",
            0 +
            self.max_unavailable.as_ref().map_or(0, |_| 1) +
            self.min_available.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max_unavailable {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "maxUnavailable", value)?;
        }
        if let Some(value) = &self.min_available {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "minAvailable", value)?;
        }
        if let Some(value) = &self.selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
