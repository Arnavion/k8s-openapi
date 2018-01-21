// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerStatus

/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HorizontalPodAutoscalerStatus {
    /// conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
    pub conditions: Vec<::api::autoscaling::v2beta1::HorizontalPodAutoscalerCondition>,

    /// currentMetrics is the last read state of the metrics used by this autoscaler.
    #[serde(rename = "currentMetrics")]
    pub current_metrics: Vec<::api::autoscaling::v2beta1::MetricStatus>,

    /// currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
    #[serde(rename = "currentReplicas")]
    pub current_replicas: i32,

    /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
    #[serde(rename = "desiredReplicas")]
    pub desired_replicas: i32,

    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
    #[serde(rename = "lastScaleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// observedGeneration is the most recent generation observed by this autoscaler.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}
