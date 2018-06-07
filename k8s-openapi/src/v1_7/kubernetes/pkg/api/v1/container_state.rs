// Generated from definition io.k8s.kubernetes.pkg.api.v1.ContainerState

/// ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerState {
    /// Details about a running container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<::v1_7::kubernetes::pkg::api::v1::ContainerStateRunning>,

    /// Details about a terminated container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<::v1_7::kubernetes::pkg::api::v1::ContainerStateTerminated>,

    /// Details about a waiting container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<::v1_7::kubernetes::pkg::api::v1::ContainerStateWaiting>,
}
