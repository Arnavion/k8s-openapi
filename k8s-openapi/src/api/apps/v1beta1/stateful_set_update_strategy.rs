// Generated from definition io.k8s.api.apps.v1beta1.StatefulSetUpdateStrategy

/// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StatefulSetUpdateStrategy {
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    #[serde(rename = "rollingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<::api::apps::v1beta1::RollingUpdateStatefulSetStrategy>,

    /// Type indicates the type of the StatefulSetUpdateStrategy.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
