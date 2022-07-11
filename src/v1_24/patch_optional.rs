// Generated from definition io.k8s.PatchOptional

/// Common parameters for all patch operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PatchOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,

    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,

    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,

    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,

}

#[cfg(feature = "dsl")]
impl<'a> PatchOptional<'a>  {
    /// Set [`Self::dry_run`]
    pub  fn dry_run_set(&mut self, dry_run: impl Into<Option<&'a str>>) -> &mut Self {
        self.dry_run = dry_run.into(); self
    }


    /// Set [`Self::field_manager`]
    pub  fn field_manager_set(&mut self, field_manager: impl Into<Option<&'a str>>) -> &mut Self {
        self.field_manager = field_manager.into(); self
    }


    /// Set [`Self::field_validation`]
    pub  fn field_validation_set(&mut self, field_validation: impl Into<Option<&'a str>>) -> &mut Self {
        self.field_validation = field_validation.into(); self
    }


    /// Set [`Self::force`]
    pub  fn force_set(&mut self, force: impl Into<Option<bool>>) -> &mut Self {
        self.force = force.into(); self
    }


}


#[cfg(feature = "api")]
impl<'a> PatchOptional<'a> {
    #[doc(hidden)]
    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]
    ///
    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.
    pub fn __serialize<T>(
        self,
        __query_pairs: &mut crate::url::form_urlencoded::Serializer<'_, T>,
    ) where T: crate::url::form_urlencoded::Target {
        if let Some(value) = self.dry_run {
            __query_pairs.append_pair("dryRun", value);
        }
        if let Some(value) = self.field_manager {
            __query_pairs.append_pair("fieldManager", value);
        }
        if let Some(value) = self.field_validation {
            __query_pairs.append_pair("fieldValidation", value);
        }
        if let Some(value) = self.force {
            __query_pairs.append_pair("force", if value { "true" } else { "false" });
        }
    }
}
