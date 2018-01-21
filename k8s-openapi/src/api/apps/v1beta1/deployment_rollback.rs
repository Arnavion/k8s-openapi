// Generated from definition io.k8s.api.apps.v1beta1.DeploymentRollback

/// DEPRECATED. DeploymentRollback stores the information required to rollback a deployment.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeploymentRollback {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Required: This must match the Name of a deployment.
    pub name: String,

    /// The config of this deployment rollback.
    #[serde(rename = "rollbackTo")]
    pub rollback_to: ::api::apps::v1beta1::RollbackConfig,

    /// The annotations to be updated to a deployment
    #[serde(rename = "updatedAnnotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_annotations: Option<::std::collections::BTreeMap<String, String>>,
}
