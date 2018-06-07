// Generated from definition io.k8s.api.core.v1.VolumeNodeAffinity

/// VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeNodeAffinity {
    /// Required specifies hard node constraints that must be met.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<::v1_10::api::core::v1::NodeSelector>,
}
