// Generated from definition io.k8s.api.autoscaling.v2beta2.ObjectMetricSource

/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMetricSource {
    pub described_object: crate::api::autoscaling::v2beta2::CrossVersionObjectReference,

    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2beta2::MetricIdentifier,

    /// target specifies the target value for the given metric
    pub target: crate::api::autoscaling::v2beta2::MetricTarget,
}

impl<'de> serde::Deserialize<'de> for ObjectMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_described_object,
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
                            "describedObject" => Field::Key_described_object,
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
            type Value = ObjectMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_described_object: Option<crate::api::autoscaling::v2beta2::CrossVersionObjectReference> = None;
                let mut value_metric: Option<crate::api::autoscaling::v2beta2::MetricIdentifier> = None;
                let mut value_target: Option<crate::api::autoscaling::v2beta2::MetricTarget> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_described_object => value_described_object = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric => value_metric = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target => value_target = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectMetricSource {
                    described_object: value_described_object.ok_or_else(|| serde::de::Error::missing_field("describedObject"))?,
                    metric: value_metric.ok_or_else(|| serde::de::Error::missing_field("metric"))?,
                    target: value_target.ok_or_else(|| serde::de::Error::missing_field("target"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMetricSource",
            &[
                "describedObject",
                "metric",
                "target",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ObjectMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMetricSource",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "describedObject", &self.described_object)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        serde::ser::SerializeStruct::end(state)
    }
}
