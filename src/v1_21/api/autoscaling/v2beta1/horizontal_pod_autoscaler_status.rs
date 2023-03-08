// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerStatus

/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerStatus {
    /// conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
    pub conditions: Vec<crate::api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition>,

    /// currentMetrics is the last read state of the metrics used by this autoscaler.
    pub current_metrics: Option<Vec<crate::api::autoscaling::v2beta1::MetricStatus>>,

    /// currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
    pub current_replicas: i32,

    /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
    pub desired_replicas: i32,

    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
    pub last_scale_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// observedGeneration is the most recent generation observed by this autoscaler.
    pub observed_generation: Option<i64>,
}

impl crate::DeepMerge for HorizontalPodAutoscalerStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.conditions, other.conditions);
        crate::merge_strategies::list::atomic(&mut self.current_metrics, other.current_metrics);
        crate::DeepMerge::merge_from(&mut self.current_replicas, other.current_replicas);
        crate::DeepMerge::merge_from(&mut self.desired_replicas, other.desired_replicas);
        crate::DeepMerge::merge_from(&mut self.last_scale_time, other.last_scale_time);
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
    }
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
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_metrics => value_current_metrics = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_replicas => value_current_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_desired_replicas => value_desired_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_scale_time => value_last_scale_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerStatus {
                    conditions: value_conditions.unwrap_or_default(),
                    current_metrics: value_current_metrics,
                    current_replicas: value_current_replicas.unwrap_or_default(),
                    desired_replicas: value_desired_replicas.unwrap_or_default(),
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
            3 +
            self.current_metrics.as_ref().map_or(0, |_| 1) +
            self.last_scale_time.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        if let Some(value) = &self.current_metrics {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentMetrics", value)?;
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HorizontalPodAutoscalerStatus {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "currentMetrics".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentMetrics is the last read state of the metrics used by this autoscaler.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::autoscaling::v2beta1::MetricStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "currentReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "desiredReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lastScaleTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "observedGeneration".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("observedGeneration is the most recent generation observed by this autoscaler.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "conditions".to_owned(),
                    "currentReplicas".to_owned(),
                    "desiredReplicas".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
