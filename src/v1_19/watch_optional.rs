// Generated from definition io.k8s.WatchOptional

/// Common parameters for all watch operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WatchOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,

    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,

    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,

    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,

    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
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
        if let Some(value) = &self.allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", &value.to_string());
        }
        if let Some(value) = &self.field_selector {
            __query_pairs.append_pair("fieldSelector", value);
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
