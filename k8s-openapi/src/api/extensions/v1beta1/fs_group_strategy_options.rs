// Generated from definition io.k8s.api.extensions.v1beta1.FSGroupStrategyOptions

/// FSGroupStrategyOptions defines the strategy type and options used to create the strategy.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FSGroupStrategyOptions {
    /// Ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<::api::extensions::v1beta1::IDRange>>,

    /// Rule is the strategy that will dictate what FSGroup is used in the SecurityContext.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}
