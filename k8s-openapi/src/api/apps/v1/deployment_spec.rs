// Generated from definition io.k8s.api.apps.v1.DeploymentSpec

/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeploymentSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(rename = "minReadySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// Indicates that the deployment is paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,

    /// The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s.
    #[serde(rename = "progressDeadlineSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,

    /// Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    #[serde(rename = "revisionHistoryLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,

    /// Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. It must match the pod template's labels.
    pub selector: ::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// The deployment strategy to use to replace existing pods with new ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<::api::apps::v1::DeploymentStrategy>,

    /// Template describes the pods that will be created.
    pub template: ::api::core::v1::PodTemplateSpec,
}
