// Generated from definition io.k8s.api.autoscaling.v2beta1.ObjectMetricSource

/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMetricSource {
    /// metricName is the name of the metric in question.
    pub metric_name: String,

    /// target is the described Kubernetes object.
    pub target: crate::api::autoscaling::v2beta1::CrossVersionObjectReference,

    /// targetValue is the target value of the metric (as a quantity).
    pub target_value: crate::apimachinery::pkg::api::resource::Quantity,
}

impl<'de> crate::serde::Deserialize<'de> for ObjectMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric_name,
            Key_target,
            Key_target_value,
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
                            "metricName" => Field::Key_metric_name,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metric_name: Option<String> = None;
                let mut value_target: Option<crate::api::autoscaling::v2beta1::CrossVersionObjectReference> = None;
                let mut value_target_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric_name => value_metric_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target => value_target = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target_value => value_target_value = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectMetricSource {
                    metric_name: value_metric_name.ok_or_else(|| crate::serde::de::Error::missing_field("metricName"))?,
                    target: value_target.ok_or_else(|| crate::serde::de::Error::missing_field("target"))?,
                    target_value: value_target_value.ok_or_else(|| crate::serde::de::Error::missing_field("targetValue"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMetricSource",
            &[
                "metricName",
                "target",
                "targetValue",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ObjectMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMetricSource",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetValue", &self.target_value)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}
