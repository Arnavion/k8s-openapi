// Generated from definition io.k8s.api.core.v1.ResourceQuotaSpec

/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceQuotaSpec {
    /// Hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}
