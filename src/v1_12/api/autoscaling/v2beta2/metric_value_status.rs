// Generated from definition io.k8s.api.autoscaling.v2beta2.MetricValueStatus

/// MetricValueStatus holds the current value for a metric
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetricValueStatus {
    /// currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
    pub average_utilization: Option<i32>,

    /// averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    pub average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// value is the current value of the metric (as a quantity).
    pub value: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> serde::Deserialize<'de> for MetricValueStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_average_utilization,
            Key_average_value,
            Key_value,
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
                            "averageUtilization" => Field::Key_average_utilization,
                            "averageValue" => Field::Key_average_value,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MetricValueStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MetricValueStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_average_utilization: Option<i32> = None;
                let mut value_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_average_utilization => value_average_utilization = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_average_value => value_average_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MetricValueStatus {
                    average_utilization: value_average_utilization,
                    average_value: value_average_value,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "MetricValueStatus",
            &[
                "averageUtilization",
                "averageValue",
                "value",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for MetricValueStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MetricValueStatus",
            self.average_utilization.as_ref().map_or(0, |_| 1) +
            self.average_value.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.average_utilization {
            serde::ser::SerializeStruct::serialize_field(&mut state, "averageUtilization", value)?;
        }
        if let Some(value) = &self.average_value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "averageValue", value)?;
        }
        if let Some(value) = &self.value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
