// Generated from definition io.k8s.api.core.v1.ContainerStateRunning

/// ContainerStateRunning is a running state of a container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerStateRunning {
    /// Time at which the container was last (re-)started
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<::apimachinery::pkg::apis::meta::v1::Time>,
}
