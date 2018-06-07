// Generated from definition io.k8s.api.core.v1.PortworxVolumeSource

/// PortworxVolumeSource represents a Portworx volume resource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PortworxVolumeSource {
    /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// VolumeID uniquely identifies a Portworx volume
    #[serde(rename = "volumeID")]
    pub volume_id: String,
}
