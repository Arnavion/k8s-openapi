// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.StatusCause

/// StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StatusCause {
    /// The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.
    ///
    /// Examples:
    ///   "name" - the field "name" on the current resource
    ///   "items[0].name" - the field "name" on the first array entry in "items"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// A human-readable description of the cause of the error.  This field may be presented as-is to a reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// A machine-readable description of the cause of the error. If this value is empty there is no information available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
