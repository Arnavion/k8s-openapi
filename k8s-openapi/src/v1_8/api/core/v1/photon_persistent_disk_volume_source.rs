// Generated from definition io.k8s.api.core.v1.PhotonPersistentDiskVolumeSource

/// Represents a Photon Controller persistent disk resource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PhotonPersistentDiskVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// ID that identifies Photon Controller persistent disk
    #[serde(rename = "pdID")]
    pub pd_id: String,
}
