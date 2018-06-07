// Generated from definition io.k8s.api.core.v1.AzureDiskVolumeSource

/// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AzureDiskVolumeSource {
    /// Host Caching mode: None, Read Only, Read Write.
    #[serde(rename = "cachingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_mode: Option<String>,

    /// The Name of the data disk in the blob storage
    #[serde(rename = "diskName")]
    pub disk_name: String,

    /// The URI the data disk in the blob storage
    #[serde(rename = "diskURI")]
    pub disk_uri: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
