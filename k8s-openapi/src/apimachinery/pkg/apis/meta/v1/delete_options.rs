// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions

/// DeleteOptions may be provided when deleting an API object.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeleteOptions {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    #[serde(rename = "gracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_period_seconds: Option<i64>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    #[serde(rename = "orphanDependents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_dependents: Option<bool>,

    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditions: Option<::apimachinery::pkg::apis::meta::v1::Preconditions>,

    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    #[serde(rename = "propagationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation_policy: Option<String>,
}
