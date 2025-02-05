// Generated from definition io.k8s.api.batch.v1.JobStatus

/// JobStatus represents the current state of a Job.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobStatus {
    /// The number of pending and running pods which are not terminating (without a deletionTimestamp). The value is zero for finished jobs.
    pub active: Option<i32>,

    /// completedIndexes holds the completed indexes when .spec.completionMode = "Indexed" in a text format. The indexes are represented as decimal integers separated by commas. The numbers are listed in increasing order. Three or more consecutive numbers are compressed and represented by the first and last element of the series, separated by a hyphen. For example, if the completed indexes are 1, 3, 4, 5 and 7, they are represented as "1,3-5,7".
    pub completed_indexes: Option<std::string::String>,

    /// Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC. The completion time is set when the job finishes successfully, and only then. The value cannot be updated or removed. The value indicates the same or later point in time as the startTime field.
    pub completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// The latest available observations of an object's current state. When a Job fails, one of the conditions will have type "Failed" and status true. When a Job is suspended, one of the conditions will have type "Suspended" and status true; when the Job is resumed, the status of this condition will become false. When a Job is completed, one of the conditions will have type "Complete" and status true.
    ///
    /// A job is considered finished when it is in a terminal condition, either "Complete" or "Failed". A Job cannot have both the "Complete" and "Failed" conditions. Additionally, it cannot be in the "Complete" and "FailureTarget" conditions. The "Complete", "Failed" and "FailureTarget" conditions cannot be disabled.
    ///
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub conditions: Option<std::vec::Vec<crate::api::batch::v1::JobCondition>>,

    /// The number of pods which reached phase Failed. The value increases monotonically.
    pub failed: Option<i32>,

    /// FailedIndexes holds the failed indexes when spec.backoffLimitPerIndex is set. The indexes are represented in the text format analogous as for the `completedIndexes` field, ie. they are kept as decimal integers separated by commas. The numbers are listed in increasing order. Three or more consecutive numbers are compressed and represented by the first and last element of the series, separated by a hyphen. For example, if the failed indexes are 1, 3, 4, 5 and 7, they are represented as "1,3-5,7". The set of failed indexes cannot overlap with the set of completed indexes.
    ///
    /// This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).
    pub failed_indexes: Option<std::string::String>,

    /// The number of active pods which have a Ready condition and are not terminating (without a deletionTimestamp).
    pub ready: Option<i32>,

    /// Represents time when the job controller started processing a job. When a Job is created in the suspended state, this field is not set until the first time it is resumed. This field is reset every time a Job is resumed from suspension. It is represented in RFC3339 form and is in UTC.
    ///
    /// Once set, the field can only be removed when the job is suspended. The field cannot be modified while the job is unsuspended or finished.
    pub start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// The number of pods which reached phase Succeeded. The value increases monotonically for a given spec. However, it may decrease in reaction to scale down of elastic indexed jobs.
    pub succeeded: Option<i32>,

    /// The number of pods which are terminating (in phase Pending or Running and have a deletionTimestamp).
    ///
    /// This field is beta-level. The job controller populates the field when the feature gate JobPodReplacementPolicy is enabled (enabled by default).
    pub terminating: Option<i32>,

    /// uncountedTerminatedPods holds the UIDs of Pods that have terminated but the job controller hasn't yet accounted for in the status counters.
    ///
    /// The job controller creates pods with a finalizer. When a pod terminates (succeeded or failed), the controller does three steps to account for it in the job status:
    ///
    /// 1. Add the pod UID to the arrays in this field. 2. Remove the pod finalizer. 3. Remove the pod UID from the arrays while increasing the corresponding
    ///     counter.
    ///
    /// Old jobs might not be tracked using this field, in which case the field remains null. The structure is empty for finished jobs.
    pub uncounted_terminated_pods: Option<crate::api::batch::v1::UncountedTerminatedPods>,
}

