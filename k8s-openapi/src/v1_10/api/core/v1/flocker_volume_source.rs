// Generated from definition io.k8s.api.core.v1.FlockerVolumeSource

/// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FlockerVolumeSource {
    /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,

    /// UUID of the dataset. This is unique identifier of a Flocker dataset
    #[serde(rename = "datasetUUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_uuid: Option<String>,
}
