// Generated from definition io.k8s.api.batch.v1beta1.CronJobSpec

/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are: - "Allow" (default): allows CronJobs to run concurrently; - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - "Replace": cancels currently running job and replaces it with a new one
    pub concurrency_policy: Option<String>,

    /// The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
    pub failed_jobs_history_limit: Option<i32>,

    /// Specifies the job that will be created when executing a CronJob.
    pub job_template: crate::api::batch::v1beta1::JobTemplateSpec,

    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    pub schedule: String,

    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    pub starting_deadline_seconds: Option<i64>,

    /// The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. Defaults to 3.
    pub successful_jobs_history_limit: Option<i32>,

    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
    pub suspend: Option<bool>,

}

#[cfg(feature = "dsl")]
impl CronJobSpec  {
    /// Set [`Self::concurrency_policy`]
    pub  fn concurrency_policy_set(&mut self, concurrency_policy: impl Into<Option<String>>) -> &mut Self {
        self.concurrency_policy = concurrency_policy.into(); self
    }

    pub  fn concurrency_policy(&mut self) -> &mut String {
        if self.concurrency_policy.is_none() { self.concurrency_policy = Some(Default::default()) }
        self.concurrency_policy.as_mut().unwrap()
    }

    /// Modify [`Self::concurrency_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn concurrency_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.concurrency_policy.is_none() { self.concurrency_policy = Some(Default::default()) };
        func(self.concurrency_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::failed_jobs_history_limit`]
    pub  fn failed_jobs_history_limit_set(&mut self, failed_jobs_history_limit: impl Into<Option<i32>>) -> &mut Self {
        self.failed_jobs_history_limit = failed_jobs_history_limit.into(); self
    }

    pub  fn failed_jobs_history_limit(&mut self) -> &mut i32 {
        if self.failed_jobs_history_limit.is_none() { self.failed_jobs_history_limit = Some(Default::default()) }
        self.failed_jobs_history_limit.as_mut().unwrap()
    }

    /// Modify [`Self::failed_jobs_history_limit`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn failed_jobs_history_limit_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.failed_jobs_history_limit.is_none() { self.failed_jobs_history_limit = Some(Default::default()) };
        func(self.failed_jobs_history_limit.as_mut().unwrap()); self
    }


    /// Set [`Self::job_template`]
    pub  fn job_template_set(&mut self, job_template: impl Into<crate::api::batch::v1beta1::JobTemplateSpec>) -> &mut Self {
        self.job_template = job_template.into(); self
    }

    pub  fn job_template(&mut self) -> &mut crate::api::batch::v1beta1::JobTemplateSpec {
        &mut self.job_template
    }

    /// Modify [`Self::job_template`] with a `func`
    pub  fn job_template_with(&mut self, func: impl FnOnce(&mut crate::api::batch::v1beta1::JobTemplateSpec)) -> &mut Self {
        func(&mut self.job_template); self
    }


    /// Set [`Self::schedule`]
    pub  fn schedule_set(&mut self, schedule: impl Into<String>) -> &mut Self {
        self.schedule = schedule.into(); self
    }

    pub  fn schedule(&mut self) -> &mut String {
        &mut self.schedule
    }

    /// Modify [`Self::schedule`] with a `func`
    pub  fn schedule_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.schedule); self
    }


    /// Set [`Self::starting_deadline_seconds`]
    pub  fn starting_deadline_seconds_set(&mut self, starting_deadline_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.starting_deadline_seconds = starting_deadline_seconds.into(); self
    }

    pub  fn starting_deadline_seconds(&mut self) -> &mut i64 {
        if self.starting_deadline_seconds.is_none() { self.starting_deadline_seconds = Some(Default::default()) }
        self.starting_deadline_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::starting_deadline_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn starting_deadline_seconds_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.starting_deadline_seconds.is_none() { self.starting_deadline_seconds = Some(Default::default()) };
        func(self.starting_deadline_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::successful_jobs_history_limit`]
    pub  fn successful_jobs_history_limit_set(&mut self, successful_jobs_history_limit: impl Into<Option<i32>>) -> &mut Self {
        self.successful_jobs_history_limit = successful_jobs_history_limit.into(); self
    }

    pub  fn successful_jobs_history_limit(&mut self) -> &mut i32 {
        if self.successful_jobs_history_limit.is_none() { self.successful_jobs_history_limit = Some(Default::default()) }
        self.successful_jobs_history_limit.as_mut().unwrap()
    }

    /// Modify [`Self::successful_jobs_history_limit`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn successful_jobs_history_limit_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.successful_jobs_history_limit.is_none() { self.successful_jobs_history_limit = Some(Default::default()) };
        func(self.successful_jobs_history_limit.as_mut().unwrap()); self
    }


    /// Set [`Self::suspend`]
    pub  fn suspend_set(&mut self, suspend: impl Into<Option<bool>>) -> &mut Self {
        self.suspend = suspend.into(); self
    }

    pub  fn suspend(&mut self) -> &mut bool {
        if self.suspend.is_none() { self.suspend = Some(Default::default()) }
        self.suspend.as_mut().unwrap()
    }

    /// Modify [`Self::suspend`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn suspend_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.suspend.is_none() { self.suspend = Some(Default::default()) };
        func(self.suspend.as_mut().unwrap()); self
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
                            "concurrencyPolicy" => Field::Key_concurrency_policy,
                            "failedJobsHistoryLimit" => Field::Key_failed_jobs_history_limit,
                            "jobTemplate" => Field::Key_job_template,
                            "schedule" => Field::Key_schedule,
                            "startingDeadlineSeconds" => Field::Key_starting_deadline_seconds,
                            "successfulJobsHistoryLimit" => Field::Key_successful_jobs_history_limit,
                            "suspend" => Field::Key_suspend,
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CronJobSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_concurrency_policy: Option<String> = None;
                let mut value_failed_jobs_history_limit: Option<i32> = None;
                let mut value_job_template: Option<crate::api::batch::v1beta1::JobTemplateSpec> = None;
                let mut value_schedule: Option<String> = None;
                let mut value_starting_deadline_seconds: Option<i64> = None;
                let mut value_successful_jobs_history_limit: Option<i32> = None;
                let mut value_suspend: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_concurrency_policy => value_concurrency_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed_jobs_history_limit => value_failed_jobs_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_job_template => value_job_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_schedule => value_schedule = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_starting_deadline_seconds => value_starting_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_successful_jobs_history_limit => value_successful_jobs_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_suspend => value_suspend = crate::serde::de::MapAccess::next_value(&mut map)?,
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
            self.suspend.as_ref().map_or(0, |_| 1),
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
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CronJobSpec {
    fn schema_name() -> String {
        "io.k8s.api.batch.v1beta1.CronJobSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CronJobSpec describes how the job execution will look like and when it will actually run.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "concurrencyPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies how to treat concurrent executions of a Job. Valid values are: - \"Allow\" (default): allows CronJobs to run concurrently; - \"Forbid\": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - \"Replace\": cancels currently running job and replaces it with a new one".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "failedJobsHistoryLimit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "jobTemplate".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1beta1::JobTemplateSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the job that will be created when executing a CronJob.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "schedule".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startingDeadlineSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "successfulJobsHistoryLimit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. Defaults to 3.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "suspend".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "jobTemplate".to_owned(),
                    "schedule".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
