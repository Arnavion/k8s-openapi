// Generated from definition io.k8s.api.policy.v1beta1.IDRange

/// ID Range provides a min/max of an allowed range of IDs.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IDRange {
    /// Max is the end of the range, inclusive.
    pub max: i64,

    /// Min is the start of the range, inclusive.
    pub min: i64,
}
