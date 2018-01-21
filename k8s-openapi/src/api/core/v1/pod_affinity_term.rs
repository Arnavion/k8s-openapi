// Generated from definition io.k8s.api.core.v1.PodAffinityTerm

/// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    #[serde(rename = "labelSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// namespaces specifies which namespaces the labelSelector applies to (matches against); null or empty list means "this pod's namespace"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,

    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}
