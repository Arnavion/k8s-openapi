// Generated from definition io.k8s.api.core.v1.ExecAction

/// ExecAction describes a "run in container" action.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ExecAction {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}
