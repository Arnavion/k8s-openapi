// Generated from definition io.k8s.WatchOptional

/// Common parameters for all watch operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WatchOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,

    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,

    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,

    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,

    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,

    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
}

#[cfg(feature = "api")]
impl<'a> WatchOptional<'a> {
    #[doc(hidden)]
    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]
    ///
    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.
    pub fn __serialize<T>(
        self,
        __query_pairs: &mut crate::url::form_urlencoded::Serializer<'_, T>,
    ) where T: crate::url::form_urlencoded::Target {
        if let Some(value) = &self.field_selector {
            __query_pairs.append_pair("fieldSelector", value);
        }
        if let Some(value) = &self.include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &value.to_string());
        }
        if let Some(value) = &self.label_selector {
            __query_pairs.append_pair("labelSelector", value);
        }
        if let Some(value) = &self.pretty {
            __query_pairs.append_pair("pretty", value);
        }
        if let Some(value) = &self.resource_version {
            __query_pairs.append_pair("resourceVersion", value);
        }
        if let Some(value) = &self.timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &value.to_string());
        }
        __query_pairs.append_pair("watch", "true");
    }
}
