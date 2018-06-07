// Generated from definition io.k8s.api.core.v1.GitRepoVolumeSource

/// Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GitRepoVolumeSource {
    /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,

    /// Repository URL
    pub repository: String,

    /// Commit hash for the specified revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
