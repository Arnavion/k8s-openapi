// Generated from definition io.k8s.api.core.v1.AttachedVolume

/// AttachedVolume describes a volume attached to a node
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AttachedVolume {
    /// DevicePath represents the device path where the volume should be available
    #[serde(rename = "devicePath")]
    pub device_path: String,

    /// Name of the attached volume
    pub name: String,
}
