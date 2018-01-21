// Generated from definition io.k8s.api.core.v1.ScaleIOVolumeSource

/// ScaleIOVolumeSource represents a persistent ScaleIO volume
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ScaleIOVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// The host address of the ScaleIO API Gateway.
    pub gateway: String,

    /// The name of the ScaleIO Protection Domain for the configured storage.
    #[serde(rename = "protectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_domain: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
    #[serde(rename = "secretRef")]
    pub secret_ref: ::api::core::v1::LocalObjectReference,

    /// Flag to enable/disable SSL communication with Gateway, default false
    #[serde(rename = "sslEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: Option<bool>,

    /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned.
    #[serde(rename = "storageMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<String>,

    /// The ScaleIO Storage Pool associated with the protection domain.
    #[serde(rename = "storagePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_pool: Option<String>,

    /// The name of the storage system as configured in ScaleIO.
    pub system: String,

    /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
    #[serde(rename = "volumeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}
