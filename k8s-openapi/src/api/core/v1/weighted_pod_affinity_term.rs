// Generated from definition io.k8s.api.core.v1.WeightedPodAffinityTerm

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WeightedPodAffinityTerm {
    /// Required. A pod affinity term, associated with the corresponding weight.
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: ::api::core::v1::PodAffinityTerm,

    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    pub weight: i32,
}
