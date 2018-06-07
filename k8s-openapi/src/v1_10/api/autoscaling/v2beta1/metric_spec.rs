// Generated from definition io.k8s.api.autoscaling.v2beta1.MetricSpec

/// MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricSpec {
    /// external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<::v1_10::api::autoscaling::v2beta1::ExternalMetricSource>,

    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<::v1_10::api::autoscaling::v2beta1::ObjectMetricSource>,

    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: Option<::v1_10::api::autoscaling::v2beta1::PodsMetricSource>,

    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<::v1_10::api::autoscaling::v2beta1::ResourceMetricSource>,

    /// type is the type of metric source.  It should be one of "Object", "Pods" or "Resource", each mapping to a matching field in the object.
    #[serde(rename = "type")]
    pub type_: String,
}
