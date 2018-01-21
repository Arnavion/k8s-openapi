// Generated from definition io.k8s.api.core.v1.VolumeDevice

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeDevice {
    /// devicePath is the path inside of the container that the device will be mapped to.
    #[serde(rename = "devicePath")]
    pub device_path: String,

    /// name must match the name of a persistentVolumeClaim in the pod
    pub name: String,
}
