// Generated from definition io.k8s.api.autoscaling.v2beta2.ExternalMetricSource

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricSource {
    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2beta2::MetricIdentifier,

    /// target specifies the target value for the given metric
    pub target: crate::api::autoscaling::v2beta2::MetricTarget,
}

impl<'de> serde::Deserialize<'de> for ExternalMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric,
            Key_target,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExternalMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExternalMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metric: Option<crate::api::autoscaling::v2beta2::MetricIdentifier> = None;
                let mut value_target: Option<crate::api::autoscaling::v2beta2::MetricTarget> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric => value_metric = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target => value_target = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricSource {
                    metric: value_metric.ok_or_else(|| serde::de::Error::missing_field("metric"))?,
                    target: value_target.ok_or_else(|| serde::de::Error::missing_field("target"))?,
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

impl serde::Serialize for ExternalMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricSource",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        serde::ser::SerializeStruct::end(state)
    }
}
