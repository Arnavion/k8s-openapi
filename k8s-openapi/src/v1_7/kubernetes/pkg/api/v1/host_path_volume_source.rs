// Generated from definition io.k8s.kubernetes.pkg.api.v1.HostPathVolumeSource

/// Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HostPathVolumeSource {
    /// Path of the directory on the host. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub path: String,
}
