// Generated from definition io.k8s.api.apps.v1beta1.ScaleStatus

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    pub replicas: i32,

    /// label query over pods that should match the replicas count. More info: http://kubernetes.io/docs/user-guide/labels#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<::std::collections::BTreeMap<String, String>>,

    /// label selector for pods that should match the replicas count. This is a serializated version of both map-based and more expressive set-based selectors. This is done to avoid introspection in the clients. The string will be in the same format as the query-param syntax. If the target type only supports map-based selectors, both this field and map-based selector field are populated. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(rename = "targetSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selector: Option<String>,
}
