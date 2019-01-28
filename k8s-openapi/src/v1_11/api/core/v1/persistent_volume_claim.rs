// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaim

/// PersistentVolumeClaim is a user's request for and claim to a persistent volume
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaim {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the desired characteristics of a volume requested by a pod author. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub spec: Option<crate::v1_11::api::core::v1::PersistentVolumeClaimSpec>,

    /// Status represents the current information/status of a persistent volume claim. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub status: Option<crate::v1_11::api::core::v1::PersistentVolumeClaimStatus>,
}

// Begin /v1/PersistentVolumeClaim

// Generated from operation createCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// create a PersistentVolumeClaim
    ///
    /// Use [`CreateNamespacedPersistentVolumeClaimResponse`](./enum.CreateNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_namespaced_persistent_volume_claim(
        namespace: &str,
        body: &crate::v1_11::api::core::v1::PersistentVolumeClaim,
        optional: CreateNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateNamespacedPersistentVolumeClaimOptional {
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::create_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.create_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct CreateNamespacedPersistentVolumeClaimOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::create_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.create_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum CreateNamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Created(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Accepted(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPersistentVolumeClaimResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPersistentVolumeClaimResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// delete collection of PersistentVolumeClaim
    ///
    /// Use [`DeleteCollectionNamespacedPersistentVolumeClaimResponse`](./enum.DeleteCollectionNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_namespaced_persistent_volume_claim(
        namespace: &str,
        optional: DeleteCollectionNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCollectionNamespacedPersistentVolumeClaimOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::delete_collection_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.delete_collection_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct DeleteCollectionNamespacedPersistentVolumeClaimOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::delete_collection_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.delete_collection_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum DeleteCollectionNamespacedPersistentVolumeClaimResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNamespacedPersistentVolumeClaimResponse {
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
                    Ok((DeleteCollectionNamespacedPersistentVolumeClaimResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedPersistentVolumeClaimResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// delete a PersistentVolumeClaim
    ///
    /// Use [`DeleteNamespacedPersistentVolumeClaimResponse`](./enum.DeleteNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteNamespacedPersistentVolumeClaimOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::delete_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.delete_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct DeleteNamespacedPersistentVolumeClaimOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::delete_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.delete_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum DeleteNamespacedPersistentVolumeClaimResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespacedPersistentVolumeClaimResponse {
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
                    Ok((DeleteNamespacedPersistentVolumeClaimResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedPersistentVolumeClaimResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// list or watch objects of kind PersistentVolumeClaim
    ///
    /// Use [`ListNamespacedPersistentVolumeClaimResponse`](./enum.ListNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_namespaced_persistent_volume_claim(
        namespace: &str,
        optional: ListNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListNamespacedPersistentVolumeClaimOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::list_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.list_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct ListNamespacedPersistentVolumeClaimOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::list_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.list_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum ListNamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaimList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((ListNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1PersistentVolumeClaimForAllNamespaces

impl PersistentVolumeClaim {
    /// list or watch objects of kind PersistentVolumeClaim
    ///
    /// Use [`ListPersistentVolumeClaimForAllNamespacesResponse`](./enum.ListPersistentVolumeClaimForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_persistent_volume_claim_for_all_namespaces(
        optional: ListPersistentVolumeClaimForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListPersistentVolumeClaimForAllNamespacesOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::list_persistent_volume_claim_for_all_namespaces`](./struct.PersistentVolumeClaim.html#method.list_persistent_volume_claim_for_all_namespaces)
#[derive(Debug, Default)]
pub struct ListPersistentVolumeClaimForAllNamespacesOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::list_persistent_volume_claim_for_all_namespaces`](./struct.PersistentVolumeClaim.html#method.list_persistent_volume_claim_for_all_namespaces)
#[derive(Debug)]
pub enum ListPersistentVolumeClaimForAllNamespacesResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaimList),
    Unauthorized,
    Other,
}

impl crate::Response for ListPersistentVolumeClaimForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListPersistentVolumeClaimForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListPersistentVolumeClaimForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListPersistentVolumeClaimForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// partially update the specified PersistentVolumeClaim
    ///
    /// Use [`PatchNamespacedPersistentVolumeClaimResponse`](./enum.PatchNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedPersistentVolumeClaimOptional {
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::patch_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.patch_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct PatchNamespacedPersistentVolumeClaimOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::patch_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.patch_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum PatchNamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPersistentVolumeClaimStatus

impl PersistentVolumeClaim {
    /// partially update status of the specified PersistentVolumeClaim
    ///
    /// Use [`PatchNamespacedPersistentVolumeClaimStatusResponse`](./enum.PatchNamespacedPersistentVolumeClaimStatusResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_persistent_volume_claim_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPersistentVolumeClaimStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedPersistentVolumeClaimStatusOptional {
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::patch_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.patch_namespaced_persistent_volume_claim_status)
#[derive(Debug, Default)]
pub struct PatchNamespacedPersistentVolumeClaimStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::patch_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.patch_namespaced_persistent_volume_claim_status)
#[derive(Debug)]
pub enum PatchNamespacedPersistentVolumeClaimStatusResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedPersistentVolumeClaimStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPersistentVolumeClaimStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedPersistentVolumeClaimStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedPersistentVolumeClaimStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// read the specified PersistentVolumeClaim
    ///
    /// Use [`ReadNamespacedPersistentVolumeClaimResponse`](./enum.ReadNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedPersistentVolumeClaimOptional {
            exact,
            export,
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::read_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.read_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct ReadNamespacedPersistentVolumeClaimOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::read_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.read_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum ReadNamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPersistentVolumeClaimStatus

impl PersistentVolumeClaim {
    /// read status of the specified PersistentVolumeClaim
    ///
    /// Use [`ReadNamespacedPersistentVolumeClaimStatusResponse`](./enum.ReadNamespacedPersistentVolumeClaimStatusResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_persistent_volume_claim_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPersistentVolumeClaimStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedPersistentVolumeClaimStatusOptional {
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::read_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.read_namespaced_persistent_volume_claim_status)
#[derive(Debug, Default)]
pub struct ReadNamespacedPersistentVolumeClaimStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::read_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.read_namespaced_persistent_volume_claim_status)
#[derive(Debug)]
pub enum ReadNamespacedPersistentVolumeClaimStatusResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPersistentVolumeClaimStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPersistentVolumeClaimStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPersistentVolumeClaimStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPersistentVolumeClaimStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// replace the specified PersistentVolumeClaim
    ///
    /// Use [`ReplaceNamespacedPersistentVolumeClaimResponse`](./enum.ReplaceNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::api::core::v1::PersistentVolumeClaim,
        optional: ReplaceNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedPersistentVolumeClaimOptional {
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::replace_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.replace_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct ReplaceNamespacedPersistentVolumeClaimOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::replace_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.replace_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum ReplaceNamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Created(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedPersistentVolumeClaimResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPersistentVolumeClaimResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPersistentVolumeClaimResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPersistentVolumeClaimStatus

impl PersistentVolumeClaim {
    /// replace status of the specified PersistentVolumeClaim
    ///
    /// Use [`ReplaceNamespacedPersistentVolumeClaimStatusResponse`](./enum.ReplaceNamespacedPersistentVolumeClaimStatusResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_persistent_volume_claim_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::api::core::v1::PersistentVolumeClaim,
        optional: ReplaceNamespacedPersistentVolumeClaimStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedPersistentVolumeClaimStatusOptional {
            pretty,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::replace_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.replace_namespaced_persistent_volume_claim_status)
#[derive(Debug, Default)]
pub struct ReplaceNamespacedPersistentVolumeClaimStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::replace_namespaced_persistent_volume_claim_status`](./struct.PersistentVolumeClaim.html#method.replace_namespaced_persistent_volume_claim_status)
#[derive(Debug)]
pub enum ReplaceNamespacedPersistentVolumeClaimStatusResponse {
    Ok(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Created(crate::v1_11::api::core::v1::PersistentVolumeClaim),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedPersistentVolumeClaimStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPersistentVolumeClaimStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPersistentVolumeClaimStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedPersistentVolumeClaimStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedPersistentVolumeClaimStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPersistentVolumeClaim

impl PersistentVolumeClaim {
    /// watch changes to an object of kind PersistentVolumeClaim
    ///
    /// Use [`WatchNamespacedPersistentVolumeClaimResponse`](./enum.WatchNamespacedPersistentVolumeClaimResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_persistent_volume_claim(
        name: &str,
        namespace: &str,
        optional: WatchNamespacedPersistentVolumeClaimOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedPersistentVolumeClaimOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::watch_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.watch_namespaced_persistent_volume_claim)
#[derive(Debug, Default)]
pub struct WatchNamespacedPersistentVolumeClaimOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::watch_namespaced_persistent_volume_claim`](./struct.PersistentVolumeClaim.html#method.watch_namespaced_persistent_volume_claim)
#[derive(Debug)]
pub enum WatchNamespacedPersistentVolumeClaimResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedPersistentVolumeClaimResponse {
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
                Ok((WatchNamespacedPersistentVolumeClaimResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedPersistentVolumeClaimResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedPersistentVolumeClaimResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPersistentVolumeClaimList

impl PersistentVolumeClaim {
    /// watch individual changes to a list of PersistentVolumeClaim
    ///
    /// Use [`WatchNamespacedPersistentVolumeClaimListResponse`](./enum.WatchNamespacedPersistentVolumeClaimListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_persistent_volume_claim_list(
        namespace: &str,
        optional: WatchNamespacedPersistentVolumeClaimListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedPersistentVolumeClaimListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::watch_namespaced_persistent_volume_claim_list`](./struct.PersistentVolumeClaim.html#method.watch_namespaced_persistent_volume_claim_list)
#[derive(Debug, Default)]
pub struct WatchNamespacedPersistentVolumeClaimListOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::watch_namespaced_persistent_volume_claim_list`](./struct.PersistentVolumeClaim.html#method.watch_namespaced_persistent_volume_claim_list)
#[derive(Debug)]
pub enum WatchNamespacedPersistentVolumeClaimListResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedPersistentVolumeClaimListResponse {
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
                Ok((WatchNamespacedPersistentVolumeClaimListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedPersistentVolumeClaimListResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedPersistentVolumeClaimListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1PersistentVolumeClaimListForAllNamespaces

impl PersistentVolumeClaim {
    /// watch individual changes to a list of PersistentVolumeClaim
    ///
    /// Use [`WatchPersistentVolumeClaimListForAllNamespacesResponse`](./enum.WatchPersistentVolumeClaimListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_persistent_volume_claim_list_for_all_namespaces(
        optional: WatchPersistentVolumeClaimListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchPersistentVolumeClaimListForAllNamespacesOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
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

/// Optional parameters of [`PersistentVolumeClaim::watch_persistent_volume_claim_list_for_all_namespaces`](./struct.PersistentVolumeClaim.html#method.watch_persistent_volume_claim_list_for_all_namespaces)
#[derive(Debug, Default)]
pub struct WatchPersistentVolumeClaimListForAllNamespacesOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`PersistentVolumeClaim::watch_persistent_volume_claim_list_for_all_namespaces`](./struct.PersistentVolumeClaim.html#method.watch_persistent_volume_claim_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchPersistentVolumeClaimListForAllNamespacesResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchPersistentVolumeClaimListForAllNamespacesResponse {
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
                Ok((WatchPersistentVolumeClaimListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchPersistentVolumeClaimListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchPersistentVolumeClaimListForAllNamespacesResponse::Other, 0)),
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
    type Ty = crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
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
                let mut value_metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_11::api::core::v1::PersistentVolumeClaimSpec> = None;
                let mut value_status: Option<crate::v1_11::api::core::v1::PersistentVolumeClaimStatus> = None;

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
