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
                        Field::Key_template => value_template = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
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
                    template: value_template.ok_or_else(|| crate::serde::de::Error::missing_field("template"))?,
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

#[cfg(feature = "schema")]
impl crate::Schema for DeploymentSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "DeploymentSpec is the specification of the desired behavior of the Deployment.",
          "properties": {
            "minReadySeconds": {
              "description": "Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)",
              "format": "int32",
              "type": "integer"
            },
            "paused": {
              "description": "Indicates that the deployment is paused and will not be processed by the deployment controller.",
              "type": "boolean"
            },
            "progressDeadlineSeconds": {
              "description": "The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. This is set to the max value of int32 (i.e. 2147483647) by default, which means \"no deadline\".",
              "format": "int32",
              "type": "integer"
            },
            "replicas": {
              "description": "Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.",
              "format": "int32",
              "type": "integer"
            },
            "revisionHistoryLimit": {
              "description": "The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. This is set to the max value of int32 (i.e. 2147483647) by default, which means \"retaining all old RelicaSets\".",
              "format": "int32",
              "type": "integer"
            },
            "rollbackTo": crate::schema_ref_with_description(crate::api::extensions::v1beta1::RollbackConfig::schema(), "DEPRECATED. The config this deployment is rolling back to. Will be cleared after rollback is done."),
            "selector": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::LabelSelector::schema(), "Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment."),
            "strategy": crate::schema_ref_with_description(crate::api::extensions::v1beta1::DeploymentStrategy::schema(), "The deployment strategy to use to replace existing pods with new ones."),
            "template": crate::schema_ref_with_description(crate::api::core::v1::PodTemplateSpec::schema(), "Template describes the pods that will be created.")
          },
          "required": [
            "template"
          ],
          "type": "object"
        })
    }
}
