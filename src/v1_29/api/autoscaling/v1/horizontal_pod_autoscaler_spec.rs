// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerSpec

/// specification of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerSpec {
    /// maxReplicas is the upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    pub max_replicas: i32,

    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    pub min_replicas: Option<i32>,

    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
    pub scale_target_ref: crate::api::autoscaling::v1::CrossVersionObjectReference,

    /// targetCPUUtilizationPercentage is the target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
    pub target_cpu_utilization_percentage: Option<i32>,
}

impl crate::DeepMerge for HorizontalPodAutoscalerSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.max_replicas, other.max_replicas);
        crate::DeepMerge::merge_from(&mut self.min_replicas, other.min_replicas);
        crate::DeepMerge::merge_from(&mut self.scale_target_ref, other.scale_target_ref);
        crate::DeepMerge::merge_from(&mut self.target_cpu_utilization_percentage, other.target_cpu_utilization_percentage);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HorizontalPodAutoscalerSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_replicas,
            Key_min_replicas,
            Key_scale_target_ref,
            Key_target_cpu_utilization_percentage,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "maxReplicas" => Field::Key_max_replicas,
                            "minReplicas" => Field::Key_min_replicas,
                            "scaleTargetRef" => Field::Key_scale_target_ref,
                            "targetCPUUtilizationPercentage" => Field::Key_target_cpu_utilization_percentage,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HorizontalPodAutoscalerSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max_replicas: Option<i32> = None;
                let mut value_min_replicas: Option<i32> = None;
                let mut value_scale_target_ref: Option<crate::api::autoscaling::v1::CrossVersionObjectReference> = None;
                let mut value_target_cpu_utilization_percentage: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_replicas => value_max_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_replicas => value_min_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_target_ref => value_scale_target_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_cpu_utilization_percentage => value_target_cpu_utilization_percentage = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerSpec {
                    max_replicas: value_max_replicas.unwrap_or_default(),
                    min_replicas: value_min_replicas,
                    scale_target_ref: value_scale_target_ref.unwrap_or_default(),
                    target_cpu_utilization_percentage: value_target_cpu_utilization_percentage,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerSpec",
            &[
                "maxReplicas",
                "minReplicas",
                "scaleTargetRef",
                "targetCPUUtilizationPercentage",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HorizontalPodAutoscalerSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerSpec",
            2 +
            self.min_replicas.as_ref().map_or(0, |_| 1) +
            self.target_cpu_utilization_percentage.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxReplicas", &self.max_replicas)?;
        if let Some(value) = &self.min_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleTargetRef", &self.scale_target_ref)?;
        if let Some(value) = &self.target_cpu_utilization_percentage {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetCPUUtilizationPercentage", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HorizontalPodAutoscalerSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("specification of a horizontal pod autoscaler.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "maxReplicas".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("maxReplicas is the upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minReplicas".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "scaleTargetRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v1::CrossVersionObjectReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "targetCPUUtilizationPercentage".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetCPUUtilizationPercentage is the target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "maxReplicas".into(),
                    "scaleTargetRef".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
