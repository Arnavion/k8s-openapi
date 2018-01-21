// Generated from definition io.k8s.api.core.v1.LimitRangeItem

/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LimitRangeItem {
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    #[serde(rename = "defaultRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_request: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Max usage constraints on this kind by resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    #[serde(rename = "maxLimitRequestRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit_request_ratio: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Min usage constraints on this kind by resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Type of resource that this limit applies to.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
