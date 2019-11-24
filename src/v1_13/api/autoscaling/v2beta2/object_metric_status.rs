// Generated from definition io.k8s.api.autoscaling.v2beta2.ObjectMetricStatus

/// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMetricStatus {
    /// current contains the current value for the given metric
    pub current: crate::api::autoscaling::v2beta2::MetricValueStatus,

    pub described_object: crate::api::autoscaling::v2beta2::CrossVersionObjectReference,

    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2beta2::MetricIdentifier,
}

impl<'de> serde::Deserialize<'de> for ObjectMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current,
            Key_described_object,
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
                            "describedObject" => Field::Key_described_object,
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
            type Value = ObjectMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_current: Option<crate::api::autoscaling::v2beta2::MetricValueStatus> = None;
                let mut value_described_object: Option<crate::api::autoscaling::v2beta2::CrossVersionObjectReference> = None;
                let mut value_metric: Option<crate::api::autoscaling::v2beta2::MetricIdentifier> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current => value_current = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_described_object => value_described_object = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric => value_metric = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectMetricStatus {
                    current: value_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    described_object: value_described_object.ok_or_else(|| serde::de::Error::missing_field("describedObject"))?,
                    metric: value_metric.ok_or_else(|| serde::de::Error::missing_field("metric"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMetricStatus",
            &[
                "current",
                "describedObject",
                "metric",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ObjectMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMetricStatus",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "current", &self.current)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "describedObject", &self.described_object)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        serde::ser::SerializeStruct::end(state)
    }
}
