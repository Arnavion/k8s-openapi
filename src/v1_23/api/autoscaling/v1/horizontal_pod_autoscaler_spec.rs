// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerSpec

/// specification of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerSpec {
    /// upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    pub max_replicas: i32,

    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    pub min_replicas: Option<i32>,

    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
    pub scale_target_ref: crate::api::autoscaling::v1::CrossVersionObjectReference,

    /// target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
    pub target_cpu_utilization_percentage: Option<i32>,

}

#[cfg(feature = "dsl")]
impl HorizontalPodAutoscalerSpec  {
    /// Set [`Self::max_replicas`]
    pub  fn max_replicas_set(&mut self, max_replicas: impl Into<i32>) -> &mut Self {
        self.max_replicas = max_replicas.into(); self
    }

    pub  fn max_replicas(&mut self) -> &mut i32 {
        &mut self.max_replicas
    }

    /// Modify [`Self::max_replicas`] with a `func`
    pub  fn max_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.max_replicas); self
    }


    /// Set [`Self::min_replicas`]
    pub  fn min_replicas_set(&mut self, min_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.min_replicas = min_replicas.into(); self
    }

    pub  fn min_replicas(&mut self) -> &mut i32 {
        if self.min_replicas.is_none() { self.min_replicas = Some(Default::default()) }
        self.min_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::min_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn min_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.min_replicas.is_none() { self.min_replicas = Some(Default::default()) };
        func(self.min_replicas.as_mut().unwrap()); self
    }


    /// Set [`Self::scale_target_ref`]
    pub  fn scale_target_ref_set(&mut self, scale_target_ref: impl Into<crate::api::autoscaling::v1::CrossVersionObjectReference>) -> &mut Self {
        self.scale_target_ref = scale_target_ref.into(); self
    }

    pub  fn scale_target_ref(&mut self) -> &mut crate::api::autoscaling::v1::CrossVersionObjectReference {
        &mut self.scale_target_ref
    }

    /// Modify [`Self::scale_target_ref`] with a `func`
    pub  fn scale_target_ref_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v1::CrossVersionObjectReference)) -> &mut Self {
        func(&mut self.scale_target_ref); self
    }


    /// Set [`Self::target_cpu_utilization_percentage`]
    pub  fn target_cpu_utilization_percentage_set(&mut self, target_cpu_utilization_percentage: impl Into<Option<i32>>) -> &mut Self {
        self.target_cpu_utilization_percentage = target_cpu_utilization_percentage.into(); self
    }

    pub  fn target_cpu_utilization_percentage(&mut self) -> &mut i32 {
        if self.target_cpu_utilization_percentage.is_none() { self.target_cpu_utilization_percentage = Some(Default::default()) }
        self.target_cpu_utilization_percentage.as_mut().unwrap()
    }

    /// Modify [`Self::target_cpu_utilization_percentage`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn target_cpu_utilization_percentage_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.target_cpu_utilization_percentage.is_none() { self.target_cpu_utilization_percentage = Some(Default::default()) };
        func(self.target_cpu_utilization_percentage.as_mut().unwrap()); self
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("specification of a horizontal pod autoscaler.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "maxReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "scaleTargetRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v1::CrossVersionObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "targetCPUUtilizationPercentage".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "maxReplicas".to_owned(),
                    "scaleTargetRef".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
