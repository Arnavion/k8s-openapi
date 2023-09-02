// Generated from definition io.k8s.api.batch.v1.JobSpec

/// JobSpec describes how the job execution will look like.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobSpec {
    /// Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.
    pub active_deadline_seconds: Option<i64>,

    /// Specifies the number of retries before marking this job failed. Defaults to 6
    pub backoff_limit: Option<i32>,

    /// Specifies the limit for the number of retries within an index before marking this index as failed. When enabled the number of failures per index is kept in the pod's batch.kubernetes.io/job-index-failure-count annotation. It can only be set when Job's completionMode=Indexed, and the Pod's restart policy is Never. The field is immutable. This field is alpha-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (disabled by default).
    pub backoff_limit_per_index: Option<i32>,

    /// completionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.
    ///
    /// `NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.
    ///
    /// `Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.
    ///
    /// More completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, which is possible during upgrades due to version skew, the controller skips updates for the Job.
    pub completion_mode: Option<String>,

    /// Specifies the desired number of successfully finished pods the job should be run with.  Setting to null means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub completions: Option<i32>,

    /// manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    pub manual_selector: Option<bool>,

    /// Specifies the maximal number of failed indexes before marking the Job as failed, when backoffLimitPerIndex is set. Once the number of failed indexes exceeds this number the entire Job is marked as Failed and its execution is terminated. When left as null the job continues execution of all of its indexes and is marked with the `Complete` Job condition. It can only be specified when backoffLimitPerIndex is set. It can be null or up to completions. It is required and must be less than or equal to 10^4 when is completions greater than 10^5. This field is alpha-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (disabled by default).
    pub max_failed_indexes: Option<i32>,

    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) \< .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub parallelism: Option<i32>,

    /// Specifies the policy of handling failed pods. In particular, it allows to specify the set of actions and conditions which need to be satisfied to take the associated action. If empty, the default behaviour applies - the counter of failed pods, represented by the jobs's .status.failed field, is incremented and it is checked against the backoffLimit. This field cannot be used in combination with restartPolicy=OnFailure.
    ///
    /// This field is beta-level. It can be used when the `JobPodFailurePolicy` feature gate is enabled (enabled by default).
    pub pod_failure_policy: Option<crate::api::batch::v1::PodFailurePolicy>,

    /// podReplacementPolicy specifies when to create replacement Pods. Possible values are: - TerminatingOrFailed means that we recreate pods
    ///   when they are terminating (has a metadata.deletionTimestamp) or failed.
    /// - Failed means to wait until a previously created Pod is fully terminated (has phase
    ///   Failed or Succeeded) before creating a replacement Pod.
    ///
    /// When using podFailurePolicy, Failed is the the only allowed value. TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use. This is an alpha field. Enable JobPodReplacementPolicy to be able to use this field.
    pub pod_replacement_policy: Option<String>,

    /// A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to true, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from false to true), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the StartTime field of the Job, effectively resetting the ActiveDeadlineSeconds timer too. Defaults to false.
    pub suspend: Option<bool>,

    /// Describes the pod that will be created when executing a job. The only allowed template.spec.restartPolicy values are "Never" or "OnFailure". More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub template: crate::api::core::v1::PodTemplateSpec,

    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes.
    pub ttl_seconds_after_finished: Option<i32>,
}

impl crate::DeepMerge for JobSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.active_deadline_seconds, other.active_deadline_seconds);
        crate::DeepMerge::merge_from(&mut self.backoff_limit, other.backoff_limit);
        crate::DeepMerge::merge_from(&mut self.backoff_limit_per_index, other.backoff_limit_per_index);
        crate::DeepMerge::merge_from(&mut self.completion_mode, other.completion_mode);
        crate::DeepMerge::merge_from(&mut self.completions, other.completions);
        crate::DeepMerge::merge_from(&mut self.manual_selector, other.manual_selector);
        crate::DeepMerge::merge_from(&mut self.max_failed_indexes, other.max_failed_indexes);
        crate::DeepMerge::merge_from(&mut self.parallelism, other.parallelism);
        crate::DeepMerge::merge_from(&mut self.pod_failure_policy, other.pod_failure_policy);
        crate::DeepMerge::merge_from(&mut self.pod_replacement_policy, other.pod_replacement_policy);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.suspend, other.suspend);
        crate::DeepMerge::merge_from(&mut self.template, other.template);
        crate::DeepMerge::merge_from(&mut self.ttl_seconds_after_finished, other.ttl_seconds_after_finished);
    }
}

