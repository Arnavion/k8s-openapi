// Generated from definition io.k8s.api.autoscaling.v2beta1.ExternalMetricStatus

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricStatus {
    /// currentAverageValue is the current value of metric averaged over autoscaled pods.
    pub current_average_value: Option<::v1_12::apimachinery::pkg::api::resource::Quantity>,

    /// currentValue is the current value of the metric (as a quantity)
    pub current_value: ::v1_12::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of a metric used for autoscaling in metric system.
    pub metric_name: String,

    /// metricSelector is used to identify a specific time series within a given metric.
    pub metric_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> ::serde::Deserialize<'de> for ExternalMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_average_value,
            Key_current_value,
            Key_metric_name,
            Key_metric_selector,
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
                            "currentAverageValue" => Field::Key_current_average_value,
                            "currentValue" => Field::Key_current_value,
                            "metricName" => Field::Key_metric_name,
                            "metricSelector" => Field::Key_metric_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalMetricStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ExternalMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_current_average_value: Option<::v1_12::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_current_value: Option<::v1_12::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_metric_name: Option<String> = None;
                let mut value_metric_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_average_value => value_current_average_value = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_value => value_current_value = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric_name => value_metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric_selector => value_metric_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricStatus {
                    current_average_value: value_current_average_value,
                    current_value: value_current_value.ok_or_else(|| ::serde::de::Error::missing_field("currentValue"))?,
                    metric_name: value_metric_name.ok_or_else(|| ::serde::de::Error::missing_field("metricName"))?,
                    metric_selector: value_metric_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricStatus",
            &[
                "currentAverageValue",
                "currentValue",
                "metricName",
                "metricSelector",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ExternalMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricStatus",
            0 +
            self.current_average_value.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.metric_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.current_average_value {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageValue", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "currentValue", &self.current_value)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.metric_selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metricSelector", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
