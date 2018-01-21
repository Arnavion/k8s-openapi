// Generated from definition io.k8s.api.policy.v1beta1.PodDisruptionBudgetSpec

/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at most "maxUnavailable" pods selected by "selector" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with "minAvailable".
    #[serde(rename = "maxUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<::apimachinery::pkg::util::intstr::IntOrString>,

    /// An eviction is allowed if at least "minAvailable" pods selected by "selector" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying "100%".
    #[serde(rename = "minAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_available: Option<::apimachinery::pkg::util::intstr::IntOrString>,

    /// Label query over pods whose evictions are managed by the disruption budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
