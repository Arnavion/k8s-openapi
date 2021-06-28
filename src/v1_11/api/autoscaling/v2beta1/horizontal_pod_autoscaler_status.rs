// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerStatus

/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerStatus {
    /// conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
    pub conditions: Vec<crate::api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition>,

    /// currentMetrics is the last read state of the metrics used by this autoscaler.
    pub current_metrics: Vec<crate::api::autoscaling::v2beta1::MetricStatus>,

    /// currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
    pub current_replicas: i32,

    /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
    pub desired_replicas: i32,

    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
    pub last_scale_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// observedGeneration is the most recent generation observed by this autoscaler.
    pub observed_generation: Option<i64>,
}

impl<'de> crate::serde::Deserialize<'de> for HorizontalPodAutoscalerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_current_metrics,
            Key_current_replicas,
            Key_desired_replicas,
            Key_last_scale_time,
            Key_observed_generation,
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
                            "conditions" => Field::Key_conditions,
                            "currentMetrics" => Field::Key_current_metrics,
                            "currentReplicas" => Field::Key_current_replicas,
                            "desiredReplicas" => Field::Key_desired_replicas,
                            "lastScaleTime" => Field::Key_last_scale_time,
                            "observedGeneration" => Field::Key_observed_generation,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HorizontalPodAutoscalerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition>> = None;
                let mut value_current_metrics: Option<Vec<crate::api::autoscaling::v2beta1::MetricStatus>> = None;
                let mut value_current_replicas: Option<i32> = None;
                let mut value_desired_replicas: Option<i32> = None;
                let mut value_last_scale_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_observed_generation: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_current_metrics => value_current_metrics = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_current_replicas => value_current_replicas = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_desired_replicas => value_desired_replicas = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_last_scale_time => value_last_scale_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerStatus {
                    conditions: value_conditions.ok_or_else(|| crate::serde::de::Error::missing_field("conditions"))?,
                    current_metrics: value_current_metrics.ok_or_else(|| crate::serde::de::Error::missing_field("currentMetrics"))?,
                    current_replicas: value_current_replicas.ok_or_else(|| crate::serde::de::Error::missing_field("currentReplicas"))?,
                    desired_replicas: value_desired_replicas.ok_or_else(|| crate::serde::de::Error::missing_field("desiredReplicas"))?,
                    last_scale_time: value_last_scale_time,
                    observed_generation: value_observed_generation,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerStatus",
            &[
                "conditions",
                "currentMetrics",
                "currentReplicas",
                "desiredReplicas",
                "lastScaleTime",
                "observedGeneration",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HorizontalPodAutoscalerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerStatus",
            4 +
            self.last_scale_time.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentMetrics", &self.current_metrics)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentReplicas", &self.current_replicas)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "desiredReplicas", &self.desired_replicas)?;
        if let Some(value) = &self.last_scale_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastScaleTime", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for HorizontalPodAutoscalerStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.",
          "properties": {
            "conditions": {
              "description": "conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.",
              "items": crate::api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition::schema(),
              "type": "array"
            },
            "currentMetrics": {
              "description": "currentMetrics is the last read state of the metrics used by this autoscaler.",
              "items": crate::api::autoscaling::v2beta1::MetricStatus::schema(),
              "type": "array"
            },
            "currentReplicas": {
              "description": "currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.",
              "format": "int32",
              "type": "integer"
            },
            "desiredReplicas": {
              "description": "desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.",
              "format": "int32",
              "type": "integer"
            },
            "lastScaleTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed."),
            "observedGeneration": {
              "description": "observedGeneration is the most recent generation observed by this autoscaler.",
              "format": "int64",
              "type": "integer"
            }
          },
          "required": [
            "conditions",
            "currentMetrics",
            "currentReplicas",
            "desiredReplicas"
          ],
          "type": "object"
        })
    }
}
