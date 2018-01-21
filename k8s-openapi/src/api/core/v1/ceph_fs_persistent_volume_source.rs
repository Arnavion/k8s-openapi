// Generated from definition io.k8s.api.core.v1.CephFSPersistentVolumeSource

/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CephFSPersistentVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors More info: https://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    pub monitors: Vec<String>,

    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "secretFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_file: Option<String>,

    /// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<::api::core::v1::SecretReference>,

    /// Optional: User is the rados user name, default is admin More info: https://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
