// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicyPeer

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NetworkPolicyPeer {
    /// IPBlock defines policy on a particular IPBlock
    #[serde(rename = "ipBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<::v1_8::api::extensions::v1beta1::IPBlock>,

    /// Selects Namespaces using cluster scoped-labels.  This matches all pods in all namespaces selected by this label selector. This field follows standard label selector semantics. If present but empty, this selector selects all namespaces.
    #[serde(rename = "namespaceSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<::v1_8::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// This is a label selector which selects Pods in this namespace. This field follows standard label selector semantics. If present but empty, this selector selects all pods in this namespace.
    #[serde(rename = "podSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<::v1_8::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
