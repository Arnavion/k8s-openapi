// Generated from definition io.k8s.api.core.v1.ProjectedVolumeSource

/// Represents a projected volume source
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectedVolumeSource {
    /// Mode bits to use on created files by default. Must be a value between 0 and 0777. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename = "defaultMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,

    /// list of volume projections
    pub sources: Vec<::api::core::v1::VolumeProjection>,
}
