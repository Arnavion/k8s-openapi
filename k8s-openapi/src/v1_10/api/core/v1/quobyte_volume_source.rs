// Generated from definition io.k8s.api.core.v1.QuobyteVolumeSource

/// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct QuobyteVolumeSource {
    /// Group to map volume access to Default is no group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
    pub registry: String,

    /// User to map volume access to Defaults to serivceaccount user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Volume is a string that references an already created Quobyte volume by name.
    pub volume: String,
}
