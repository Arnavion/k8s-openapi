// Generated from definition io.k8s.api.core.v1.NodeSpec

/// NodeSpec describes the attributes that a node is created with.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeSpec {
    /// If specified, the source to get node configuration from The DynamicKubeletConfig feature gate must be enabled for the Kubelet to use this field
    #[serde(rename = "configSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_source: Option<::api::core::v1::NodeConfigSource>,

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
    pub taints: Option<Vec<::api::core::v1::Taint>>,

    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
}
