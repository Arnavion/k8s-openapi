// Generated from definition io.k8s.api.autoscaling.v2.HPAScalingRules

/// HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance.
///
/// Scaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
///
/// The tolerance is applied to the metric values and prevents scaling too eagerly for small metric variations. (Note that setting a tolerance requires enabling the alpha HPAConfigurableTolerance feature gate.)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HPAScalingRules {
    /// policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.
    pub policies: Option<std::vec::Vec<crate::api::autoscaling::v2::HPAScalingPolicy>>,

    /// selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
    pub select_policy: Option<std::string::String>,

    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    pub stabilization_window_seconds: Option<i32>,

    /// tolerance is the tolerance on the ratio between the current and desired metric value under which no updates are made to the desired number of replicas (e.g. 0.01 for 1%). Must be greater than or equal to zero. If not set, the default cluster-wide tolerance is applied (by default 10%).
    ///
    /// For example, if autoscaling is configured with a memory consumption target of 100Mi, and scale-down and scale-up tolerances of 5% and 1% respectively, scaling will be triggered when the actual consumption falls below 95Mi or exceeds 101Mi.
    ///
    /// This is an alpha field and requires enabling the HPAConfigurableTolerance feature gate.
    pub tolerance: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl crate::DeepMerge for HPAScalingRules {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.policies, other.policies);
        crate::DeepMerge::merge_from(&mut self.select_policy, other.select_policy);
        crate::DeepMerge::merge_from(&mut self.stabilization_window_seconds, other.stabilization_window_seconds);
        crate::DeepMerge::merge_from(&mut self.tolerance, other.tolerance);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HPAScalingRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_policies,
            Key_select_policy,
            Key_stabilization_window_seconds,
            Key_tolerance,
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
                            "policies" => Field::Key_policies,
                            "selectPolicy" => Field::Key_select_policy,
                            "stabilizationWindowSeconds" => Field::Key_stabilization_window_seconds,
                            "tolerance" => Field::Key_tolerance,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HPAScalingRules;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HPAScalingRules")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_policies: Option<std::vec::Vec<crate::api::autoscaling::v2::HPAScalingPolicy>> = None;
                let mut value_select_policy: Option<std::string::String> = None;
                let mut value_stabilization_window_seconds: Option<i32> = None;
                let mut value_tolerance: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_policies => value_policies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_select_policy => value_select_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stabilization_window_seconds => value_stabilization_window_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerance => value_tolerance = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HPAScalingRules {
                    policies: value_policies,
                    select_policy: value_select_policy,
                    stabilization_window_seconds: value_stabilization_window_seconds,
                    tolerance: value_tolerance,
                })
            }
        }

        deserializer.deserialize_struct(
            "HPAScalingRules",
            &[
                "policies",
                "selectPolicy",
                "stabilizationWindowSeconds",
                "tolerance",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HPAScalingRules {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HPAScalingRules",
            self.policies.as_ref().map_or(0, |_| 1) +
            self.select_policy.as_ref().map_or(0, |_| 1) +
            self.stabilization_window_seconds.as_ref().map_or(0, |_| 1) +
            self.tolerance.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.policies {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "policies", value)?;
        }
        if let Some(value) = &self.select_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectPolicy", value)?;
        }
        if let Some(value) = &self.stabilization_window_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stabilizationWindowSeconds", value)?;
        }
        if let Some(value) = &self.tolerance {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerance", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HPAScalingRules {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.HPAScalingRules".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance.\n\nScaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.\n\nThe tolerance is applied to the metric values and prevents scaling too eagerly for small metric variations. (Note that setting a tolerance requires enabling the alpha HPAConfigurableTolerance feature gate.)".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "policies".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::autoscaling::v2::HPAScalingPolicy>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selectPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "stabilizationWindowSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tolerance".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("tolerance is the tolerance on the ratio between the current and desired metric value under which no updates are made to the desired number of replicas (e.g. 0.01 for 1%). Must be greater than or equal to zero. If not set, the default cluster-wide tolerance is applied (by default 10%).\n\nFor example, if autoscaling is configured with a memory consumption target of 100Mi, and scale-down and scale-up tolerances of 5% and 1% respectively, scaling will be triggered when the actual consumption falls below 95Mi or exceeds 101Mi.\n\nThis is an alpha field and requires enabling the HPAConfigurableTolerance feature gate.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
