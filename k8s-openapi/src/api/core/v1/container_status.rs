// Generated from definition io.k8s.api.core.v1.ContainerStatus

/// ContainerStatus contains details for the current status of this container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerStatus {
    /// Container's ID in the format 'docker://<container_id>'.
    #[serde(rename = "containerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images
    pub image: String,

    /// ImageID of the container's image.
    #[serde(rename = "imageID")]
    pub image_id: String,

    /// Details about the container's last termination condition.
    #[serde(rename = "lastState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state: Option<::api::core::v1::ContainerState>,

    /// This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.
    pub name: String,

    /// Specifies whether the container has passed its readiness probe.
    pub ready: bool,

    /// The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC.
    #[serde(rename = "restartCount")]
    pub restart_count: i32,

    /// Details about the container's current condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<::api::core::v1::ContainerState>,
}
