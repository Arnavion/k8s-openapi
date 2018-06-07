// Generated from definition io.k8s.kubernetes.pkg.api.v1.NodeSpec

/// NodeSpec describes the attributes that a node is created with.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeSpec {
    /// External ID of the node assigned by some machine database (e.g. a cloud provider). Deprecated.
    #[serde(rename = "externalID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    /// PodCIDR represents the pod IP range assigned to the node.
    #[serde(rename = "podCIDR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,

    /// ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>
    #[serde(rename = "providerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// If specified, the node's taints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<::v1_7::kubernetes::pkg::api::v1::Taint>>,

    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
}
