// Generated from definition io.k8s.api.core.v1.Lifecycle

/// Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(rename = "postStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_start: Option<::api::core::v1::Handler>,

    /// PreStop is called immediately before a container is terminated. The container is terminated after the handler completes. The reason for termination is passed to the handler. Regardless of the outcome of the handler, the container is eventually terminated. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(rename = "preStop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<::api::core::v1::Handler>,
}
