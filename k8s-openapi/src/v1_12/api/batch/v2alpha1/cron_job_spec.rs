// Generated from definition io.k8s.api.batch.v2alpha1.CronJobSpec

/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are: - "Allow" (default): allows CronJobs to run concurrently; - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - "Replace": cancels currently running job and replaces it with a new one
    pub concurrency_policy: Option<String>,

    /// The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    pub failed_jobs_history_limit: Option<i32>,

    /// Specifies the job that will be created when executing a CronJob.
    pub job_template: ::v1_12::api::batch::v2alpha1::JobTemplateSpec,

    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    pub schedule: String,

    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    pub starting_deadline_seconds: Option<i64>,

    /// The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    pub successful_jobs_history_limit: Option<i32>,

    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
    pub suspend: Option<bool>,
}

impl<'de> ::serde::Deserialize<'de> for CronJobSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CronJobSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CronJobSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_concurrency_policy: Option<String> = None;
                let mut value_failed_jobs_history_limit: Option<i32> = None;
                let mut value_job_template: Option<::v1_12::api::batch::v2alpha1::JobTemplateSpec> = None;
                let mut value_schedule: Option<String> = None;
                let mut value_starting_deadline_seconds: Option<i64> = None;
                let mut value_successful_jobs_history_limit: Option<i32> = None;
                let mut value_suspend: Option<bool> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_concurrency_policy => value_concurrency_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed_jobs_history_limit => value_failed_jobs_history_limit = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_job_template => value_job_template = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_schedule => value_schedule = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_starting_deadline_seconds => value_starting_deadline_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_successful_jobs_history_limit => value_successful_jobs_history_limit = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_suspend => value_suspend = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CronJobSpec {
                    concurrency_policy: value_concurrency_policy,
                    failed_jobs_history_limit: value_failed_jobs_history_limit,
                    job_template: value_job_template.ok_or_else(|| ::serde::de::Error::missing_field("jobTemplate"))?,
                    schedule: value_schedule.ok_or_else(|| ::serde::de::Error::missing_field("schedule"))?,
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

impl ::serde::Serialize for CronJobSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CronJobSpec",
            0 +
            self.concurrency_policy.as_ref().map_or(0, |_| 1) +
            self.failed_jobs_history_limit.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.starting_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.successful_jobs_history_limit.as_ref().map_or(0, |_| 1) +
            self.suspend.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.concurrency_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "concurrencyPolicy", value)?;
        }
        if let Some(value) = &self.failed_jobs_history_limit {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "failedJobsHistoryLimit", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "jobTemplate", &self.job_template)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "schedule", &self.schedule)?;
        if let Some(value) = &self.starting_deadline_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "startingDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.successful_jobs_history_limit {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "successfulJobsHistoryLimit", value)?;
        }
        if let Some(value) = &self.suspend {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "suspend", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
