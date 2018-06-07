// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Initializer

/// Initializer is information about an initializer that has not yet completed.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Initializer {
    /// name of the process that is responsible for initializing this object.
    pub name: String,
}
