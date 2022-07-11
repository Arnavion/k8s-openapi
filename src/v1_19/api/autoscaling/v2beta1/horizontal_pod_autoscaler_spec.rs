// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerSpec

/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerSpec {
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
    pub max_replicas: i32,

    /// metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond.
    pub metrics: Option<Vec<crate::api::autoscaling::v2beta1::MetricSpec>>,

    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    pub min_replicas: Option<i32>,

    /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
    pub scale_target_ref: crate::api::autoscaling::v2beta1::CrossVersionObjectReference,

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


    /// Set [`Self::metrics`]
    pub  fn metrics_set(&mut self, metrics: impl Into<Option<Vec<crate::api::autoscaling::v2beta1::MetricSpec>>>) -> &mut Self {
        self.metrics = metrics.into(); self
    }

    pub  fn metrics(&mut self) -> &mut Vec<crate::api::autoscaling::v2beta1::MetricSpec> {
        if self.metrics.is_none() { self.metrics = Some(Default::default()) }
        self.metrics.as_mut().unwrap()
    }

    /// Modify [`Self::metrics`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn metrics_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::autoscaling::v2beta1::MetricSpec>)) -> &mut Self {
        if self.metrics.is_none() { self.metrics = Some(Default::default()) };
        func(self.metrics.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::metrics`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn metrics_push_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2beta1::MetricSpec)) -> &mut Self {
        if self.metrics.is_none() {
            self.metrics = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.metrics.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::metrics`]
    pub  fn metrics_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::autoscaling::v2beta1::MetricSpec]>) -> &mut Self {
         if self.metrics.is_none() { self.metrics = Some(Vec::new()); }
         let metrics = &mut self.metrics.as_mut().unwrap();
         for item in other.borrow() {
             metrics.push(item.to_owned());
         }
         self
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
    pub  fn scale_target_ref_set(&mut self, scale_target_ref: impl Into<crate::api::autoscaling::v2beta1::CrossVersionObjectReference>) -> &mut Self {
        self.scale_target_ref = scale_target_ref.into(); self
    }

    pub  fn scale_target_ref(&mut self) -> &mut crate::api::autoscaling::v2beta1::CrossVersionObjectReference {
        &mut self.scale_target_ref
    }

    /// Modify [`Self::scale_target_ref`] with a `func`
    pub  fn scale_target_ref_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2beta1::CrossVersionObjectReference)) -> &mut Self {
        func(&mut self.scale_target_ref); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for HorizontalPodAutoscalerSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_replicas,
            Key_metrics,
            Key_min_replicas,
            Key_scale_target_ref,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HorizontalPodAutoscalerSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max_replicas: Option<i32> = None;
                let mut value_metrics: Option<Vec<crate::api::autoscaling::v2beta1::MetricSpec>> = None;
                let mut value_min_replicas: Option<i32> = None;
                let mut value_scale_target_ref: Option<crate::api::autoscaling::v2beta1::CrossVersionObjectReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_replicas => value_max_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metrics => value_metrics = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_replicas => value_min_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_target_ref => value_scale_target_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerSpec {
                    max_replicas: value_max_replicas.unwrap_or_default(),
                    metrics: value_metrics,
                    min_replicas: value_min_replicas,
                    scale_target_ref: value_scale_target_ref.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerSpec",
            &[
                "maxReplicas",
                "metrics",
                "minReplicas",
                "scaleTargetRef",
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
            self.metrics.as_ref().map_or(0, |_| 1) +
            self.min_replicas.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxReplicas", &self.max_replicas)?;
        if let Some(value) = &self.metrics {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metrics", value)?;
        }
        if let Some(value) = &self.min_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleTargetRef", &self.scale_target_ref)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HorizontalPodAutoscalerSpec {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "maxReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metrics".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::autoscaling::v2beta1::MetricSpec>()))),
                                ..Default::default()
                            })),
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
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2beta1::CrossVersionObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
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
