// Generated from definition io.k8s.api.core.v1.ISCSIVolumeSource

/// Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ISCSIVolumeSource {
    /// whether support iSCSI Discovery CHAP authentication
    #[serde(rename = "chapAuthDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_auth_discovery: Option<bool>,

    /// whether support iSCSI Session CHAP authentication
    #[serde(rename = "chapAuthSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_auth_session: Option<bool>,

    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
    #[serde(rename = "fsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,

    /// Custom iSCSI initiator name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
    #[serde(rename = "initiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,

    /// Target iSCSI Qualified Name.
    pub iqn: String,

    /// Optional: Defaults to 'default' (tcp). iSCSI interface name that uses an iSCSI transport.
    #[serde(rename = "iscsiInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi_interface: Option<String>,

    /// iSCSI target lun number.
    pub lun: i32,

    /// iSCSI target portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portals: Option<Vec<String>>,

    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// CHAP secret for iSCSI target and initiator authentication
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<::v1_8::api::core::v1::LocalObjectReference>,

    /// iSCSI target portal. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(rename = "targetPortal")]
    pub target_portal: String,
}
