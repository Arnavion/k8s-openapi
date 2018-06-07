// Generated from definition io.k8s.api.core.v1.ContainerStateWaiting

/// ContainerStateWaiting is a waiting state of a container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerStateWaiting {
    /// Message regarding why the container is not yet running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// (brief) reason the container is not yet running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
