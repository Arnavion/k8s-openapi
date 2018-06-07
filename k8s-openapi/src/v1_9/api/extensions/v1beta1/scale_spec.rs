// Generated from definition io.k8s.api.extensions.v1beta1.ScaleSpec

/// describes the attributes of a scale subresource
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ScaleSpec {
    /// desired number of instances for the scaled object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}
