// Generated from definition io.k8s.api.batch.v1.JobStatus

/// JobStatus represents the current state of a Job.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JobStatus {
    /// The number of actively running pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,

    /// Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    #[serde(rename = "completionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// The latest available observations of an object's current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::batch::v1::JobCondition>>,

    /// The number of pods which reached phase Failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,

    /// Represents time when the job was acknowledged by the job controller. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// The number of pods which reached phase Succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
}
