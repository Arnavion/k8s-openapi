// Generated from definition io.k8s.api.autoscaling.v2beta1.ObjectMetricStatus

/// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ObjectMetricStatus {
    /// currentValue is the current value of the metric (as a quantity).
    #[serde(rename = "currentValue")]
    pub current_value: ::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of the metric in question.
    #[serde(rename = "metricName")]
    pub metric_name: String,

    /// target is the described Kubernetes object.
    pub target: ::api::autoscaling::v2beta1::CrossVersionObjectReference,
}
