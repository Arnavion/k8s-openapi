// Generated from definition io.k8s.api.core.v1.AzureFileVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AzureFileVolumeSource {
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// the name of secret that contains Azure Storage Account Name and Key
    #[serde(rename = "secretName")]
    pub secret_name: String,

    /// Share Name
    #[serde(rename = "shareName")]
    pub share_name: String,
}
