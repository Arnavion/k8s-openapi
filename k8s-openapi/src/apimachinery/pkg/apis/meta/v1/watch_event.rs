// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

/// Event represents a single event to a watched resource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WatchEvent {
    /// Object is:
    ///  * If Type is Added or Modified: the new state of the object.
    ///  * If Type is Deleted: the state of the object immediately before deletion.
    ///  * If Type is Error: *Status is recommended; other types may make sense
    ///    depending on context.
    pub object: ::apimachinery::pkg::runtime::RawExtension,

    #[serde(rename = "type")]
    pub type_: String,
}
