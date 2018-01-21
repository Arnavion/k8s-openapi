// Generated from definition io.k8s.api.autoscaling.v2beta1.ObjectMetricSource

/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ObjectMetricSource {
    /// metricName is the name of the metric in question.
    #[serde(rename = "metricName")]
    pub metric_name: String,

    /// target is the described Kubernetes object.
    pub target: ::api::autoscaling::v2beta1::CrossVersionObjectReference,

    /// targetValue is the target value of the metric (as a quantity).
    #[serde(rename = "targetValue")]
    pub target_value: ::apimachinery::pkg::api::resource::Quantity,
}
