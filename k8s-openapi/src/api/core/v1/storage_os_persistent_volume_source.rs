// Generated from definition io.k8s.api.core.v1.StorageOSPersistentVolumeSource

/// Represents a StorageOS persistent volume resource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StorageOSPersistentVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<::api::core::v1::ObjectReference>,

    /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    #[serde(rename = "volumeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,

    /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    #[serde(rename = "volumeNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_namespace: Option<String>,
}
