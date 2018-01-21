// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerSpec

/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HorizontalPodAutoscalerSpec {
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
    #[serde(rename = "maxReplicas")]
    pub max_replicas: i32,

    /// metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<::api::autoscaling::v2beta1::MetricSpec>>,

    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down. It defaults to 1 pod.
    #[serde(rename = "minReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,

    /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
    #[serde(rename = "scaleTargetRef")]
    pub scale_target_ref: ::api::autoscaling::v2beta1::CrossVersionObjectReference,
}
