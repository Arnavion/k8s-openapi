// Generated from definition io.k8s.api.core.v1.LocalVolumeSource

/// Local represents directly-attached storage with node affinity
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LocalVolumeSource {
    /// The full path to the volume on the node For alpha, this path must be a directory Once block as a source is supported, then this path can point to a block device
    pub path: String,
}
