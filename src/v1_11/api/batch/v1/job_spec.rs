// Generated from definition io.k8s.api.batch.v1.JobSpec

/// JobSpec describes how the job execution will look like.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobSpec {
    /// Specifies the duration in seconds relative to the startTime that the job may be active before the system tries to terminate it; value must be positive integer
    pub active_deadline_seconds: Option<i64>,

    /// Specifies the number of retries before marking this job failed. Defaults to 6
    pub backoff_limit: Option<i32>,

    /// Specifies the desired number of successfully finished pods the job should be run with.  Setting to nil means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub completions: Option<i32>,

    /// manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    pub manual_selector: Option<bool>,

    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) \< .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub parallelism: Option<i32>,

    /// A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Describes the pod that will be created when executing a job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub template: crate::api::core::v1::PodTemplateSpec,
}

impl<'de> crate::serde::Deserialize<'de> for JobSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active_deadline_seconds,
            Key_backoff_limit,
            Key_completions,
            Key_manual_selector,
            Key_parallelism,
            Key_selector,
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
                            "activeDeadlineSeconds" => Field::Key_active_deadline_seconds,
                            "backoffLimit" => Field::Key_backoff_limit,
                            "completions" => Field::Key_completions,
                            "manualSelector" => Field::Key_manual_selector,
                            "parallelism" => Field::Key_parallelism,
                            "selector" => Field::Key_selector,
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
            type Value = JobSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JobSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active_deadline_seconds: Option<i64> = None;
                let mut value_backoff_limit: Option<i32> = None;
                let mut value_completions: Option<i32> = None;
                let mut value_manual_selector: Option<bool> = None;
                let mut value_parallelism: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_backoff_limit => value_backoff_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completions => value_completions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_manual_selector => value_manual_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parallelism => value_parallelism = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JobSpec {
                    active_deadline_seconds: value_active_deadline_seconds,
                    backoff_limit: value_backoff_limit,
                    completions: value_completions,
                    manual_selector: value_manual_selector,
                    parallelism: value_parallelism,
                    selector: value_selector,
                    template: value_template.ok_or_else(|| crate::serde::de::Error::missing_field("template"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "JobSpec",
            &[
                "activeDeadlineSeconds",
                "backoffLimit",
                "completions",
                "manualSelector",
                "parallelism",
                "selector",
                "template",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for JobSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JobSpec",
            1 +
            self.active_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.backoff_limit.as_ref().map_or(0, |_| 1) +
            self.completions.as_ref().map_or(0, |_| 1) +
            self.manual_selector.as_ref().map_or(0, |_| 1) +
            self.parallelism.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active_deadline_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "activeDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.backoff_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "backoffLimit", value)?;
        }
        if let Some(value) = &self.completions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completions", value)?;
        }
        if let Some(value) = &self.manual_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "manualSelector", value)?;
        }
        if let Some(value) = &self.parallelism {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parallelism", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for JobSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "JobSpec describes how the job execution will look like.",
          "properties": {
            "activeDeadlineSeconds": {
              "description": "Specifies the duration in seconds relative to the startTime that the job may be active before the system tries to terminate it; value must be positive integer",
              "format": "int64",
              "type": "integer"
            },
            "backoffLimit": {
              "description": "Specifies the number of retries before marking this job failed. Defaults to 6",
              "format": "int32",
              "type": "integer"
            },
            "completions": {
              "description": "Specifies the desired number of successfully finished pods the job should be run with.  Setting to nil means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/",
              "format": "int32",
              "type": "integer"
            },
            "manualSelector": {
              "description": "manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector",
              "type": "boolean"
            },
            "parallelism": {
              "description": "Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/",
              "format": "int32",
              "type": "integer"
            },
            "selector": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::LabelSelector::schema(), "A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors"),
            "template": crate::schema_ref_with_description(crate::api::core::v1::PodTemplateSpec::schema(), "Describes the pod that will be created when executing a job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/")
          },
          "required": [
            "template"
          ],
          "type": "object"
        })
    }
}
