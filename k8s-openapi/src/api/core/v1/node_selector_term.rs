// Generated from definition io.k8s.api.core.v1.NodeSelectorTerm

/// A null or empty node selector term matches no objects.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeSelectorTerm {
    /// Required. A list of node selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    pub match_expressions: Vec<::api::core::v1::NodeSelectorRequirement>,
}
