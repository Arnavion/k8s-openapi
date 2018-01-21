// Generated from definition io.k8s.api.batch.v2alpha1.CronJobSpec

/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are: - "Allow" (default): allows CronJobs to run concurrently; - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - "Replace": cancels currently running job and replaces it with a new one
    #[serde(rename = "concurrencyPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_policy: Option<String>,

    /// The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    #[serde(rename = "failedJobsHistoryLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,

    /// Specifies the job that will be created when executing a CronJob.
    #[serde(rename = "jobTemplate")]
    pub job_template: ::api::batch::v2alpha1::JobTemplateSpec,

    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    pub schedule: String,

    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    #[serde(rename = "startingDeadlineSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,

    /// The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    #[serde(rename = "successfulJobsHistoryLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,

    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}
