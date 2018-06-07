// Generated from definition io.k8s.api.core.v1.NamespaceStatus

/// NamespaceStatus is information about the current status of a Namespace.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NamespaceStatus {
    /// Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}
