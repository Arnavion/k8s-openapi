// Generated from definition io.k8s.api.policy.v1beta1.AllowedHostPath

/// defines the host volume conditions that will be enabled by a policy for pods to use. It requires the path prefix to be defined.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AllowedHostPath {
    /// is the path prefix that the host volume must match. It does not support `*`. Trailing slashes are trimmed when validating the path prefix with a host path.
    ///
    /// Examples: `/foo` would allow `/foo`, `/foo/` and `/foo/bar` `/foo` would not allow `/food` or `/etc/foo`
    #[serde(rename = "pathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}
