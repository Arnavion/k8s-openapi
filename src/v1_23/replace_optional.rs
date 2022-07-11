// Generated from definition io.k8s.ReplaceOptional

/// Common parameters for all replace operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ReplaceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,

    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,

    /// fieldValidation determines how the server should respond to unknown/duplicate fields in the object in the request. Introduced as alpha in 1.23, older servers or servers with the `ServerSideFieldValidation` feature disabled will discard valid values specified in  this param and not perform any server side field validation. Valid values are: - Ignore: ignores unknown/duplicate fields. - Warn: responds with a warning for each unknown/duplicate field, but successfully serves the request. - Strict: fails the request on unknown/duplicate fields.
    pub field_validation: Option<&'a str>,

}

#[cfg(feature = "dsl")]
impl<'a> ReplaceOptional<'a>  {
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


}


#[cfg(feature = "api")]
impl<'a> ReplaceOptional<'a> {
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
    }
}
