// Generated from definition io.k8s.api.core.v1.DownwardAPIVolumeFile

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownwardAPIVolumeFile {
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    #[serde(rename = "fieldRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<::api::core::v1::ObjectFieldSelector>,

    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,

    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: String,

    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(rename = "resourceFieldRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<::api::core::v1::ResourceFieldSelector>,
}