impl crate::DeepMerge for JobStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.active, other.active);
        crate::DeepMerge::merge_from(&mut self.completed_indexes, other.completed_indexes);
        crate::DeepMerge::merge_from(&mut self.completion_time, other.completion_time);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.failed, other.failed);
        crate::DeepMerge::merge_from(&mut self.failed_indexes, other.failed_indexes);
        crate::DeepMerge::merge_from(&mut self.ready, other.ready);
        crate::DeepMerge::merge_from(&mut self.start_time, other.start_time);
        crate::DeepMerge::merge_from(&mut self.succeeded, other.succeeded);
        crate::DeepMerge::merge_from(&mut self.terminating, other.terminating);
        crate::DeepMerge::merge_from(&mut self.uncounted_terminated_pods, other.uncounted_terminated_pods);
    }
}

impl<'de> crate::serde::Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active,
            Key_completed_indexes,
            Key_completion_time,
            Key_conditions,
            Key_failed,
            Key_failed_indexes,
            Key_ready,
            Key_start_time,
            Key_succeeded,
            Key_terminating,
            Key_uncounted_terminated_pods,
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
                            "active" => Field::Key_active,
                            "completedIndexes" => Field::Key_completed_indexes,
                            "completionTime" => Field::Key_completion_time,
                            "conditions" => Field::Key_conditions,
                            "failed" => Field::Key_failed,
                            "failedIndexes" => Field::Key_failed_indexes,
                            "ready" => Field::Key_ready,
                            "startTime" => Field::Key_start_time,
                            "succeeded" => Field::Key_succeeded,
                            "terminating" => Field::Key_terminating,
                            "uncountedTerminatedPods" => Field::Key_uncounted_terminated_pods,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JobStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JobStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active: Option<i32> = None;
                let mut value_completed_indexes: Option<std::string::String> = None;
                let mut value_completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::batch::v1::JobCondition>> = None;
                let mut value_failed: Option<i32> = None;
                let mut value_failed_indexes: Option<std::string::String> = None;
                let mut value_ready: Option<i32> = None;
                let mut value_start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_succeeded: Option<i32> = None;
                let mut value_terminating: Option<i32> = None;
                let mut value_uncounted_terminated_pods: Option<crate::api::batch::v1::UncountedTerminatedPods> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active => value_active = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completed_indexes => value_completed_indexes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completion_time => value_completion_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed => value_failed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed_indexes => value_failed_indexes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready => value_ready = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_succeeded => value_succeeded = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_terminating => value_terminating = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uncounted_terminated_pods => value_uncounted_terminated_pods = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JobStatus {
                    active: value_active,
                    completed_indexes: value_completed_indexes,
                    completion_time: value_completion_time,
                    conditions: value_conditions,
                    failed: value_failed,
                    failed_indexes: value_failed_indexes,
                    ready: value_ready,
                    start_time: value_start_time,
                    succeeded: value_succeeded,
                    terminating: value_terminating,
                    uncounted_terminated_pods: value_uncounted_terminated_pods,
                })
            }
        }

        deserializer.deserialize_struct(
            "JobStatus",
            &[
                "active",
                "completedIndexes",
                "completionTime",
                "conditions",
                "failed",
                "failedIndexes",
                "ready",
                "startTime",
                "succeeded",
                "terminating",
                "uncountedTerminatedPods",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JobStatus",
            self.active.as_ref().map_or(0, |_| 1) +
            self.completed_indexes.as_ref().map_or(0, |_| 1) +
            self.completion_time.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.failed.as_ref().map_or(0, |_| 1) +
            self.failed_indexes.as_ref().map_or(0, |_| 1) +
            self.ready.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1) +
            self.succeeded.as_ref().map_or(0, |_| 1) +
            self.terminating.as_ref().map_or(0, |_| 1) +
            self.uncounted_terminated_pods.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "active", value)?;
        }
        if let Some(value) = &self.completed_indexes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completedIndexes", value)?;
        }
        if let Some(value) = &self.completion_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completionTime", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.failed {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failed", value)?;
        }
        if let Some(value) = &self.failed_indexes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failedIndexes", value)?;
        }
        if let Some(value) = &self.ready {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ready", value)?;
        }
        if let Some(value) = &self.start_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        if let Some(value) = &self.succeeded {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "succeeded", value)?;
        }
        if let Some(value) = &self.terminating {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminating", value)?;
        }
        if let Some(value) = &self.uncounted_terminated_pods {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uncountedTerminatedPods", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JobStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.batch.v1.JobStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("JobStatus represents the current state of a Job.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "active".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pending and running pods which are not terminating (without a deletionTimestamp). The value is zero for finished jobs.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completedIndexes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("completedIndexes holds the completed indexes when .spec.completionMode = \"Indexed\" in a text format. The indexes are represented as decimal integers separated by commas. The numbers are listed in increasing order. Three or more consecutive numbers are compressed and represented by the first and last element of the series, separated by a hyphen. For example, if the completed indexes are 1, 3, 4, 5 and 7, they are represented as \"1,3-5,7\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC. The completion time is set when the job finishes successfully, and only then. The value cannot be updated or removed. The value indicates the same or later point in time as the startTime field.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "conditions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The latest available observations of an object's current state. When a Job fails, one of the conditions will have type \"Failed\" and status true. When a Job is suspended, one of the conditions will have type \"Suspended\" and status true; when the Job is resumed, the status of this condition will become false. When a Job is completed, one of the conditions will have type \"Complete\" and status true.\n\nA job is considered finished when it is in a terminal condition, either \"Complete\" or \"Failed\". A Job cannot have both the \"Complete\" and \"Failed\" conditions. Additionally, it cannot be in the \"Complete\" and \"FailureTarget\" conditions. The \"Complete\", \"Failed\" and \"FailureTarget\" conditions cannot be disabled.\n\nMore info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::batch::v1::JobCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "failed".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pods which reached phase Failed. The value increases monotonically.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "failedIndexes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("FailedIndexes holds the failed indexes when spec.backoffLimitPerIndex is set. The indexes are represented in the text format analogous as for the `completedIndexes` field, ie. they are kept as decimal integers separated by commas. The numbers are listed in increasing order. Three or more consecutive numbers are compressed and represented by the first and last element of the series, separated by a hyphen. For example, if the failed indexes are 1, 3, 4, 5 and 7, they are represented as \"1,3-5,7\". The set of failed indexes cannot overlap with the set of completed indexes.\n\nThis field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ready".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of active pods which have a Ready condition and are not terminating (without a deletionTimestamp).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents time when the job controller started processing a job. When a Job is created in the suspended state, this field is not set until the first time it is resumed. This field is reset every time a Job is resumed from suspension. It is represented in RFC3339 form and is in UTC.\n\nOnce set, the field can only be removed when the job is suspended. The field cannot be modified while the job is unsuspended or finished.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "succeeded".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pods which reached phase Succeeded. The value increases monotonically for a given spec. However, it may decrease in reaction to scale down of elastic indexed jobs.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "terminating".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pods which are terminating (in phase Pending or Running and have a deletionTimestamp).\n\nThis field is beta-level. The job controller populates the field when the feature gate JobPodReplacementPolicy is enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uncountedTerminatedPods".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::UncountedTerminatedPods>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("uncountedTerminatedPods holds the UIDs of Pods that have terminated but the job controller hasn't yet accounted for in the status counters.\n\nThe job controller creates pods with a finalizer. When a pod terminates (succeeded or failed), the controller does three steps to account for it in the job status:\n\n1. Add the pod UID to the arrays in this field. 2. Remove the pod finalizer. 3. Remove the pod UID from the arrays while increasing the corresponding\n    counter.\n\nOld jobs might not be tracked using this field, in which case the field remains null. The structure is empty for finished jobs.".into()),
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
