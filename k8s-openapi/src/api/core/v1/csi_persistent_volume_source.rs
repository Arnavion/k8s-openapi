// Generated from definition io.k8s.api.core.v1.CSIPersistentVolumeSource

/// Represents storage that is managed by an external CSI volume driver
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CSIPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume. Required.
    pub driver: String,

    /// Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    #[serde(rename = "volumeHandle")]
    pub volume_handle: String,
}
