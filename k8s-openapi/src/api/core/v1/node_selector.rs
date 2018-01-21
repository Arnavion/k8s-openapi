// Generated from definition io.k8s.api.core.v1.NodeSelector

/// A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<::api::core::v1::NodeSelectorTerm>,
}
