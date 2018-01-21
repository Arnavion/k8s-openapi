// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Status

/// Status is a return value for calls that don't return other objects.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Status {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Suggested HTTP return code for this status, 0 if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,

    /// Extended data associated with the reason.  Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::apimachinery::pkg::apis::meta::v1::StatusDetails>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A human-readable description of the status of this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ListMeta>,

    /// A machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Status of the operation. One of: "Success" or "Failure". More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
