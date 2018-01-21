// Generated from definition io.k8s.api.autoscaling.v2beta1.PodsMetricSource

/// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodsMetricSource {
    /// metricName is the name of the metric in question
    #[serde(rename = "metricName")]
    pub metric_name: String,

    /// targetAverageValue is the target value of the average of the metric across all relevant pods (as a quantity)
    #[serde(rename = "targetAverageValue")]
    pub target_average_value: ::apimachinery::pkg::api::resource::Quantity,
}
