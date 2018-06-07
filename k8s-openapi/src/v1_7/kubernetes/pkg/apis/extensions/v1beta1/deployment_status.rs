// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.DeploymentStatus

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeploymentStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    #[serde(rename = "availableReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,

    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    #[serde(rename = "collisionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i64>,

    /// Represents the latest available observations of a deployment's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DeploymentCondition>>,

    /// The generation observed by the deployment controller.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Total number of ready pods targeted by this deployment.
    #[serde(rename = "readyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Total number of unavailable pods targeted by this deployment.
    #[serde(rename = "unavailableReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    #[serde(rename = "updatedReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}
