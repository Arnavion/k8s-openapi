// Generated from definition io.k8s.ListOptional

/// Common parameters for all list operations.
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ListOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,

    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,

    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,

    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,

    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,

    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,

    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,

}

#[cfg(feature = "dsl")]
impl<'a> ListOptional<'a>  {
    /// Set [`Self::continue_`]
    pub  fn continue_set(&mut self, continue_: impl Into<Option<&'a str>>) -> &mut Self {
        self.continue_ = continue_.into(); self
    }


    /// Set [`Self::field_selector`]
    pub  fn field_selector_set(&mut self, field_selector: impl Into<Option<&'a str>>) -> &mut Self {
        self.field_selector = field_selector.into(); self
    }


    /// Set [`Self::label_selector`]
    pub  fn label_selector_set(&mut self, label_selector: impl Into<Option<&'a str>>) -> &mut Self {
        self.label_selector = label_selector.into(); self
    }


    /// Set [`Self::limit`]
    pub  fn limit_set(&mut self, limit: impl Into<Option<i64>>) -> &mut Self {
        self.limit = limit.into(); self
    }


    /// Set [`Self::resource_version`]
    pub  fn resource_version_set(&mut self, resource_version: impl Into<Option<&'a str>>) -> &mut Self {
        self.resource_version = resource_version.into(); self
    }


    /// Set [`Self::resource_version_match`]
    pub  fn resource_version_match_set(&mut self, resource_version_match: impl Into<Option<&'a str>>) -> &mut Self {
        self.resource_version_match = resource_version_match.into(); self
    }


    /// Set [`Self::timeout_seconds`]
    pub  fn timeout_seconds_set(&mut self, timeout_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.timeout_seconds = timeout_seconds.into(); self
    }


}


#[cfg(feature = "api")]
impl<'a> ListOptional<'a> {
    #[doc(hidden)]
    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]
    ///
    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.
    pub fn __serialize<T>(
        self,
        __query_pairs: &mut crate::url::form_urlencoded::Serializer<'_, T>,
    ) where T: crate::url::form_urlencoded::Target {
        if let Some(value) = self.continue_ {
            __query_pairs.append_pair("continue", value);
        }
        if let Some(value) = self.field_selector {
            __query_pairs.append_pair("fieldSelector", value);
        }
        if let Some(value) = self.label_selector {
            __query_pairs.append_pair("labelSelector", value);
        }
        if let Some(value) = self.limit {
            __query_pairs.append_pair("limit", &value.to_string());
        }
        if let Some(value) = self.resource_version {
            __query_pairs.append_pair("resourceVersion", value);
        }
        if let Some(value) = self.resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", value);
        }
        if let Some(value) = self.timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &value.to_string());
        }
    }
}
