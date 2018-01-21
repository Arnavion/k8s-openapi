// Generated from definition io.k8s.api.policy.v1beta1.PodDisruptionBudgetStatus

/// PodDisruptionBudgetStatus represents information about the status of a PodDisruptionBudget. Status may trail the actual state of a system.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodDisruptionBudgetStatus {
    /// current number of healthy pods
    #[serde(rename = "currentHealthy")]
    pub current_healthy: i32,

    /// minimum desired number of healthy pods
    #[serde(rename = "desiredHealthy")]
    pub desired_healthy: i32,

    /// DisruptedPods contains information about pods whose eviction was processed by the API server eviction subresource handler but has not yet been observed by the PodDisruptionBudget controller. A pod will be in this map from the time when the API server processed the eviction request to the time when the pod is seen by PDB controller as having been marked for deletion (or after a timeout). The key in the map is the name of the pod and the value is the time when the API server processed the eviction request. If the deletion didn't occur and a pod is still there it will be removed from the list automatically by PodDisruptionBudget controller after some time. If everything goes smooth this map should be empty for the most of the time. Large number of entries in the map may indicate problems with pod deletions.
    #[serde(rename = "disruptedPods")]
    pub disrupted_pods: ::std::collections::BTreeMap<String, ::apimachinery::pkg::apis::meta::v1::Time>,

    /// Number of pod disruptions that are currently allowed.
    #[serde(rename = "disruptionsAllowed")]
    pub disruptions_allowed: i32,

    /// total number of pods counted by this disruption budget
    #[serde(rename = "expectedPods")]
    pub expected_pods: i32,

    /// Most recent generation observed when updating this PDB status. PodDisruptionsAllowed and other status informatio is valid only if observedGeneration equals to PDB's object generation.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}
