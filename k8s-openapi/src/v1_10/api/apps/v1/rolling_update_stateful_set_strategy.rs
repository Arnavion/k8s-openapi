// Generated from definition io.k8s.api.apps.v1.RollingUpdateStatefulSetStrategy

/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be partitioned. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
}
