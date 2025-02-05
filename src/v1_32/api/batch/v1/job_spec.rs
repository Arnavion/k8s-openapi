// Generated from definition io.k8s.api.batch.v1.JobSpec

/// JobSpec describes how the job execution will look like.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobSpec {
    /// Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.
    pub active_deadline_seconds: Option<i64>,

    /// Specifies the number of retries before marking this job failed. Defaults to 6
    pub backoff_limit: Option<i32>,

    /// Specifies the limit for the number of retries within an index before marking this index as failed. When enabled the number of failures per index is kept in the pod's batch.kubernetes.io/job-index-failure-count annotation. It can only be set when Job's completionMode=Indexed, and the Pod's restart policy is Never. The field is immutable. This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).
    pub backoff_limit_per_index: Option<i32>,

    /// completionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.
    ///
    /// `NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.
    ///
    /// `Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.
    ///
    /// More completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, which is possible during upgrades due to version skew, the controller skips updates for the Job.
    pub completion_mode: Option<std::string::String>,

    /// Specifies the desired number of successfully finished pods the job should be run with.  Setting to null means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub completions: Option<i32>,

    /// ManagedBy field indicates the controller that manages a Job. The k8s Job controller reconciles jobs which don't have this field at all or the field value is the reserved string `kubernetes.io/job-controller`, but skips reconciling Jobs with a custom value for this field. The value must be a valid domain-prefixed path (e.g. acme.io/foo) - all characters before the first "/" must be a valid subdomain as defined by RFC 1123. All characters trailing the first "/" must be valid HTTP Path characters as defined by RFC 3986. The value cannot exceed 63 characters. This field is immutable.
    ///
    /// This field is beta-level. The job controller accepts setting the field when the feature gate JobManagedBy is enabled (enabled by default).
    pub managed_by: Option<std::string::String>,

    /// manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    pub manual_selector: Option<bool>,

    /// Specifies the maximal number of failed indexes before marking the Job as failed, when backoffLimitPerIndex is set. Once the number of failed indexes exceeds this number the entire Job is marked as Failed and its execution is terminated. When left as null the job continues execution of all of its indexes and is marked with the `Complete` Job condition. It can only be specified when backoffLimitPerIndex is set. It can be null or up to completions. It is required and must be less than or equal to 10^4 when is completions greater than 10^5. This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).
    pub max_failed_indexes: Option<i32>,

    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) \< .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub parallelism: Option<i32>,

    /// Specifies the policy of handling failed pods. In particular, it allows to specify the set of actions and conditions which need to be satisfied to take the associated action. If empty, the default behaviour applies - the counter of failed pods, represented by the jobs's .status.failed field, is incremented and it is checked against the backoffLimit. This field cannot be used in combination with restartPolicy=OnFailure.
    pub pod_failure_policy: Option<crate::api::batch::v1::PodFailurePolicy>,

    /// podReplacementPolicy specifies when to create replacement Pods. Possible values are: - TerminatingOrFailed means that we recreate pods
    ///   when they are terminating (has a metadata.deletionTimestamp) or failed.
    /// - Failed means to wait until a previously created Pod is fully terminated (has phase
    ///   Failed or Succeeded) before creating a replacement Pod.
    ///
    /// When using podFailurePolicy, Failed is the the only allowed value. TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use. This is an beta field. To use this, enable the JobPodReplacementPolicy feature toggle. This is on by default.
    pub pod_replacement_policy: Option<std::string::String>,

    /// A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// successPolicy specifies the policy when the Job can be declared as succeeded. If empty, the default behavior applies - the Job is declared as succeeded only when the number of succeeded pods equals to the completions. When the field is specified, it must be immutable and works only for the Indexed Jobs. Once the Job meets the SuccessPolicy, the lingering pods are terminated.
    ///
    /// This field is beta-level. To use this field, you must enable the `JobSuccessPolicy` feature gate (enabled by default).
    pub success_policy: Option<crate::api::batch::v1::SuccessPolicy>,

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
        crate::DeepMerge::merge_from(&mut self.managed_by, other.managed_by);
        crate::DeepMerge::merge_from(&mut self.manual_selector, other.manual_selector);
        crate::DeepMerge::merge_from(&mut self.max_failed_indexes, other.max_failed_indexes);
        crate::DeepMerge::merge_from(&mut self.parallelism, other.parallelism);
        crate::DeepMerge::merge_from(&mut self.pod_failure_policy, other.pod_failure_policy);
        crate::DeepMerge::merge_from(&mut self.pod_replacement_policy, other.pod_replacement_policy);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.success_policy, other.success_policy);
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
            Key_managed_by,
            Key_manual_selector,
            Key_max_failed_indexes,
            Key_parallelism,
            Key_pod_failure_policy,
            Key_pod_replacement_policy,
            Key_selector,
            Key_success_policy,
            Key_suspend,
            Key_template,
            Key_ttl_seconds_after_finished,
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
                            "activeDeadlineSeconds" => Field::Key_active_deadline_seconds,
                            "backoffLimit" => Field::Key_backoff_limit,
                            "backoffLimitPerIndex" => Field::Key_backoff_limit_per_index,
                            "completionMode" => Field::Key_completion_mode,
                            "completions" => Field::Key_completions,
                            "managedBy" => Field::Key_managed_by,
                            "manualSelector" => Field::Key_manual_selector,
                            "maxFailedIndexes" => Field::Key_max_failed_indexes,
                            "parallelism" => Field::Key_parallelism,
                            "podFailurePolicy" => Field::Key_pod_failure_policy,
                            "podReplacementPolicy" => Field::Key_pod_replacement_policy,
                            "selector" => Field::Key_selector,
                            "successPolicy" => Field::Key_success_policy,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JobSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active_deadline_seconds: Option<i64> = None;
                let mut value_backoff_limit: Option<i32> = None;
                let mut value_backoff_limit_per_index: Option<i32> = None;
                let mut value_completion_mode: Option<std::string::String> = None;
                let mut value_completions: Option<i32> = None;
                let mut value_managed_by: Option<std::string::String> = None;
                let mut value_manual_selector: Option<bool> = None;
                let mut value_max_failed_indexes: Option<i32> = None;
                let mut value_parallelism: Option<i32> = None;
                let mut value_pod_failure_policy: Option<crate::api::batch::v1::PodFailurePolicy> = None;
                let mut value_pod_replacement_policy: Option<std::string::String> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_success_policy: Option<crate::api::batch::v1::SuccessPolicy> = None;
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
                        Field::Key_managed_by => value_managed_by = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_manual_selector => value_manual_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_failed_indexes => value_max_failed_indexes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parallelism => value_parallelism = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_failure_policy => value_pod_failure_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_replacement_policy => value_pod_replacement_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_success_policy => value_success_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    managed_by: value_managed_by,
                    manual_selector: value_manual_selector,
                    max_failed_indexes: value_max_failed_indexes,
                    parallelism: value_parallelism,
                    pod_failure_policy: value_pod_failure_policy,
                    pod_replacement_policy: value_pod_replacement_policy,
                    selector: value_selector,
                    success_policy: value_success_policy,
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
                "managedBy",
                "manualSelector",
                "maxFailedIndexes",
                "parallelism",
                "podFailurePolicy",
                "podReplacementPolicy",
                "selector",
                "successPolicy",
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
            self.managed_by.as_ref().map_or(0, |_| 1) +
            self.manual_selector.as_ref().map_or(0, |_| 1) +
            self.max_failed_indexes.as_ref().map_or(0, |_| 1) +
            self.parallelism.as_ref().map_or(0, |_| 1) +
            self.pod_failure_policy.as_ref().map_or(0, |_| 1) +
            self.pod_replacement_policy.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.success_policy.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.managed_by {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "managedBy", value)?;
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
        if let Some(value) = &self.success_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "successPolicy", value)?;
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
    fn schema_name() -> std::string::String {
        "io.k8s.api.batch.v1.JobSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("JobSpec describes how the job execution will look like.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "activeDeadlineSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "backoffLimit".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the number of retries before marking this job failed. Defaults to 6".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "backoffLimitPerIndex".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the limit for the number of retries within an index before marking this index as failed. When enabled the number of failures per index is kept in the pod's batch.kubernetes.io/job-index-failure-count annotation. It can only be set when Job's completionMode=Indexed, and the Pod's restart policy is Never. The field is immutable. This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completionMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("completionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.\n\n`NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.\n\n`Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.\n\nMore completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, which is possible during upgrades due to version skew, the controller skips updates for the Job.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the desired number of successfully finished pods the job should be run with.  Setting to null means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "managedBy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ManagedBy field indicates the controller that manages a Job. The k8s Job controller reconciles jobs which don't have this field at all or the field value is the reserved string `kubernetes.io/job-controller`, but skips reconciling Jobs with a custom value for this field. The value must be a valid domain-prefixed path (e.g. acme.io/foo) - all characters before the first \"/\" must be a valid subdomain as defined by RFC 1123. All characters trailing the first \"/\" must be valid HTTP Path characters as defined by RFC 3986. The value cannot exceed 63 characters. This field is immutable.\n\nThis field is beta-level. The job controller accepts setting the field when the feature gate JobManagedBy is enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "manualSelector".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxFailedIndexes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the maximal number of failed indexes before marking the Job as failed, when backoffLimitPerIndex is set. Once the number of failed indexes exceeds this number the entire Job is marked as Failed and its execution is terminated. When left as null the job continues execution of all of its indexes and is marked with the `Complete` Job condition. It can only be specified when backoffLimitPerIndex is set. It can be null or up to completions. It is required and must be less than or equal to 10^4 when is completions greater than 10^5. This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "parallelism".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podFailurePolicy".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::PodFailurePolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the policy of handling failed pods. In particular, it allows to specify the set of actions and conditions which need to be satisfied to take the associated action. If empty, the default behaviour applies - the counter of failed pods, represented by the jobs's .status.failed field, is incremented and it is checked against the backoffLimit. This field cannot be used in combination with restartPolicy=OnFailure.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "podReplacementPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("podReplacementPolicy specifies when to create replacement Pods. Possible values are: - TerminatingOrFailed means that we recreate pods\n  when they are terminating (has a metadata.deletionTimestamp) or failed.\n- Failed means to wait until a previously created Pod is fully terminated (has phase\n  Failed or Succeeded) before creating a replacement Pod.\n\nWhen using podFailurePolicy, Failed is the the only allowed value. TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use. This is an beta field. To use this, enable the JobPodReplacementPolicy feature toggle. This is on by default.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "successPolicy".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::SuccessPolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("successPolicy specifies the policy when the Job can be declared as succeeded. If empty, the default behavior applies - the Job is declared as succeeded only when the number of succeeded pods equals to the completions. When the field is specified, it must be immutable and works only for the Indexed Jobs. Once the Job meets the SuccessPolicy, the lingering pods are terminated.\n\nThis field is beta-level. To use this field, you must enable the `JobSuccessPolicy` feature gate (enabled by default).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "suspend".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to true, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from false to true), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the StartTime field of the Job, effectively resetting the ActiveDeadlineSeconds timer too. Defaults to false.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "template".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Describes the pod that will be created when executing a job. The only allowed template.spec.restartPolicy values are \"Never\" or \"OnFailure\". More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "ttlSecondsAfterFinished".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "template".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
