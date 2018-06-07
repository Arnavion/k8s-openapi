// Generated from definition io.k8s.api.core.v1.AzureFilePersistentVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AzureFilePersistentVolumeSource {
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// the name of secret that contains Azure Storage Account Name and Key
    #[serde(rename = "secretName")]
    pub secret_name: String,

    /// the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod
    #[serde(rename = "secretNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<String>,

    /// Share Name
    #[serde(rename = "shareName")]
    pub share_name: String,
}
