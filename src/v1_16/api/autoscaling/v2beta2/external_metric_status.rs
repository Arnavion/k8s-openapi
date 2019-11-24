// Generated from definition io.k8s.api.autoscaling.v2beta2.ExternalMetricStatus

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricStatus {
    /// current contains the current value for the given metric
    pub current: crate::api::autoscaling::v2beta2::MetricValueStatus,

    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2beta2::MetricIdentifier,
}

impl<'de> serde::Deserialize<'de> for ExternalMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current,
            Key_metric,
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
                            "current" => Field::Key_current,
                            "metric" => Field::Key_metric,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExternalMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExternalMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_current: Option<crate::api::autoscaling::v2beta2::MetricValueStatus> = None;
                let mut value_metric: Option<crate::api::autoscaling::v2beta2::MetricIdentifier> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current => value_current = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric => value_metric = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricStatus {
                    current: value_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    metric: value_metric.ok_or_else(|| serde::de::Error::missing_field("metric"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricStatus",
            &[
                "current",
                "metric",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ExternalMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricStatus",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "current", &self.current)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        serde::ser::SerializeStruct::end(state)
    }
}
