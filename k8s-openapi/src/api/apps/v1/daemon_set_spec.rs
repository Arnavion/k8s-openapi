// Generated from definition io.k8s.api.apps.v1.DaemonSetSpec

/// DaemonSetSpec is the specification of a daemon set.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DaemonSetSpec {
    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
    #[serde(rename = "minReadySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    #[serde(rename = "revisionHistoryLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,

    /// A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: ::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    pub template: ::api::core::v1::PodTemplateSpec,

    /// An update strategy to replace existing DaemonSet pods with new pods.
    #[serde(rename = "updateStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<::api::apps::v1::DaemonSetUpdateStrategy>,
}
