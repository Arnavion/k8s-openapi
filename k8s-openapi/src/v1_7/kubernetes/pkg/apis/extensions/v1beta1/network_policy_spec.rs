// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.NetworkPolicySpec

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NetworkPolicySpec {
    /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::NetworkPolicyIngressRule>>,

    /// Selects the pods to which this NetworkPolicy object applies.  The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods.  In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
    #[serde(rename = "podSelector")]
    pub pod_selector: ::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector,
}
