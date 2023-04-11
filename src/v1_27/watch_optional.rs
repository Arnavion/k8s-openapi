// Generated from definition io.k8s.WatchOptional

/// Common parameters for all watch operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WatchOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,

    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,

    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,

    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,

    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,

    /// `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic "Bookmark" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `"k8s.io/initial-events-end": "true"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.
    ///
    /// When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan
    ///   is interpreted as "data at least as new as the provided `resourceVersion`"
    ///   and the bookmark event is send when the state is synced
    ///   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.
    ///   If `resourceVersion` is unset, this is interpreted as "consistent read" and the
    ///   bookmark event is send when the state is synced at least to the moment
    ///   when request started being processed.
    /// - `resourceVersionMatch` set to any other value or unset
    ///   Invalid error is returned.
    ///
    /// Defaults to true if `resourceVersion=""` or `resourceVersion="0"` (for backward compatibility reasons) and to false otherwise.
    pub send_initial_events: Option<bool>,

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
        if let Some(value) = self.allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if value { "true" } else { "false" });
        }
        if let Some(value) = self.field_selector {
            __query_pairs.append_pair("fieldSelector", value);
        }
        if let Some(value) = self.label_selector {
            __query_pairs.append_pair("labelSelector", value);
        }
        if let Some(value) = self.resource_version {
            __query_pairs.append_pair("resourceVersion", value);
        }
        if let Some(value) = self.resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", value);
        }
        if let Some(value) = self.send_initial_events {
            __query_pairs.append_pair("sendInitialEvents", if value { "true" } else { "false" });
        }
        if let Some(value) = self.timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &value.to_string());
        }
        __query_pairs.append_pair("watch", "true");
    }
}
