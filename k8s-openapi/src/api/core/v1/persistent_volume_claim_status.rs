// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimStatus

/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PersistentVolumeClaimStatus {
    /// AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(rename = "accessModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<Vec<String>>,

    /// Represents the actual resources of the underlying volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::core::v1::PersistentVolumeClaimCondition>>,

    /// Phase represents the current phase of PersistentVolumeClaim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}
