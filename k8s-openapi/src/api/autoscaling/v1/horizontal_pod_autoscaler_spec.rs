// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerSpec

/// specification of a horizontal pod autoscaler.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HorizontalPodAutoscalerSpec {
    /// upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    #[serde(rename = "maxReplicas")]
    pub max_replicas: i32,

    /// lower limit for the number of pods that can be set by the autoscaler, default 1.
    #[serde(rename = "minReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,

    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
    #[serde(rename = "scaleTargetRef")]
    pub scale_target_ref: ::api::autoscaling::v1::CrossVersionObjectReference,

    /// target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
    #[serde(rename = "targetCPUUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cpu_utilization_percentage: Option<i32>,
}
