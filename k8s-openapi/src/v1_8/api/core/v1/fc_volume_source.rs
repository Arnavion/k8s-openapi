// Generated from definition io.k8s.api.core.v1.FCVolumeSource

/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FCVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Optional: FC target lun number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Optional: FC target worldwide names (WWNs)
    #[serde(rename = "targetWWNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ww_ns: Option<Vec<String>>,

    /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wwids: Option<Vec<String>>,
}
