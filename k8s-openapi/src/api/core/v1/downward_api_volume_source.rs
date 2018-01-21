// Generated from definition io.k8s.api.core.v1.DownwardAPIVolumeSource

/// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownwardAPIVolumeSource {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename = "defaultMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,

    /// Items is a list of downward API volume file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<::api::core::v1::DownwardAPIVolumeFile>>,
}
