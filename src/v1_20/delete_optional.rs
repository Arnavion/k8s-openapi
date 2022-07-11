// Generated from definition io.k8s.DeleteOptional

/// Common parameters for all delete and delete-collection operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeleteOptional<'a> {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    pub api_version: Option<&'a str>,

    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a [String]>,

    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: Option<&'a str>,

    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,

    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    pub preconditions: Option<&'a crate::apimachinery::pkg::apis::meta::v1::Preconditions>,

    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,

}

#[cfg(feature = "dsl")]
impl<'a> DeleteOptional<'a>  {
    /// Set [`Self::api_version`]
    pub  fn api_version_set(&mut self, api_version: impl Into<Option<&'a str>>) -> &mut Self {
        self.api_version = api_version.into(); self
    }


    /// Set [`Self::dry_run`]
    pub  fn dry_run_set(&mut self, dry_run: impl Into<Option<&'a [String]>>) -> &mut Self {
        self.dry_run = dry_run.into(); self
    }


    /// Set [`Self::grace_period_seconds`]
    pub  fn grace_period_seconds_set(&mut self, grace_period_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.grace_period_seconds = grace_period_seconds.into(); self
    }


    /// Set [`Self::kind`]
    pub  fn kind_set(&mut self, kind: impl Into<Option<&'a str>>) -> &mut Self {
        self.kind = kind.into(); self
    }


    /// Set [`Self::orphan_dependents`]
    pub  fn orphan_dependents_set(&mut self, orphan_dependents: impl Into<Option<bool>>) -> &mut Self {
        self.orphan_dependents = orphan_dependents.into(); self
    }


    /// Set [`Self::preconditions`]
    pub  fn preconditions_set(&mut self, preconditions: impl Into<Option<&'a crate::apimachinery::pkg::apis::meta::v1::Preconditions>>) -> &mut Self {
        self.preconditions = preconditions.into(); self
    }


    /// Set [`Self::propagation_policy`]
    pub  fn propagation_policy_set(&mut self, propagation_policy: impl Into<Option<&'a str>>) -> &mut Self {
        self.propagation_policy = propagation_policy.into(); self
    }


}


impl<'a> crate::serde::Serialize for DeleteOptional<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeleteOptional",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.dry_run.as_ref().map_or(0, |_| 1) +
            self.grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.orphan_dependents.as_ref().map_or(0, |_| 1) +
            self.preconditions.as_ref().map_or(0, |_| 1) +
            self.propagation_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.dry_run {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dryRun", value)?;
        }
        if let Some(value) = &self.grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.orphan_dependents {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "orphanDependents", value)?;
        }
        if let Some(value) = &self.preconditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preconditions", value)?;
        }
        if let Some(value) = &self.propagation_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "propagationPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
