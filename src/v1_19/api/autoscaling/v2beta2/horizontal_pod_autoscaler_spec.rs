// Generated from definition io.k8s.api.autoscaling.v2beta2.HorizontalPodAutoscalerSpec

/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerSpec {
    /// behavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively). If not set, the default HPAScalingRules for scale up and scale down are used.
    pub behavior: Option<crate::api::autoscaling::v2beta2::HorizontalPodAutoscalerBehavior>,

    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
    pub max_replicas: i32,

    /// metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond. If not set, the default metric will be set to 80% average CPU utilization.
    pub metrics: Option<Vec<crate::api::autoscaling::v2beta2::MetricSpec>>,

    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    pub min_replicas: Option<i32>,

    /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
    pub scale_target_ref: crate::api::autoscaling::v2beta2::CrossVersionObjectReference,
}

impl<'de> serde::Deserialize<'de> for HorizontalPodAutoscalerSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_behavior,
            Key_max_replicas,
            Key_metrics,
            Key_min_replicas,
            Key_scale_target_ref,
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
                            "behavior" => Field::Key_behavior,
                            "maxReplicas" => Field::Key_max_replicas,
                            "metrics" => Field::Key_metrics,
                            "minReplicas" => Field::Key_min_replicas,
                            "scaleTargetRef" => Field::Key_scale_target_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HorizontalPodAutoscalerSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_behavior: Option<crate::api::autoscaling::v2beta2::HorizontalPodAutoscalerBehavior> = None;
                let mut value_max_replicas: Option<i32> = None;
                let mut value_metrics: Option<Vec<crate::api::autoscaling::v2beta2::MetricSpec>> = None;
                let mut value_min_replicas: Option<i32> = None;
                let mut value_scale_target_ref: Option<crate::api::autoscaling::v2beta2::CrossVersionObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_behavior => value_behavior = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_replicas => value_max_replicas = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metrics => value_metrics = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_replicas => value_min_replicas = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_target_ref => value_scale_target_ref = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerSpec {
                    behavior: value_behavior,
                    max_replicas: value_max_replicas.ok_or_else(|| serde::de::Error::missing_field("maxReplicas"))?,
                    metrics: value_metrics,
                    min_replicas: value_min_replicas,
                    scale_target_ref: value_scale_target_ref.ok_or_else(|| serde::de::Error::missing_field("scaleTargetRef"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerSpec",
            &[
                "behavior",
                "maxReplicas",
                "metrics",
                "minReplicas",
                "scaleTargetRef",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for HorizontalPodAutoscalerSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerSpec",
            2 +
            self.behavior.as_ref().map_or(0, |_| 1) +
            self.metrics.as_ref().map_or(0, |_| 1) +
            self.min_replicas.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.behavior {
            serde::ser::SerializeStruct::serialize_field(&mut state, "behavior", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "maxReplicas", &self.max_replicas)?;
        if let Some(value) = &self.metrics {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metrics", value)?;
        }
        if let Some(value) = &self.min_replicas {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minReplicas", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "scaleTargetRef", &self.scale_target_ref)?;
        serde::ser::SerializeStruct::end(state)
    }
}
