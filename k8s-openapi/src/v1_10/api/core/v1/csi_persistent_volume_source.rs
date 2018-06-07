// Generated from definition io.k8s.api.core.v1.CSIPersistentVolumeSource

/// Represents storage that is managed by an external CSI volume driver (Beta feature)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CSIPersistentVolumeSource {
    /// ControllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(rename = "controllerPublishSecretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_publish_secret_ref: Option<::v1_10::api::core::v1::SecretReference>,

    /// Driver is the name of the driver to use for this volume. Required.
    pub driver: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(rename = "nodePublishSecretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<::v1_10::api::core::v1::SecretReference>,

    /// NodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(rename = "nodeStageSecretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_stage_secret_ref: Option<::v1_10::api::core::v1::SecretReference>,

    /// Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Attributes of the volume to publish.
    #[serde(rename = "volumeAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes: Option<::std::collections::BTreeMap<String, String>>,

    /// VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    #[serde(rename = "volumeHandle")]
    pub volume_handle: String,
}
