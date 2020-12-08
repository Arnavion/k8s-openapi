// Generated from definition io.k8s.api.autoscaling.v2beta1.ObjectMetricSource

/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMetricSource {
    /// averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
    pub average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// metricName is the name of the metric in question.
    pub metric_name: String,

    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping When unset, just the metricName will be used to gather metrics.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// target is the described Kubernetes object.
    pub target: crate::api::autoscaling::v2beta1::CrossVersionObjectReference,

    /// targetValue is the target value of the metric (as a quantity).
    pub target_value: crate::apimachinery::pkg::api::resource::Quantity,
}

impl<'de> serde::Deserialize<'de> for ObjectMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_average_value,
            Key_metric_name,
            Key_selector,
            Key_target,
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
                            "averageValue" => Field::Key_average_value,
                            "metricName" => Field::Key_metric_name,
                            "selector" => Field::Key_selector,
                            "target" => Field::Key_target,
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
            type Value = ObjectMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_metric_name: Option<String> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_target: Option<crate::api::autoscaling::v2beta1::CrossVersionObjectReference> = None;
                let mut value_target_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_average_value => value_average_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metric_name => value_metric_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_selector => value_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target => value_target = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target_value => value_target_value = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectMetricSource {
                    average_value: value_average_value,
                    metric_name: value_metric_name.ok_or_else(|| serde::de::Error::missing_field("metricName"))?,
                    selector: value_selector,
                    target: value_target.ok_or_else(|| serde::de::Error::missing_field("target"))?,
                    target_value: value_target_value.ok_or_else(|| serde::de::Error::missing_field("targetValue"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMetricSource",
            &[
                "averageValue",
                "metricName",
                "selector",
                "target",
                "targetValue",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ObjectMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMetricSource",
            3 +
            self.average_value.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.average_value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "averageValue", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "targetValue", &self.target_value)?;
        serde::ser::SerializeStruct::end(state)
    }
}
