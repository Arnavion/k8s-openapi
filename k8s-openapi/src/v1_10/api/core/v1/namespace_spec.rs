// Generated from definition io.k8s.api.core.v1.NamespaceSpec

/// NamespaceSpec describes the attributes on a Namespace.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
}
