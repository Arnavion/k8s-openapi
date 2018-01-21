// Generated from definition io.k8s.api.core.v1.DownwardAPIProjection

/// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<::api::core::v1::DownwardAPIVolumeFile>>,
}
