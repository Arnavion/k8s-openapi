// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.SELinuxStrategyOptions

/// SELinux  Strategy Options defines the strategy type and any options used to create the strategy.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SELinuxStrategyOptions {
    /// type is the strategy that will dictate the allowable labels that may be set.
    pub rule: String,

    /// seLinuxOptions required to run as; required for MustRunAs More info: https://git.k8s.io/community/contributors/design-proposals/security_context.md
    #[serde(rename = "seLinuxOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<::v1_7::kubernetes::pkg::api::v1::SELinuxOptions>,
}
