// Generated from definition io.k8s.api.batch.v1.CronJobSpec

/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are:
    ///
    /// - "Allow" (default): allows CronJobs to run concurrently; - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - "Replace": cancels currently running job and replaces it with a new one
    pub concurrency_policy: Option<std::string::String>,

    /// The number of failed finished jobs to retain. Value must be non-negative integer. Defaults to 1.
    pub failed_jobs_history_limit: Option<i32>,

    /// Specifies the job that will be created when executing a CronJob.
    pub job_template: crate::api::batch::v1::JobTemplateSpec,

    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    pub schedule: std::string::String,

    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    pub starting_deadline_seconds: Option<i64>,

    /// The number of successful finished jobs to retain. Value must be non-negative integer. Defaults to 3.
    pub successful_jobs_history_limit: Option<i32>,

    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
    pub suspend: Option<bool>,

    /// The time zone name for the given schedule, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. If not specified, this will default to the time zone of the kube-controller-manager process. The set of valid time zone names and the time zone offset is loaded from the system-wide time zone database by the API server during CronJob validation and the controller manager during execution. If no system-wide time zone database can be found a bundled version of the database is used instead. If the time zone name becomes invalid during the lifetime of a CronJob or due to a change in host configuration, the controller will stop creating new new Jobs and will create a system event with the reason UnknownTimeZone. More information can be found in https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#time-zones
    pub time_zone: Option<std::string::String>,
}

impl crate::DeepMerge for CronJobSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.concurrency_policy, other.concurrency_policy);
        crate::DeepMerge::merge_from(&mut self.failed_jobs_history_limit, other.failed_jobs_history_limit);
        crate::DeepMerge::merge_from(&mut self.job_template, other.job_template);
        crate::DeepMerge::merge_from(&mut self.schedule, other.schedule);
        crate::DeepMerge::merge_from(&mut self.starting_deadline_seconds, other.starting_deadline_seconds);
        crate::DeepMerge::merge_from(&mut self.successful_jobs_history_limit, other.successful_jobs_history_limit);
        crate::DeepMerge::merge_from(&mut self.suspend, other.suspend);
        crate::DeepMerge::merge_from(&mut self.time_zone, other.time_zone);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CronJobSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_concurrency_policy,
            Key_failed_jobs_history_limit,
            Key_job_template,
            Key_schedule,
            Key_starting_deadline_seconds,
            Key_successful_jobs_history_limit,
            Key_suspend,
            Key_time_zone,
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
                            "concurrencyPolicy" => Field::Key_concurrency_policy,
                            "failedJobsHistoryLimit" => Field::Key_failed_jobs_history_limit,
                            "jobTemplate" => Field::Key_job_template,
                            "schedule" => Field::Key_schedule,
                            "startingDeadlineSeconds" => Field::Key_starting_deadline_seconds,
                            "successfulJobsHistoryLimit" => Field::Key_successful_jobs_history_limit,
                            "suspend" => Field::Key_suspend,
                            "timeZone" => Field::Key_time_zone,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CronJobSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CronJobSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_concurrency_policy: Option<std::string::String> = None;
                let mut value_failed_jobs_history_limit: Option<i32> = None;
                let mut value_job_template: Option<crate::api::batch::v1::JobTemplateSpec> = None;
                let mut value_schedule: Option<std::string::String> = None;
                let mut value_starting_deadline_seconds: Option<i64> = None;
                let mut value_successful_jobs_history_limit: Option<i32> = None;
                let mut value_suspend: Option<bool> = None;
                let mut value_time_zone: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_concurrency_policy => value_concurrency_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed_jobs_history_limit => value_failed_jobs_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_job_template => value_job_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_schedule => value_schedule = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_starting_deadline_seconds => value_starting_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_successful_jobs_history_limit => value_successful_jobs_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_suspend => value_suspend = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time_zone => value_time_zone = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CronJobSpec {
                    concurrency_policy: value_concurrency_policy,
                    failed_jobs_history_limit: value_failed_jobs_history_limit,
                    job_template: value_job_template.unwrap_or_default(),
                    schedule: value_schedule.unwrap_or_default(),
                    starting_deadline_seconds: value_starting_deadline_seconds,
                    successful_jobs_history_limit: value_successful_jobs_history_limit,
                    suspend: value_suspend,
                    time_zone: value_time_zone,
                })
            }
        }

        deserializer.deserialize_struct(
            "CronJobSpec",
            &[
                "concurrencyPolicy",
                "failedJobsHistoryLimit",
                "jobTemplate",
                "schedule",
                "startingDeadlineSeconds",
                "successfulJobsHistoryLimit",
                "suspend",
                "timeZone",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CronJobSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CronJobSpec",
            2 +
            self.concurrency_policy.as_ref().map_or(0, |_| 1) +
            self.failed_jobs_history_limit.as_ref().map_or(0, |_| 1) +
            self.starting_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.successful_jobs_history_limit.as_ref().map_or(0, |_| 1) +
            self.suspend.as_ref().map_or(0, |_| 1) +
            self.time_zone.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.concurrency_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "concurrencyPolicy", value)?;
        }
        if let Some(value) = &self.failed_jobs_history_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failedJobsHistoryLimit", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "jobTemplate", &self.job_template)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedule", &self.schedule)?;
        if let Some(value) = &self.starting_deadline_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startingDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.successful_jobs_history_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "successfulJobsHistoryLimit", value)?;
        }
        if let Some(value) = &self.suspend {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "suspend", value)?;
        }
        if let Some(value) = &self.time_zone {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeZone", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CronJobSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.batch.v1.CronJobSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("CronJobSpec describes how the job execution will look like and when it will actually run.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "concurrencyPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies how to treat concurrent executions of a Job. Valid values are:\n\n- \"Allow\" (default): allows CronJobs to run concurrently; - \"Forbid\": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - \"Replace\": cancels currently running job and replaces it with a new one".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "failedJobsHistoryLimit".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of failed finished jobs to retain. Value must be non-negative integer. Defaults to 1.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "jobTemplate".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::JobTemplateSpec>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the job that will be created when executing a CronJob.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "schedule".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startingDeadlineSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "successfulJobsHistoryLimit".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of successful finished jobs to retain. Value must be non-negative integer. Defaults to 3.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "suspend".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "timeZone".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The time zone name for the given schedule, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. If not specified, this will default to the time zone of the kube-controller-manager process. The set of valid time zone names and the time zone offset is loaded from the system-wide time zone database by the API server during CronJob validation and the controller manager during execution. If no system-wide time zone database can be found a bundled version of the database is used instead. If the time zone name becomes invalid during the lifetime of a CronJob or due to a change in host configuration, the controller will stop creating new new Jobs and will create a system event with the reason UnknownTimeZone. More information can be found in https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#time-zones".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "jobTemplate".into(),
                    "schedule".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
