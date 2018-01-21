// Generated from definition io.k8s.api.core.v1.NodeStatus

/// NodeStatus is information about the current status of a node.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeStatus {
    /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<::api::core::v1::NodeAddress>>,

    /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::core::v1::NodeCondition>>,

    /// Endpoints of daemons running on the Node.
    #[serde(rename = "daemonEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_endpoints: Option<::api::core::v1::NodeDaemonEndpoints>,

    /// List of container images on this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<::api::core::v1::ContainerImage>>,

    /// Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#info
    #[serde(rename = "nodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info: Option<::api::core::v1::NodeSystemInfo>,

    /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// List of volumes that are attached to the node.
    #[serde(rename = "volumesAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_attached: Option<Vec<::api::core::v1::AttachedVolume>>,

    /// List of attachable volumes in use (mounted) by the node.
    #[serde(rename = "volumesInUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_in_use: Option<Vec<String>>,
}
