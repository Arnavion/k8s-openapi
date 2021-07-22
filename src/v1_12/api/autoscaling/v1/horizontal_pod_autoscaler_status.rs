// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerStatus

/// current status of a horizontal pod autoscaler
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerStatus {
    /// current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU.
    pub current_cpu_utilization_percentage: Option<i32>,

    /// current number of replicas of pods managed by this autoscaler.
    pub current_replicas: i32,

    /// desired number of replicas of pods managed by this autoscaler.
    pub desired_replicas: i32,

    /// last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed.
    pub last_scale_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// most recent generation observed by this autoscaler.
    pub observed_generation: Option<i64>,
}

impl<'de> crate::serde::Deserialize<'de> for HorizontalPodAutoscalerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_cpu_utilization_percentage,
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
                            "currentCPUUtilizationPercentage" => Field::Key_current_cpu_utilization_percentage,
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
                let mut value_current_cpu_utilization_percentage: Option<i32> = None;
                let mut value_current_replicas: Option<i32> = None;
                let mut value_desired_replicas: Option<i32> = None;
                let mut value_last_scale_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_observed_generation: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_cpu_utilization_percentage => value_current_cpu_utilization_percentage = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_replicas => value_current_replicas = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_desired_replicas => value_desired_replicas = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_last_scale_time => value_last_scale_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerStatus {
                    current_cpu_utilization_percentage: value_current_cpu_utilization_percentage,
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
                "currentCPUUtilizationPercentage",
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
            2 +
            self.current_cpu_utilization_percentage.as_ref().map_or(0, |_| 1) +
            self.last_scale_time.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.current_cpu_utilization_percentage {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentCPUUtilizationPercentage", value)?;
        }
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
          "description": "current status of a horizontal pod autoscaler",
          "properties": {
            "currentCPUUtilizationPercentage": {
              "description": "current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU.",
              "format": "int32",
              "type": "integer"
            },
            "currentReplicas": {
              "description": "current number of replicas of pods managed by this autoscaler.",
              "format": "int32",
              "type": "integer"
            },
            "desiredReplicas": {
              "description": "desired number of replicas of pods managed by this autoscaler.",
              "format": "int32",
              "type": "integer"
            },
            "lastScaleTime": crate::schema_ref_with_values(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), serde_json::json!({"description": "last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed."})),
            "observedGeneration": {
              "description": "most recent generation observed by this autoscaler.",
              "format": "int64",
              "type": "integer"
            }
          },
          "required": [
            "currentReplicas",
            "desiredReplicas"
          ],
          "type": "object"
        })
    }
}
