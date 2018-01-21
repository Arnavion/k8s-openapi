// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicyIngressRule

/// DEPRECATED 1.9 - This group version of NetworkPolicyIngressRule is deprecated by networking/v1/NetworkPolicyIngressRule. This NetworkPolicyIngressRule matches traffic if and only if the traffic matches both ports AND from.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NetworkPolicyIngressRule {
    /// List of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least on item, this rule allows traffic only if the traffic matches at least one item in the from list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<::api::extensions::v1beta1::NetworkPolicyPeer>>,

    /// List of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<::api::extensions::v1beta1::NetworkPolicyPort>>,
}
