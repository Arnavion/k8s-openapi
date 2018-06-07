// Generated from definition io.k8s.api.autoscaling.v2beta1.ExternalMetricStatus

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ExternalMetricStatus {
    /// currentAverageValue is the current value of metric averaged over autoscaled pods.
    #[serde(rename = "currentAverageValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<::v1_10::apimachinery::pkg::api::resource::Quantity>,

    /// currentValue is the current value of the metric (as a quantity)
    #[serde(rename = "currentValue")]
    pub current_value: ::v1_10::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of a metric used for autoscaling in metric system.
    #[serde(rename = "metricName")]
    pub metric_name: String,

    /// metricSelector is used to identify a specific time series within a given metric.
    #[serde(rename = "metricSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<::v1_10::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
