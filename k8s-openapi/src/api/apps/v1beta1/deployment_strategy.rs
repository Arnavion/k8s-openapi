// Generated from definition io.k8s.api.apps.v1beta1.DeploymentStrategy

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeploymentStrategy {
    /// Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
    #[serde(rename = "rollingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<::api::apps::v1beta1::RollingUpdateDeployment>,

    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
