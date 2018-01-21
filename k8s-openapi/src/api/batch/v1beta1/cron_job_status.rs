// Generated from definition io.k8s.api.batch.v1beta1.CronJobStatus

/// CronJobStatus represents the current state of a cron job.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<::api::core::v1::ObjectReference>>,

    /// Information when was the last time the job was successfully scheduled.
    #[serde(rename = "lastScheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,
}
