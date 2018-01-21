// Generated from definition io.k8s.api.core.v1.ReplicationControllerSpec

/// ReplicationControllerSpec is the specification of a replication controller.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ReplicationControllerSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(rename = "minReadySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Selector is a label query over pods that should match the Replicas count. If Selector is empty, it is defaulted to the labels present on the Pod template. Label keys and values that must match in order to be controlled by this replication controller, if empty defaulted to labels on Pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<::std::collections::BTreeMap<String, String>>,

    /// Template is the object that describes the pod that will be created if insufficient replicas are detected. This takes precedence over a TemplateRef. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<::api::core::v1::PodTemplateSpec>,
}
