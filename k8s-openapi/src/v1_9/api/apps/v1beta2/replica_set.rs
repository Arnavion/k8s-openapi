// Generated from definition io.k8s.api.apps.v1beta2.ReplicaSet

/// DEPRECATED - This group version of ReplicaSet is deprecated by apps/v1/ReplicaSet. See the release notes for more information. ReplicaSet ensures that a specified number of pod replicas are running at any given time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicaSet {
    /// If the Labels of a ReplicaSet are empty, they are defaulted to be the same as the Pod(s) that the ReplicaSet manages. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the specification of the desired behavior of the ReplicaSet. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_9::api::apps::v1beta2::ReplicaSetSpec>,

    /// Status is the most recently observed status of the ReplicaSet. This data may be out of date by some window of time. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_9::api::apps::v1beta2::ReplicaSetStatus>,
}

// Begin apps/v1beta2/ReplicaSet

// Generated from operation createAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// create a ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedReplicaSetResponse`]`>` constructor, or [`CreateNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_replica_set(
        namespace: &str,
        body: &crate::v1_9::api::apps::v1beta2::ReplicaSet,
        optional: CreateNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedReplicaSetResponse>), crate::RequestError> {
        let CreateNamespacedReplicaSetOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::create_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedReplicaSetOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::create_namespaced_replica_set`]
#[derive(Debug)]
pub enum CreateNamespacedReplicaSetResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Created(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Accepted(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedReplicaSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedReplicaSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedReplicaSetResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedReplicaSetResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAppsV1beta2CollectionNamespacedReplicaSet

impl ReplicaSet {
    /// delete collection of ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedReplicaSetResponse`]`>` constructor, or [`DeleteCollectionNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_namespaced_replica_set(
        namespace: &str,
        optional: DeleteCollectionNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedReplicaSetResponse>), crate::RequestError> {
        let DeleteCollectionNamespacedReplicaSetOptional {
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
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets?", namespace = namespace);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::delete_collection_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionNamespacedReplicaSetOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<DeleteCollectionNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::delete_collection_namespaced_replica_set`]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedReplicaSetResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNamespacedReplicaSetResponse {
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
                    Ok((DeleteCollectionNamespacedReplicaSetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedReplicaSetResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// delete a ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedReplicaSetResponse`]`>` constructor, or [`DeleteNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_replica_set(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedReplicaSetResponse>), crate::RequestError> {
        let DeleteNamespacedReplicaSetOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}?", name = name, namespace = namespace);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::delete_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespacedReplicaSetOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::delete_namespaced_replica_set`]
#[derive(Debug)]
pub enum DeleteNamespacedReplicaSetResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespacedReplicaSetResponse {
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
                    Ok((DeleteNamespacedReplicaSetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedReplicaSetResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation listAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// list or watch objects of kind ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedReplicaSetResponse`]`>` constructor, or [`ListNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_replica_set(
        namespace: &str,
        optional: ListNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedReplicaSetResponse>), crate::RequestError> {
        let ListNamespacedReplicaSetOptional {
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
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets?", namespace = namespace);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::list_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListNamespacedReplicaSetOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<ListNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::list_namespaced_replica_set`]
#[derive(Debug)]
pub enum ListNamespacedReplicaSetResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSetList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespacedReplicaSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedReplicaSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((ListNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation listAppsV1beta2ReplicaSetForAllNamespaces

impl ReplicaSet {
    /// list or watch objects of kind ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListReplicaSetForAllNamespacesResponse`]`>` constructor, or [`ListReplicaSetForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_replica_set_for_all_namespaces(
        optional: ListReplicaSetForAllNamespacesOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListReplicaSetForAllNamespacesResponse>), crate::RequestError> {
        let ListReplicaSetForAllNamespacesOptional {
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
        let __url = "/apis/apps/v1beta2/replicasets?".to_string();
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::list_replica_set_for_all_namespaces`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListReplicaSetForAllNamespacesOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<ListReplicaSetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::list_replica_set_for_all_namespaces`]
#[derive(Debug)]
pub enum ListReplicaSetForAllNamespacesResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSetList),
    Unauthorized,
    Other,
}

impl crate::Response for ListReplicaSetForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListReplicaSetForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListReplicaSetForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListReplicaSetForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// partially update the specified ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedReplicaSetResponse`]`>` constructor, or [`PatchNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
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
    pub fn patch_namespaced_replica_set(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedReplicaSetResponse>), crate::RequestError> {
        let PatchNamespacedReplicaSetOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::patch_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedReplicaSetOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::patch_namespaced_replica_set`]
#[derive(Debug)]
pub enum PatchNamespacedReplicaSetResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedReplicaSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedReplicaSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAppsV1beta2NamespacedReplicaSetStatus

impl ReplicaSet {
    /// partially update status of the specified ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedReplicaSetStatusResponse`]`>` constructor, or [`PatchNamespacedReplicaSetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
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
    pub fn patch_namespaced_replica_set_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedReplicaSetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedReplicaSetStatusResponse>), crate::RequestError> {
        let PatchNamespacedReplicaSetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::patch_namespaced_replica_set_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedReplicaSetStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedReplicaSetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::patch_namespaced_replica_set_status`]
#[derive(Debug)]
pub enum PatchNamespacedReplicaSetStatusResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedReplicaSetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedReplicaSetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedReplicaSetStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedReplicaSetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// read the specified ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedReplicaSetResponse`]`>` constructor, or [`ReadNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_replica_set(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedReplicaSetResponse>), crate::RequestError> {
        let ReadNamespacedReplicaSetOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}?", name = name, namespace = namespace);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::read_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedReplicaSetOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::read_namespaced_replica_set`]
#[derive(Debug)]
pub enum ReadNamespacedReplicaSetResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedReplicaSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedReplicaSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1beta2NamespacedReplicaSetStatus

impl ReplicaSet {
    /// read status of the specified ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedReplicaSetStatusResponse`]`>` constructor, or [`ReadNamespacedReplicaSetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_replica_set_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedReplicaSetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedReplicaSetStatusResponse>), crate::RequestError> {
        let ReadNamespacedReplicaSetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::read_namespaced_replica_set_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedReplicaSetStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedReplicaSetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::read_namespaced_replica_set_status`]
#[derive(Debug)]
pub enum ReadNamespacedReplicaSetStatusResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedReplicaSetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedReplicaSetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedReplicaSetStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedReplicaSetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// replace the specified ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedReplicaSetResponse`]`>` constructor, or [`ReplaceNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
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
    pub fn replace_namespaced_replica_set(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::api::apps::v1beta2::ReplicaSet,
        optional: ReplaceNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedReplicaSetResponse>), crate::RequestError> {
        let ReplaceNamespacedReplicaSetOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::replace_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedReplicaSetOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::replace_namespaced_replica_set`]
#[derive(Debug)]
pub enum ReplaceNamespacedReplicaSetResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Created(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedReplicaSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedReplicaSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedReplicaSetResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1beta2NamespacedReplicaSetStatus

impl ReplicaSet {
    /// replace status of the specified ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedReplicaSetStatusResponse`]`>` constructor, or [`ReplaceNamespacedReplicaSetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
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
    pub fn replace_namespaced_replica_set_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_9::api::apps::v1beta2::ReplicaSet,
        optional: ReplaceNamespacedReplicaSetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedReplicaSetStatusResponse>), crate::RequestError> {
        let ReplaceNamespacedReplicaSetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::replace_namespaced_replica_set_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedReplicaSetStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedReplicaSetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::replace_namespaced_replica_set_status`]
#[derive(Debug)]
pub enum ReplaceNamespacedReplicaSetStatusResponse {
    Ok(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Created(crate::v1_9::api::apps::v1beta2::ReplicaSet),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedReplicaSetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedReplicaSetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedReplicaSetStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedReplicaSetStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedReplicaSetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAppsV1beta2NamespacedReplicaSet

impl ReplicaSet {
    /// watch changes to an object of kind ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedReplicaSetResponse`]`>` constructor, or [`WatchNamespacedReplicaSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ReplicaSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_replica_set(
        name: &str,
        namespace: &str,
        optional: WatchNamespacedReplicaSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedReplicaSetResponse>), crate::RequestError> {
        let WatchNamespacedReplicaSetOptional {
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
        let __url = format!("/apis/apps/v1beta2/watch/namespaces/{namespace}/replicasets/{name}?", name = name, namespace = namespace);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::watch_namespaced_replica_set`]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchNamespacedReplicaSetOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchNamespacedReplicaSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::watch_namespaced_replica_set`]
#[derive(Debug)]
pub enum WatchNamespacedReplicaSetResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedReplicaSetResponse {
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
                Ok((WatchNamespacedReplicaSetResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedReplicaSetResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedReplicaSetResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAppsV1beta2NamespacedReplicaSetList

impl ReplicaSet {
    /// watch individual changes to a list of ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedReplicaSetListResponse`]`>` constructor, or [`WatchNamespacedReplicaSetListResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_replica_set_list(
        namespace: &str,
        optional: WatchNamespacedReplicaSetListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedReplicaSetListResponse>), crate::RequestError> {
        let WatchNamespacedReplicaSetListOptional {
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
        let __url = format!("/apis/apps/v1beta2/watch/namespaces/{namespace}/replicasets?", namespace = namespace);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::watch_namespaced_replica_set_list`]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchNamespacedReplicaSetListOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchNamespacedReplicaSetListResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::watch_namespaced_replica_set_list`]
#[derive(Debug)]
pub enum WatchNamespacedReplicaSetListResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedReplicaSetListResponse {
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
                Ok((WatchNamespacedReplicaSetListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedReplicaSetListResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedReplicaSetListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAppsV1beta2ReplicaSetListForAllNamespaces

impl ReplicaSet {
    /// watch individual changes to a list of ReplicaSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchReplicaSetListForAllNamespacesResponse`]`>` constructor, or [`WatchReplicaSetListForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_replica_set_list_for_all_namespaces(
        optional: WatchReplicaSetListForAllNamespacesOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchReplicaSetListForAllNamespacesResponse>), crate::RequestError> {
        let WatchReplicaSetListForAllNamespacesOptional {
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
        let __url = "/apis/apps/v1beta2/watch/replicasets?".to_string();
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ReplicaSet::watch_replica_set_list_for_all_namespaces`]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchReplicaSetListForAllNamespacesOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchReplicaSetListForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`ReplicaSet::watch_replica_set_list_for_all_namespaces`]
#[derive(Debug)]
pub enum WatchReplicaSetListForAllNamespacesResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchReplicaSetListForAllNamespacesResponse {
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
                Ok((WatchReplicaSetListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchReplicaSetListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchReplicaSetListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End apps/v1beta2/ReplicaSet

impl crate::Resource for ReplicaSet {
    fn api_version() -> &'static str {
        "apps/v1beta2"
    }

    fn group() -> &'static str {
        "apps"
    }

    fn kind() -> &'static str {
        "ReplicaSet"
    }

    fn version() -> &'static str {
        "v1beta2"
    }
}

impl crate::Metadata for ReplicaSet {
    type Ty = crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ReplicaSet {
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
            type Value = ReplicaSet;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ReplicaSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_9::api::apps::v1beta2::ReplicaSetSpec> = None;
                let mut value_status: Option<crate::v1_9::api::apps::v1beta2::ReplicaSetStatus> = None;

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

                Ok(ReplicaSet {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "ReplicaSet",
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

impl serde::Serialize for ReplicaSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicaSet",
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
