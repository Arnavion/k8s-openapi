// Generated from definition io.k8s.api.autoscaling.v2beta2.ExternalMetricSource

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricSource {
    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2beta2::MetricIdentifier,

    /// target specifies the target value for the given metric
    pub target: crate::api::autoscaling::v2beta2::MetricTarget,
}

impl<'de> crate::serde::Deserialize<'de> for ExternalMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric,
            Key_target,
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
                            "metric" => Field::Key_metric,
                            "target" => Field::Key_target,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExternalMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metric: Option<crate::api::autoscaling::v2beta2::MetricIdentifier> = None;
                let mut value_target: Option<crate::api::autoscaling::v2beta2::MetricTarget> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric => value_metric = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target => value_target = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricSource {
                    metric: value_metric.ok_or_else(|| crate::serde::de::Error::missing_field("metric"))?,
                    target: value_target.ok_or_else(|| crate::serde::de::Error::missing_field("target"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricSource",
            &[
                "metric",
                "target",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExternalMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricSource",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ExternalMetricSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).",
          "properties": {
            "metric": crate::schema_ref_with_description(crate::api::autoscaling::v2beta2::MetricIdentifier::schema(), "metric identifies the target metric by name and selector"),
            "target": crate::schema_ref_with_description(crate::api::autoscaling::v2beta2::MetricTarget::schema(), "target specifies the target value for the given metric")
          },
          "required": [
            "metric",
            "target"
          ],
          "type": "object"
        })
    }
}
