// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Initializers

/// Initializers tracks the progress of initialization.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Initializers {
    /// Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.
    pub pending: Vec<::apimachinery::pkg::apis::meta::v1::Initializer>,

    /// If result is set with the Failure field, the object will be persisted to storage and then deleted, ensuring that other clients can observe the deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<::apimachinery::pkg::apis::meta::v1::Status>,
}
