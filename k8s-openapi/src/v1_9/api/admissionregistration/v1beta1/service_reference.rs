// Generated from definition io.k8s.api.admissionregistration.v1beta1.ServiceReference

/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServiceReference {
    /// `name` is the name of the service. Required
    pub name: String,

    /// `namespace` is the namespace of the service. Required
    pub namespace: String,

    /// `path` is an optional URL path which will be sent in any request to this service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
