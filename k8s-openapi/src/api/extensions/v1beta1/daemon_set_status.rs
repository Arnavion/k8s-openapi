// Generated from definition io.k8s.api.extensions.v1beta1.DaemonSetStatus

/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DaemonSetStatus {
    /// Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    #[serde(rename = "collisionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a DaemonSet's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::extensions::v1beta1::DaemonSetCondition>>,

    /// The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[serde(rename = "currentNumberScheduled")]
    pub current_number_scheduled: i32,

    /// The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[serde(rename = "desiredNumberScheduled")]
    pub desired_number_scheduled: i32,

    /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)
    #[serde(rename = "numberAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_available: Option<i32>,

    /// The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[serde(rename = "numberMisscheduled")]
    pub number_misscheduled: i32,

    /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and ready.
    #[serde(rename = "numberReady")]
    pub number_ready: i32,

    /// The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
    #[serde(rename = "numberUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_unavailable: Option<i32>,

    /// The most recent generation observed by the daemon set controller.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// The total number of nodes that are running updated daemon pod
    #[serde(rename = "updatedNumberScheduled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: Option<i32>,
}
