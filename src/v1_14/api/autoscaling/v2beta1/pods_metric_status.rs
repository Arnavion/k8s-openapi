// Generated from definition io.k8s.api.autoscaling.v2beta1.PodsMetricStatus

/// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodsMetricStatus {
    /// currentAverageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    pub current_average_value: crate::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of the metric in question
    pub metric_name: String,

    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric When set in the PodsMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> serde::Deserialize<'de> for PodsMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_average_value,
            Key_metric_name,
            Key_selector,
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
                            "currentAverageValue" => Field::Key_current_average_value,
                            "metricName" => Field::Key_metric_name,
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodsMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodsMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_current_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_metric_name: Option<String> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_average_value => value_current_average_value = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric_name => value_metric_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_selector => value_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodsMetricStatus {
                    current_average_value: value_current_average_value.ok_or_else(|| serde::de::Error::missing_field("currentAverageValue"))?,
                    metric_name: value_metric_name.ok_or_else(|| serde::de::Error::missing_field("metricName"))?,
                    selector: value_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodsMetricStatus",
            &[
                "currentAverageValue",
                "metricName",
                "selector",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodsMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodsMetricStatus",
            2 +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageValue", &self.current_average_value)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
