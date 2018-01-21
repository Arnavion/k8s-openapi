// Generated from definition io.k8s.api.autoscaling.v2beta1.ResourceMetricSource

/// ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.  Only one "target" type should be set.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceMetricSource {
    /// name is the name of the resource in question.
    pub name: String,

    /// targetAverageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
    #[serde(rename = "targetAverageUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,

    /// targetAverageValue is the target value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the "pods" metric source type.
    #[serde(rename = "targetAverageValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<::apimachinery::pkg::api::resource::Quantity>,
}
