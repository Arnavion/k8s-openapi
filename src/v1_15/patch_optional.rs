// Generated from definition io.k8s.PatchOptional

/// Common parameters for all patch operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PatchOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,

    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,

    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,

    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
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
        if let Some(value) = &self.dry_run {
            __query_pairs.append_pair("dryRun", value);
        }
        if let Some(value) = &self.field_manager {
            __query_pairs.append_pair("fieldManager", value);
        }
        if let Some(value) = &self.force {
            __query_pairs.append_pair("force", &value.to_string());
        }
        if let Some(value) = &self.pretty {
            __query_pairs.append_pair("pretty", value);
        }
    }
}
