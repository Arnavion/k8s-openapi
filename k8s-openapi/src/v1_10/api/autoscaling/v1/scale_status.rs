// Generated from definition io.k8s.api.autoscaling.v1.ScaleStatus

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    pub replicas: i32,

    /// label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}
