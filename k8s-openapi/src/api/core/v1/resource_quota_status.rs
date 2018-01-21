// Generated from definition io.k8s.api.core.v1.ResourceQuotaStatus

/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,

    /// Used is the current observed total usage of the resource in the namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<::std::collections::BTreeMap<String, ::apimachinery::pkg::api::resource::Quantity>>,
}
