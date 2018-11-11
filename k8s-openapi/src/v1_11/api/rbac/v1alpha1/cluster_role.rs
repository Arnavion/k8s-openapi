// Generated from definition io.k8s.api.rbac.v1alpha1.ClusterRole

/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRole {
    /// AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
    pub aggregation_rule: Option<::v1_11::api::rbac::v1alpha1::AggregationRule>,

    /// Standard object's metadata.
    pub metadata: Option<::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Rules holds all the PolicyRules for this ClusterRole
    pub rules: Vec<::v1_11::api::rbac::v1alpha1::PolicyRule>,
}

// Begin rbac.authorization.k8s.io/v1alpha1/ClusterRole

// Generated from operation createRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// create a ClusterRole
    ///
    /// Use [`CreateRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.CreateRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_rbac_authorization_v1alpha1_cluster_role(
        body: &::v1_11::api::rbac::v1alpha1::ClusterRole,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::create_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.create_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum CreateRbacAuthorizationV1alpha1ClusterRoleResponse {
    Ok(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Created(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Accepted(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Unauthorized,
    Other,
}

impl ::Response for CreateRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1alpha1ClusterRoleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1alpha1ClusterRoleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::ACCEPTED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1alpha1ClusterRoleResponse::Accepted(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((CreateRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// delete a ClusterRole
    ///
    /// Use [`DeleteRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.DeleteRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
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
    pub fn delete_rbac_authorization_v1alpha1_cluster_role(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::delete_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.delete_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1alpha1ClusterRoleResponse {
    OkStatus(::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Unauthorized,
    Other,
}

impl ::Response for DeleteRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result: ::serde_json::Map<String, ::serde_json::Value> = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(::serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1CollectionClusterRole

impl ClusterRole {
    /// delete collection of ClusterRole
    ///
    /// Use [`DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse`](./enum.DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse.html) to parse the HTTP response.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn delete_rbac_authorization_v1alpha1_collection_cluster_role(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::delete_rbac_authorization_v1alpha1_collection_cluster_role`](./struct.ClusterRole.html#method.delete_rbac_authorization_v1alpha1_collection_cluster_role)
#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse {
    OkStatus(::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Unauthorized,
    Other,
}

impl ::Response for DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result: ::serde_json::Map<String, ::serde_json::Value> = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(::serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// list or watch objects of kind ClusterRole
    ///
    /// Use [`ListRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.ListRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_rbac_authorization_v1alpha1_cluster_role(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::list_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.list_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum ListRbacAuthorizationV1alpha1ClusterRoleResponse {
    Ok(::v1_11::api::rbac::v1alpha1::ClusterRoleList),
    Unauthorized,
    Other,
}

impl ::Response for ListRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListRbacAuthorizationV1alpha1ClusterRoleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((ListRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// partially update the specified ClusterRole
    ///
    /// Use [`PatchRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.PatchRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_rbac_authorization_v1alpha1_cluster_role(
        name: &str,
        body: &::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::patch_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.patch_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum PatchRbacAuthorizationV1alpha1ClusterRoleResponse {
    Ok(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Unauthorized,
    Other,
}

impl ::Response for PatchRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchRbacAuthorizationV1alpha1ClusterRoleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((PatchRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation readRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// read the specified ClusterRole
    ///
    /// Use [`ReadRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.ReadRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_rbac_authorization_v1alpha1_cluster_role(
        name: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::read_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.read_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum ReadRbacAuthorizationV1alpha1ClusterRoleResponse {
    Ok(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Unauthorized,
    Other,
}

impl ::Response for ReadRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadRbacAuthorizationV1alpha1ClusterRoleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((ReadRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// replace the specified ClusterRole
    ///
    /// Use [`ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_rbac_authorization_v1alpha1_cluster_role(
        name: &str,
        body: &::v1_11::api::rbac::v1alpha1::ClusterRole,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::replace_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.replace_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse {
    Ok(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Created(::v1_11::api::rbac::v1alpha1::ClusterRole),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRole

impl ClusterRole {
    /// watch changes to an object of kind ClusterRole
    ///
    /// Use [`WatchRbacAuthorizationV1alpha1ClusterRoleResponse`](./enum.WatchRbacAuthorizationV1alpha1ClusterRoleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_rbac_authorization_v1alpha1_cluster_role(
        name: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterroles/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::watch_rbac_authorization_v1alpha1_cluster_role`](./struct.ClusterRole.html#method.watch_rbac_authorization_v1alpha1_cluster_role)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1alpha1ClusterRoleResponse {
    Ok(::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchRbacAuthorizationV1alpha1ClusterRoleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                Ok((WatchRbacAuthorizationV1alpha1ClusterRoleResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRoleList

impl ClusterRole {
    /// watch individual changes to a list of ClusterRole
    ///
    /// Use [`WatchRbacAuthorizationV1alpha1ClusterRoleListResponse`](./enum.WatchRbacAuthorizationV1alpha1ClusterRoleListResponse.html) to parse the HTTP response.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_rbac_authorization_v1alpha1_cluster_role_list(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterroles?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ClusterRole::watch_rbac_authorization_v1alpha1_cluster_role_list`](./struct.ClusterRole.html#method.watch_rbac_authorization_v1alpha1_cluster_role_list)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1alpha1ClusterRoleListResponse {
    Ok(::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchRbacAuthorizationV1alpha1ClusterRoleListResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                Ok((WatchRbacAuthorizationV1alpha1ClusterRoleListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleListResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleListResponse::Other, 0)),
        }
    }
}

// End rbac.authorization.k8s.io/v1alpha1/ClusterRole

impl ::Resource for ClusterRole {
    fn api_version() -> &'static str {
        "rbac.authorization.k8s.io/v1alpha1"
    }

    fn group() -> &'static str {
        "rbac.authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "ClusterRole"
    }

    fn version() -> &'static str {
        "v1alpha1"
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_aggregation_rule,
            Key_metadata,
            Key_rules,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "aggregationRule" => Field::Key_aggregation_rule,
                            "metadata" => Field::Key_metadata,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRole;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ClusterRole")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_aggregation_rule: Option<::v1_11::api::rbac::v1alpha1::AggregationRule> = None;
                let mut value_metadata: Option<::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_rules: Option<Vec<::v1_11::api::rbac::v1alpha1::PolicyRule>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as ::Resource>::api_version() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as ::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as ::Resource>::kind() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_kind), &<Self::Value as ::Resource>::kind()));
                            }
                        },
                        Field::Key_aggregation_rule => value_aggregation_rule = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRole {
                    aggregation_rule: value_aggregation_rule,
                    metadata: value_metadata,
                    rules: value_rules.ok_or_else(|| ::serde::de::Error::missing_field("rules"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterRole",
            &[
                "apiVersion",
                "kind",
                "aggregationRule",
                "metadata",
                "rules",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ClusterRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterRole",
            0 +
            2 +
            self.aggregation_rule.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
        if let Some(value) = &self.aggregation_rule {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "aggregationRule", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
