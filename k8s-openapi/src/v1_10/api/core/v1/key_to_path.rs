// Generated from definition io.k8s.api.core.v1.KeyToPath

/// Maps a string key to a path within a volume.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct KeyToPath {
    /// The key to project.
    pub key: String,

    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,

    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}
