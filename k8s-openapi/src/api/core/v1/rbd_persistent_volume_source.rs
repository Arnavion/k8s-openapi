// Generated from definition io.k8s.api.core.v1.RBDPersistentVolumeSource

/// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RBDPersistentVolumeSource {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// The rados image name. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub image: String,

    /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyring: Option<String>,

    /// A collection of Ceph monitors. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub monitors: Vec<String>,

    /// The rados pool name. Default is rbd. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<::api::core::v1::SecretReference>,

    /// The rados user name. Default is admin. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
