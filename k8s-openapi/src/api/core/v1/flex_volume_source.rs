// Generated from definition io.k8s.api.core.v1.FlexVolumeSource

/// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FlexVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Optional: Extra command options if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::BTreeMap<String, String>>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<::api::core::v1::LocalObjectReference>,
}
