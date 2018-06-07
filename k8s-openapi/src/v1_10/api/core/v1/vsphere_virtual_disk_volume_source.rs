// Generated from definition io.k8s.api.core.v1.VsphereVirtualDiskVolumeSource

/// Represents a vSphere volume resource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VsphereVirtualDiskVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
    #[serde(rename = "storagePolicyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_id: Option<String>,

    /// Storage Policy Based Management (SPBM) profile name.
    #[serde(rename = "storagePolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_name: Option<String>,

    /// Path that identifies vSphere volume vmdk
    #[serde(rename = "volumePath")]
    pub volume_path: String,
}
