// Generated from definition io.k8s.api.core.v1.ResourceFieldSelector

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisor: Option<::apimachinery::pkg::api::resource::Quantity>,

    /// Required: resource to select
    pub resource: String,
}
