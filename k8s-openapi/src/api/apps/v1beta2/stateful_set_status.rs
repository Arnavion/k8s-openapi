// Generated from definition io.k8s.api.apps.v1beta2.StatefulSetStatus

/// StatefulSetStatus represents the current state of a StatefulSet.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StatefulSetStatus {
    /// collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    #[serde(rename = "collisionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a statefulset's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::apps::v1beta2::StatefulSetCondition>>,

    /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
    #[serde(rename = "currentReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,

    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<String>,

    /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// readyReplicas is the number of Pods created by the StatefulSet controller that have a Ready Condition.
    #[serde(rename = "readyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,

    /// replicas is the number of Pods created by the StatefulSet controller.
    pub replicas: i32,

    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)
    #[serde(rename = "updateRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_revision: Option<String>,

    /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
    #[serde(rename = "updatedReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}
