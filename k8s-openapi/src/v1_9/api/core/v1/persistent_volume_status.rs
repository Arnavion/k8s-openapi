// Generated from definition io.k8s.api.core.v1.PersistentVolumeStatus

/// PersistentVolumeStatus is the current status of a persistent volume.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PersistentVolumeStatus {
    /// A human-readable message indicating details about why the volume is in this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// Reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
