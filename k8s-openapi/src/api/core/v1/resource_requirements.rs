// Generated from definition io.k8s.api.core.v1.ResourceRequirements

/// ResourceRequirements describes the compute resource requirements.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,
}
