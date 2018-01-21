// Generated from definition io.k8s.api.extensions.v1beta1.RunAsUserStrategyOptions

/// Run A sUser Strategy Options defines the strategy type and any options used to create the strategy.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RunAsUserStrategyOptions {
    /// Ranges are the allowed ranges of uids that may be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<::api::extensions::v1beta1::IDRange>>,

    /// Rule is the strategy that will dictate the allowable RunAsUser values that may be set.
    pub rule: String,
}
