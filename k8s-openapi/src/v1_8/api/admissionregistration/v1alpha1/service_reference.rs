// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ServiceReference

/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServiceReference {
    /// Name is the name of the service Required
    pub name: String,

    /// Namespace is the namespace of the service Required
    pub namespace: String,
}
