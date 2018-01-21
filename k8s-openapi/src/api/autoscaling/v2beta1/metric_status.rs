// Generated from definition io.k8s.api.autoscaling.v2beta1.MetricStatus

/// MetricStatus describes the last-read state of a single metric.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricStatus {
    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<::api::autoscaling::v2beta1::ObjectMetricStatus>,

    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: Option<::api::autoscaling::v2beta1::PodsMetricStatus>,

    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<::api::autoscaling::v2beta1::ResourceMetricStatus>,

    /// type is the type of metric source.  It will be one of "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
    #[serde(rename = "type")]
    pub type_: String,
}
