// Generated from definition io.k8s.api.storage.v1alpha1.VolumeError

/// VolumeError captures an error encountered during a volume operation.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeError {
    /// String detailing the error encountered during Attach or Detach operation. This string maybe logged, so it should not contain sensitive information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Time the error was encountered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<::apimachinery::pkg::apis::meta::v1::Time>,
}
