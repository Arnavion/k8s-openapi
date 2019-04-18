// Generated from definition io.k8s.api.rbac.v1beta1.ClusterRole

/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRole {
    /// AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
    pub aggregation_rule: Option<crate::v1_9::api::rbac::v1beta1::AggregationRule>,

    /// Standard object's metadata.
    pub metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Rules holds all the PolicyRules for this ClusterRole
    pub rules: Vec<crate::v1_9::api::rbac::v1beta1::PolicyRule>,
}

// Begin rbac.authorization.k8s.io/v1beta1/ClusterRole

// Generated from operation createRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// create a ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateClusterRoleResponse`]`>` constructor, or [`CreateClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_cluster_role(
        body: &crate::v1_9::api::rbac::v1beta1::ClusterRole,
        optional: CreateClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateClusterRoleResponse>), crate::RequestError> {
        let CreateClusterRoleOptional {
            pretty,
        } = optional;
        let __url = "/apis/rbac.authorization.k8s.io/v1beta1/clusterroles?".to_string();
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

/// Optional parameters of [`ClusterRole::create_cluster_role`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateClusterRoleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::create_cluster_role`]
#[derive(Debug)]
pub enum CreateClusterRoleResponse {
    Ok(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Created(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Accepted(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Unauthorized,
    Other,
}

impl crate::Response for CreateClusterRoleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateClusterRoleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateClusterRoleResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateClusterRoleResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((CreateClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// delete a ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteClusterRoleResponse`]`>` constructor, or [`DeleteClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_cluster_role(
        name: &str,
        optional: DeleteClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteClusterRoleResponse>), crate::RequestError> {
        let DeleteClusterRoleOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name}?", name = name);
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

/// Optional parameters of [`ClusterRole::delete_cluster_role`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteClusterRoleOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::delete_cluster_role`]
#[derive(Debug)]
pub enum DeleteClusterRoleResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteClusterRoleResponse {
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
                    Ok((DeleteClusterRoleResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteClusterRoleResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((DeleteClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1beta1CollectionClusterRole

impl ClusterRole {
    /// delete collection of ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionClusterRoleResponse`]`>` constructor, or [`DeleteCollectionClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_cluster_role(
        optional: DeleteCollectionClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionClusterRoleResponse>), crate::RequestError> {
        let DeleteCollectionClusterRoleOptional {
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
        let __url = "/apis/rbac.authorization.k8s.io/v1beta1/clusterroles?".to_string();
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

/// Optional parameters of [`ClusterRole::delete_collection_cluster_role`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionClusterRoleOptional<'a> {
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

/// Use `<DeleteCollectionClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::delete_collection_cluster_role`]
#[derive(Debug)]
pub enum DeleteCollectionClusterRoleResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionClusterRoleResponse {
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
                    Ok((DeleteCollectionClusterRoleResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionClusterRoleResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// list or watch objects of kind ClusterRole
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListClusterRoleResponse`]`>` constructor, or [`ListClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_cluster_role(
        optional: crate::v1_9::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListClusterRoleResponse>), crate::RequestError> {
        let crate::v1_9::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/rbac.authorization.k8s.io/v1beta1/clusterroles?".to_string();
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
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ListClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::list_cluster_role`]
#[derive(Debug)]
pub enum ListClusterRoleResponse {
    Ok(crate::v1_9::api::rbac::v1beta1::ClusterRoleList),
    Unauthorized,
    Other,
}

impl crate::Response for ListClusterRoleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListClusterRoleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((ListClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// partially update the specified ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchClusterRoleResponse`]`>` constructor, or [`PatchClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_cluster_role(
        name: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchClusterRoleResponse>), crate::RequestError> {
        let PatchClusterRoleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name}?", name = name);
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

/// Optional parameters of [`ClusterRole::patch_cluster_role`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchClusterRoleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::patch_cluster_role`]
#[derive(Debug)]
pub enum PatchClusterRoleResponse {
    Ok(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Unauthorized,
    Other,
}

impl crate::Response for PatchClusterRoleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchClusterRoleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((PatchClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation readRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// read the specified ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadClusterRoleResponse`]`>` constructor, or [`ReadClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_cluster_role(
        name: &str,
        optional: ReadClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadClusterRoleResponse>), crate::RequestError> {
        let ReadClusterRoleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name}?", name = name);
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

/// Optional parameters of [`ClusterRole::read_cluster_role`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadClusterRoleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::read_cluster_role`]
#[derive(Debug)]
pub enum ReadClusterRoleResponse {
    Ok(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Unauthorized,
    Other,
}

impl crate::Response for ReadClusterRoleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadClusterRoleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((ReadClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// replace the specified ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceClusterRoleResponse`]`>` constructor, or [`ReplaceClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_cluster_role(
        name: &str,
        body: &crate::v1_9::api::rbac::v1beta1::ClusterRole,
        optional: ReplaceClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceClusterRoleResponse>), crate::RequestError> {
        let ReplaceClusterRoleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name}?", name = name);
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

/// Optional parameters of [`ClusterRole::replace_cluster_role`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceClusterRoleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::replace_cluster_role`]
#[derive(Debug)]
pub enum ReplaceClusterRoleResponse {
    Ok(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Created(crate::v1_9::api::rbac::v1beta1::ClusterRole),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceClusterRoleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceClusterRoleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceClusterRoleResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceClusterRoleResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1beta1ClusterRole

impl ClusterRole {
    /// list or watch objects of kind ClusterRole
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchClusterRoleResponse`]`>` constructor, or [`WatchClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_cluster_role(
        optional: crate::v1_9::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchClusterRoleResponse>), crate::RequestError> {
        let crate::v1_9::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/rbac.authorization.k8s.io/v1beta1/clusterroles?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
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
        __query_pairs.append_pair("watch", "true");
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<WatchClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::watch_cluster_role`]
#[derive(Debug)]
pub enum WatchClusterRoleResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent<ClusterRole>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchClusterRoleResponse {
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
                Ok((WatchClusterRoleResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchClusterRoleResponse::Unauthorized, 0)),
            _ => Ok((WatchClusterRoleResponse::Other, 0)),
        }
    }
}

// End rbac.authorization.k8s.io/v1beta1/ClusterRole

impl crate::Resource for ClusterRole {
    fn api_version() -> &'static str {
        "rbac.authorization.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "rbac.authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "ClusterRole"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for ClusterRole {
    type Ty = crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ClusterRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_aggregation_rule,
            Key_metadata,
            Key_rules,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRole;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ClusterRole")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_aggregation_rule: Option<crate::v1_9::api::rbac::v1beta1::AggregationRule> = None;
                let mut value_metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_rules: Option<Vec<crate::v1_9::api::rbac::v1beta1::PolicyRule>> = None;

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
                        Field::Key_aggregation_rule => value_aggregation_rule = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRole {
                    aggregation_rule: value_aggregation_rule,
                    metadata: value_metadata,
                    rules: value_rules.ok_or_else(|| serde::de::Error::missing_field("rules"))?,
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

impl serde::Serialize for ClusterRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterRole",
            3 +
            self.aggregation_rule.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.aggregation_rule {
            serde::ser::SerializeStruct::serialize_field(&mut state, "aggregationRule", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        serde::ser::SerializeStruct::end(state)
    }
}
