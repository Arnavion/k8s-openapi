// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.NetworkPolicyPeer

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NetworkPolicyPeer {
    /// Selects Namespaces using cluster scoped-labels.  This matches all pods in all namespaces selected by this label selector. This field follows standard label selector semantics. If present but empty, this selector selects all namespaces.
    #[serde(rename = "namespaceSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// This is a label selector which selects Pods in this namespace. This field follows standard label selector semantics. If present but empty, this selector selects all pods in this namespace.
    #[serde(rename = "podSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