impl<'de> crate::serde::Deserialize<'de> for JobSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active_deadline_seconds,
            Key_backoff_limit,
            Key_backoff_limit_per_index,
            Key_completion_mode,
            Key_completions,
            Key_manual_selector,
            Key_max_failed_indexes,
            Key_parallelism,
            Key_pod_failure_policy,
            Key_pod_replacement_policy,
            Key_selector,
            Key_suspend,
            Key_template,
            Key_ttl_seconds_after_finished,
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
                            "backoffLimitPerIndex" => Field::Key_backoff_limit_per_index,
                            "completionMode" => Field::Key_completion_mode,
                            "completions" => Field::Key_completions,
                            "manualSelector" => Field::Key_manual_selector,
                            "maxFailedIndexes" => Field::Key_max_failed_indexes,
                            "parallelism" => Field::Key_parallelism,
                            "podFailurePolicy" => Field::Key_pod_failure_policy,
                            "podReplacementPolicy" => Field::Key_pod_replacement_policy,
                            "selector" => Field::Key_selector,
                            "suspend" => Field::Key_suspend,
                            "template" => Field::Key_template,
                            "ttlSecondsAfterFinished" => Field::Key_ttl_seconds_after_finished,
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
                let mut value_backoff_limit_per_index: Option<i32> = None;
                let mut value_completion_mode: Option<String> = None;
                let mut value_completions: Option<i32> = None;
                let mut value_manual_selector: Option<bool> = None;
                let mut value_max_failed_indexes: Option<i32> = None;
                let mut value_parallelism: Option<i32> = None;
                let mut value_pod_failure_policy: Option<crate::api::batch::v1::PodFailurePolicy> = None;
                let mut value_pod_replacement_policy: Option<String> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_suspend: Option<bool> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;
                let mut value_ttl_seconds_after_finished: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_backoff_limit => value_backoff_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_backoff_limit_per_index => value_backoff_limit_per_index = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completion_mode => value_completion_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completions => value_completions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_manual_selector => value_manual_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_failed_indexes => value_max_failed_indexes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parallelism => value_parallelism = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_failure_policy => value_pod_failure_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_replacement_policy => value_pod_replacement_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_suspend => value_suspend = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ttl_seconds_after_finished => value_ttl_seconds_after_finished = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JobSpec {
                    active_deadline_seconds: value_active_deadline_seconds,
                    backoff_limit: value_backoff_limit,
                    backoff_limit_per_index: value_backoff_limit_per_index,
                    completion_mode: value_completion_mode,
                    completions: value_completions,
                    manual_selector: value_manual_selector,
                    max_failed_indexes: value_max_failed_indexes,
                    parallelism: value_parallelism,
                    pod_failure_policy: value_pod_failure_policy,
                    pod_replacement_policy: value_pod_replacement_policy,
                    selector: value_selector,
                    suspend: value_suspend,
                    template: value_template.unwrap_or_default(),
                    ttl_seconds_after_finished: value_ttl_seconds_after_finished,
                })
            }
        }

        deserializer.deserialize_struct(
            "JobSpec",
            &[
                "activeDeadlineSeconds",
                "backoffLimit",
                "backoffLimitPerIndex",
                "completionMode",
                "completions",
                "manualSelector",
                "maxFailedIndexes",
                "parallelism",
                "podFailurePolicy",
                "podReplacementPolicy",
                "selector",
                "suspend",
                "template",
                "ttlSecondsAfterFinished",
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
            self.backoff_limit_per_index.as_ref().map_or(0, |_| 1) +
            self.completion_mode.as_ref().map_or(0, |_| 1) +
            self.completions.as_ref().map_or(0, |_| 1) +
            self.manual_selector.as_ref().map_or(0, |_| 1) +
            self.max_failed_indexes.as_ref().map_or(0, |_| 1) +
            self.parallelism.as_ref().map_or(0, |_| 1) +
            self.pod_failure_policy.as_ref().map_or(0, |_| 1) +
            self.pod_replacement_policy.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.suspend.as_ref().map_or(0, |_| 1) +
            self.ttl_seconds_after_finished.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active_deadline_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "activeDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.backoff_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "backoffLimit", value)?;
        }
        if let Some(value) = &self.backoff_limit_per_index {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "backoffLimitPerIndex", value)?;
        }
        if let Some(value) = &self.completion_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completionMode", value)?;
        }
        if let Some(value) = &self.completions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completions", value)?;
        }
        if let Some(value) = &self.manual_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "manualSelector", value)?;
        }
        if let Some(value) = &self.max_failed_indexes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxFailedIndexes", value)?;
        }
        if let Some(value) = &self.parallelism {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parallelism", value)?;
        }
        if let Some(value) = &self.pod_failure_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podFailurePolicy", value)?;
        }
        if let Some(value) = &self.pod_replacement_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podReplacementPolicy", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.suspend {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "suspend", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.ttl_seconds_after_finished {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ttlSecondsAfterFinished", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JobSpec {
    fn schema_name() -> String {
        "io.k8s.api.batch.v1.JobSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("JobSpec describes how the job execution will look like.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "activeDeadlineSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "backoffLimit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the number of retries before marking this job failed. Defaults to 6".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "backoffLimitPerIndex".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the limit for the number of retries within an index before marking this index as failed. When enabled the number of failures per index is kept in the pod's batch.kubernetes.io/job-index-failure-count annotation. It can only be set when Job's completionMode=Indexed, and the Pod's restart policy is Never. The field is immutable. This field is alpha-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (disabled by default).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completionMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("completionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.\n\n`NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.\n\n`Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.\n\nMore completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, which is possible during upgrades due to version skew, the controller skips updates for the Job.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the desired number of successfully finished pods the job should be run with.  Setting to null means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "manualSelector".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxFailedIndexes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the maximal number of failed indexes before marking the Job as failed, when backoffLimitPerIndex is set. Once the number of failed indexes exceeds this number the entire Job is marked as Failed and its execution is terminated. When left as null the job continues execution of all of its indexes and is marked with the `Complete` Job condition. It can only be specified when backoffLimitPerIndex is set. It can be null or up to completions. It is required and must be less than or equal to 10^4 when is completions greater than 10^5. This field is alpha-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (disabled by default).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "parallelism".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podFailurePolicy".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::PodFailurePolicy>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the policy of handling failed pods. In particular, it allows to specify the set of actions and conditions which need to be satisfied to take the associated action. If empty, the default behaviour applies - the counter of failed pods, represented by the jobs's .status.failed field, is incremented and it is checked against the backoffLimit. This field cannot be used in combination with restartPolicy=OnFailure.\n\nThis field is beta-level. It can be used when the `JobPodFailurePolicy` feature gate is enabled (enabled by default).".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "podReplacementPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("podReplacementPolicy specifies when to create replacement Pods. Possible values are: - TerminatingOrFailed means that we recreate pods\n  when they are terminating (has a metadata.deletionTimestamp) or failed.\n- Failed means to wait until a previously created Pod is fully terminated (has phase\n  Failed or Succeeded) before creating a replacement Pod.\n\nWhen using podFailurePolicy, Failed is the the only allowed value. TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use. This is an alpha field. Enable JobPodReplacementPolicy to be able to use this field.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "suspend".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to true, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from false to true), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the StartTime field of the Job, effectively resetting the ActiveDeadlineSeconds timer too. Defaults to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "template".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Describes the pod that will be created when executing a job. The only allowed template.spec.restartPolicy values are \"Never\" or \"OnFailure\". More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "ttlSecondsAfterFinished".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "template".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
