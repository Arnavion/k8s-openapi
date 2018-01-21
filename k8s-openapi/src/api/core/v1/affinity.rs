// Generated from definition io.k8s.api.core.v1.Affinity

/// Affinity is a group of affinity scheduling rules.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Affinity {
    /// Describes node affinity scheduling rules for the pod.
    #[serde(rename = "nodeAffinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<::api::core::v1::NodeAffinity>,

    /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
    #[serde(rename = "podAffinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<::api::core::v1::PodAffinity>,

    /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
    #[serde(rename = "podAntiAffinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<::api::core::v1::PodAntiAffinity>,
}
