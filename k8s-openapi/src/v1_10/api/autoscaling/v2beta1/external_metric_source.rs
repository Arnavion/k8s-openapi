// Generated from definition io.k8s.api.autoscaling.v2beta1.ExternalMetricSource

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). Exactly one "target" type should be set.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ExternalMetricSource {
    /// metricName is the name of the metric in question.
    #[serde(rename = "metricName")]
    pub metric_name: String,

    /// metricSelector is used to identify a specific time series within a given metric.
    #[serde(rename = "metricSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<::v1_10::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue.
    #[serde(rename = "targetAverageValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<::v1_10::apimachinery::pkg::api::resource::Quantity>,

    /// targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue.
    #[serde(rename = "targetValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<::v1_10::apimachinery::pkg::api::resource::Quantity>,
}
