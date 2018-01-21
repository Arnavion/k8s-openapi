// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerStatus

/// current status of a horizontal pod autoscaler
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HorizontalPodAutoscalerStatus {
    /// current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU.
    #[serde(rename = "currentCPUUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_cpu_utilization_percentage: Option<i32>,

    /// current number of replicas of pods managed by this autoscaler.
    #[serde(rename = "currentReplicas")]
    pub current_replicas: i32,

    /// desired number of replicas of pods managed by this autoscaler.
    #[serde(rename = "desiredReplicas")]
    pub desired_replicas: i32,

    /// last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed.
    #[serde(rename = "lastScaleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// most recent generation observed by this autoscaler.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}
