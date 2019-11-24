// Generated from definition io.k8s.api.autoscaling.v2beta1.ExternalMetricSource

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). Exactly one "target" type should be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricSource {
    /// metricName is the name of the metric in question.
    pub metric_name: String,

    /// metricSelector is used to identify a specific time series within a given metric.
    pub metric_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue.
    pub target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue.
    pub target_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> serde::Deserialize<'de> for ExternalMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric_name,
            Key_metric_selector,
            Key_target_average_value,
            Key_target_value,
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
                            "metricName" => Field::Key_metric_name,
                            "metricSelector" => Field::Key_metric_selector,
                            "targetAverageValue" => Field::Key_target_average_value,
                            "targetValue" => Field::Key_target_value,
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
                let mut value_metric_name: Option<String> = None;
                let mut value_metric_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_target_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric_name => value_metric_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric_selector => value_metric_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_average_value => value_target_average_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_value => value_target_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricSource {
                    metric_name: value_metric_name.ok_or_else(|| serde::de::Error::missing_field("metricName"))?,
                    metric_selector: value_metric_selector,
                    target_average_value: value_target_average_value,
                    target_value: value_target_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricSource",
            &[
                "metricName",
                "metricSelector",
                "targetAverageValue",
                "targetValue",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ExternalMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricSource",
            1 +
            self.metric_selector.as_ref().map_or(0, |_| 1) +
            self.target_average_value.as_ref().map_or(0, |_| 1) +
            self.target_value.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.metric_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metricSelector", value)?;
        }
        if let Some(value) = &self.target_average_value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "targetAverageValue", value)?;
        }
        if let Some(value) = &self.target_value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "targetValue", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
