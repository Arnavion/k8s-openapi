// Generated from definition io.k8s.api.core.v1.LimitRangeSpec

/// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LimitRangeSpec {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    pub limits: Vec<::api::core::v1::LimitRangeItem>,
}
