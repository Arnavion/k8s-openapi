// Generated from definition io.k8s.api.extensions.v1beta1.DeploymentSpec

/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    pub min_ready_seconds: Option<i32>,

    /// Indicates that the deployment is paused and will not be processed by the deployment controller.
    pub paused: Option<bool>,

    /// The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. This is set to the max value of int32 (i.e. 2147483647) by default, which means "no deadline".
    pub progress_deadline_seconds: Option<i32>,

    /// Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
    pub replicas: Option<i32>,

    /// The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. This is set to the max value of int32 (i.e. 2147483647) by default, which means "retaining all old RelicaSets".
    pub revision_history_limit: Option<i32>,

    /// DEPRECATED. The config this deployment is rolling back to. Will be cleared after rollback is done.
    pub rollback_to: Option<crate::api::extensions::v1beta1::RollbackConfig>,

    /// Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// The deployment strategy to use to replace existing pods with new ones.
    pub strategy: Option<crate::api::extensions::v1beta1::DeploymentStrategy>,

    /// Template describes the pods that will be created.
    pub template: crate::api::core::v1::PodTemplateSpec,
}

impl<'de> crate::serde::Deserialize<'de> for DeploymentSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_paused,
            Key_progress_deadline_seconds,
            Key_replicas,
            Key_revision_history_limit,
            Key_rollback_to,
            Key_selector,
            Key_strategy,
            Key_template,
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
                            "minReadySeconds" => Field::Key_min_ready_seconds,
                            "paused" => Field::Key_paused,
                            "progressDeadlineSeconds" => Field::Key_progress_deadline_seconds,
                            "replicas" => Field::Key_replicas,
                            "revisionHistoryLimit" => Field::Key_revision_history_limit,
                            "rollbackTo" => Field::Key_rollback_to,
                            "selector" => Field::Key_selector,
                            "strategy" => Field::Key_strategy,
                            "template" => Field::Key_template,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_paused: Option<bool> = None;
                let mut value_progress_deadline_seconds: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_rollback_to: Option<crate::api::extensions::v1beta1::RollbackConfig> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_strategy: Option<crate::api::extensions::v1beta1::DeploymentStrategy> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_paused => value_paused = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_progress_deadline_seconds => value_progress_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rollback_to => value_rollback_to = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    paused: value_paused,
                    progress_deadline_seconds: value_progress_deadline_seconds,
                    replicas: value_replicas,
                    revision_history_limit: value_revision_history_limit,
                    rollback_to: value_rollback_to,
                    selector: value_selector,
                    strategy: value_strategy,
                    template: value_template.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentSpec",
            &[
                "minReadySeconds",
                "paused",
                "progressDeadlineSeconds",
                "replicas",
                "revisionHistoryLimit",
                "rollbackTo",
                "selector",
                "strategy",
                "template",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeploymentSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentSpec",
            1 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.paused.as_ref().map_or(0, |_| 1) +
            self.progress_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            self.rollback_to.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.strategy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.paused {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "paused", value)?;
        }
        if let Some(value) = &self.progress_deadline_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "progressDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        if let Some(value) = &self.rollback_to {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rollbackTo", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.strategy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeploymentSpec {
    fn schema_name() -> String {
        "io.k8s.api.extensions.v1beta1.DeploymentSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("DeploymentSpec is the specification of the desired behavior of the Deployment.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "minReadySeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "paused".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Indicates that the deployment is paused and will not be processed by the deployment controller.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "progressDeadlineSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. This is set to the max value of int32 (i.e. 2147483647) by default, which means \"no deadline\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "replicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "revisionHistoryLimit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. This is set to the max value of int32 (i.e. 2147483647) by default, which means \"retaining all old RelicaSets\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "rollbackTo".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::extensions::v1beta1::RollbackConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DEPRECATED. The config this deployment is rolling back to. Will be cleared after rollback is done.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "selector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "strategy".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::extensions::v1beta1::DeploymentStrategy>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The deployment strategy to use to replace existing pods with new ones.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "template".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Template describes the pods that will be created.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: IntoIterator::into_iter([
                    "template",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
