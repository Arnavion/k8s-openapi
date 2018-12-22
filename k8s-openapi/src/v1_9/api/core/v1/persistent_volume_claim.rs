// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaim

/// PersistentVolumeClaim is a user's request for and claim to a persistent volume
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaim {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the desired characteristics of a volume requested by a pod author. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub spec: Option<crate::v1_9::api::core::v1::PersistentVolumeClaimSpec>,

    /// Status represents the current information/status of a persistent volume claim. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub status: Option<crate::v1_9::api::core::v1::PersistentVolumeClaimStatus>,
}

// Begin /v1/PersistentVolumeClaim

// Generated from operation createCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// create a PersistentVolumeClaim
    ///
    /// Use [`CreateCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.CreateCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_core_v1_namespaced_persistent_volume_claim(
        namespace: &str,
        body: &crate::v1_9::api::core::v1::PersistentVolumeClaim,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::create_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.create_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum CreateCoreV1NamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Created(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Accepted(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedPersistentVolumeClaimResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedPersistentVolumeClaimResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((CreateCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// delete collection of PersistentVolumeClaim
    ///
    /// Use [`DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse`](./enum.DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn delete_core_v1_collection_namespaced_persistent_volume_claim(
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::delete_core_v1_collection_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.delete_core_v1_collection_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1CollectionNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// delete a PersistentVolumeClaim
    ///
    /// Use [`DeleteCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.DeleteCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `grace_period_seconds`
    ///
    ///     The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    ///
    /// * `orphan_dependents`
    ///
    ///     Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `propagation_policy`
    ///
    ///     Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub fn delete_core_v1_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::delete_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.delete_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum DeleteCoreV1NamespacedPersistentVolumeClaimResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1NamespacedPersistentVolumeClaimResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1NamespacedPersistentVolumeClaimResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// list or watch objects of kind PersistentVolumeClaim
    ///
    /// Use [`ListCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.ListCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_core_v1_namespaced_persistent_volume_claim(
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::list_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.list_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum ListCoreV1NamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaimList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1NamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1PersistentVolumeClaimForAllNamespaces

impl PersistentVolumeClaim {
    /// list or watch objects of kind PersistentVolumeClaim
    ///
    /// Use [`ListCoreV1PersistentVolumeClaimForAllNamespacesResponse`](./enum.ListCoreV1PersistentVolumeClaimForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_core_v1_persistent_volume_claim_for_all_namespaces(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/persistentvolumeclaims?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::list_core_v1_persistent_volume_claim_for_all_namespaces`](./struct.PersistentVolumeClaim.html#method.list_core_v1_persistent_volume_claim_for_all_namespaces)
#[derive(Debug)]
pub enum ListCoreV1PersistentVolumeClaimForAllNamespacesResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaimList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1PersistentVolumeClaimForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1PersistentVolumeClaimForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1PersistentVolumeClaimForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1PersistentVolumeClaimForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// partially update the specified PersistentVolumeClaim
    ///
    /// Use [`PatchCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.PatchCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_core_v1_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::patch_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.patch_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPersistentVolumeClaimStatus

impl PersistentVolumeClaim {
    /// partially update status of the specified PersistentVolumeClaim
    ///
    /// Use [`PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse`](./enum.PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_core_v1_namespaced_persistent_volume_claim_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::patch_core_v1_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.patch_core_v1_namespaced_persistent_volume_claim_status)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedPersistentVolumeClaimStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// read the specified PersistentVolumeClaim
    ///
    /// Use [`ReadCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.ReadCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `exact`
    ///
    ///     Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    ///
    /// * `export`
    ///
    ///     Should this value be exported.  Export strips fields that a user can not specify.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_core_v1_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::read_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.read_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPersistentVolumeClaimStatus

impl PersistentVolumeClaim {
    /// read status of the specified PersistentVolumeClaim
    ///
    /// Use [`ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse`](./enum.ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_core_v1_namespaced_persistent_volume_claim_status(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::read_core_v1_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.read_core_v1_namespaced_persistent_volume_claim_status)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedPersistentVolumeClaimStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// replace the specified PersistentVolumeClaim
    ///
    /// Use [`ReplaceCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.ReplaceCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_core_v1_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::api::core::v1::PersistentVolumeClaim,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::replace_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.replace_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Created(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPersistentVolumeClaimStatus

impl PersistentVolumeClaim {
    /// replace status of the specified PersistentVolumeClaim
    ///
    /// Use [`ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse`](./enum.ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_core_v1_namespaced_persistent_volume_claim_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::api::core::v1::PersistentVolumeClaim,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::replace_core_v1_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.replace_core_v1_namespaced_persistent_volume_claim_status)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse {
    Ok(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Created(crate::v1_9::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedPersistentVolumeClaimStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// watch changes to an object of kind PersistentVolumeClaim
    ///
    /// Use [`WatchCoreV1NamespacedPersistentVolumeClaimResponse`](./enum.WatchCoreV1NamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PersistentVolumeClaim
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_core_v1_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/persistentvolumeclaims/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::watch_core_v1_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.watch_core_v1_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchCoreV1NamespacedPersistentVolumeClaimResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPersistentVolumeClaimList

impl PersistentVolumeClaim {
    /// watch individual changes to a list of PersistentVolumeClaim
    ///
    /// Use [`WatchCoreV1NamespacedPersistentVolumeClaimListResponse`](./enum.WatchCoreV1NamespacedPersistentVolumeClaimListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_core_v1_namespaced_persistent_volume_claim_list(
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/persistentvolumeclaims?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::watch_core_v1_namespaced_persistent_volume_claim_list`](./struct.PersistentVolumeClaim.html#method.watch_core_v1_namespaced_persistent_volume_claim_list)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedPersistentVolumeClaimListResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NamespacedPersistentVolumeClaimListResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchCoreV1NamespacedPersistentVolumeClaimListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedPersistentVolumeClaimListResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedPersistentVolumeClaimListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1PersistentVolumeClaimListForAllNamespaces

impl PersistentVolumeClaim {
    /// watch individual changes to a list of PersistentVolumeClaim
    ///
    /// Use [`WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse`](./enum.WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_core_v1_persistent_volume_claim_list_for_all_namespaces(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/watch/persistentvolumeclaims?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PersistentVolumeClaim::watch_core_v1_persistent_volume_claim_list_for_all_namespaces`](./struct.PersistentVolumeClaim.html#method.watch_core_v1_persistent_volume_claim_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1PersistentVolumeClaimListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End /v1/PersistentVolumeClaim

impl crate::Resource for PersistentVolumeClaim {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "PersistentVolumeClaim"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for PersistentVolumeClaim {
    type Ty = crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&Self::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for PersistentVolumeClaim {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaim;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct PersistentVolumeClaim")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_9::api::core::v1::PersistentVolumeClaimSpec> = None;
                let mut value_status: Option<crate::v1_9::api::core::v1::PersistentVolumeClaimStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaim {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaim",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PersistentVolumeClaim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaim",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
