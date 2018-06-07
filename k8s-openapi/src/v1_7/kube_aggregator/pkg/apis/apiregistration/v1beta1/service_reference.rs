// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.ServiceReference

/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServiceReference {
    /// Name is the name of the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace is the namespace of the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
