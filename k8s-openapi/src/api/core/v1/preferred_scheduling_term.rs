// Generated from definition io.k8s.api.core.v1.PreferredSchedulingTerm

/// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PreferredSchedulingTerm {
    /// A node selector term, associated with the corresponding weight.
    pub preference: ::api::core::v1::NodeSelectorTerm,

    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    pub weight: i32,
}
