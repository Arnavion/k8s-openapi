// Generated from definition io.k8s.api.autoscaling.v2beta2.PodsMetricStatus

/// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodsMetricStatus {
    /// current contains the current value for the given metric
    pub current: crate::api::autoscaling::v2beta2::MetricValueStatus,

    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2beta2::MetricIdentifier,
}

impl<'de> crate::serde::Deserialize<'de> for PodsMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current,
            Key_metric,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodsMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodsMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_current: Option<crate::api::autoscaling::v2beta2::MetricValueStatus> = None;
                let mut value_metric: Option<crate::api::autoscaling::v2beta2::MetricIdentifier> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current => value_current = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric => value_metric = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodsMetricStatus {
                    current: value_current.ok_or_else(|| crate::serde::de::Error::missing_field("current"))?,
                    metric: value_metric.ok_or_else(|| crate::serde::de::Error::missing_field("metric"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodsMetricStatus",
            &[
                "current",
                "metric",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodsMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodsMetricStatus",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "current", &self.current)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PodsMetricStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).",
          "properties": {
            "current": crate::schema_ref_with_description(crate::api::autoscaling::v2beta2::MetricValueStatus::schema(), "current contains the current value for the given metric"),
            "metric": crate::schema_ref_with_description(crate::api::autoscaling::v2beta2::MetricIdentifier::schema(), "metric identifies the target metric by name and selector")
          },
          "required": [
            "current",
            "metric"
          ],
          "type": "object"
        })
    }
}
