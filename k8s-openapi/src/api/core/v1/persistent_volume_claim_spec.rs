// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimSpec

/// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(rename = "accessModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<Vec<String>>,

    /// Resources represents the minimum resources the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<::api::core::v1::ResourceRequirements>,

    /// A label query over volumes to consider for binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    #[serde(rename = "storageClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,

    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. This is an alpha feature and may change in the future.
    #[serde(rename = "volumeMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,

    /// VolumeName is the binding reference to the PersistentVolume backing this claim.
    #[serde(rename = "volumeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}
