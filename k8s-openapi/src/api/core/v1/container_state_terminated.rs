// Generated from definition io.k8s.api.core.v1.ContainerStateTerminated

/// ContainerStateTerminated is a terminated state of a container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerStateTerminated {
    /// Container's ID in the format 'docker://<container_id>'
    #[serde(rename = "containerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Exit status from the last termination of the container
    #[serde(rename = "exitCode")]
    pub exit_code: i32,

    /// Time at which the container last terminated
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Message regarding the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// (brief) reason from the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Signal from the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,

    /// Time at which previous execution of the container started
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<::apimachinery::pkg::apis::meta::v1::Time>,
}
