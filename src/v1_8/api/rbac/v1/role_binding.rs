// Generated from definition io.k8s.api.rbac.v1.RoleBinding

/// RoleBinding references a role, but does not contain it.  It can reference a Role in the same namespace or a ClusterRole in the global namespace. It adds who information via Subjects and namespace information by which namespace it exists in.  RoleBindings in a given namespace only have effect in that namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoleBinding {
    /// Standard object's metadata.
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
    pub role_ref: crate::v1_8::api::rbac::v1::RoleRef,

    /// Subjects holds references to the objects the role applies to.
    pub subjects: Vec<crate::v1_8::api::rbac::v1::Subject>,
}

// Begin rbac.authorization.k8s.io/v1/RoleBinding

// Generated from operation createRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// create a RoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedRoleBindingResponse`]`>` constructor, or [`CreateNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_role_binding(
        namespace: &str,
        body: &crate::v1_8::api::rbac::v1::RoleBinding,
        optional: CreateNamespacedRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedRoleBindingResponse>), crate::RequestError> {
        let CreateNamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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

/// Optional parameters of [`RoleBinding::create_namespaced_role_binding`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::create_namespaced_role_binding`]
#[derive(Debug)]
pub enum CreateNamespacedRoleBindingResponse {
    Ok(crate::v1_8::api::rbac::v1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1CollectionNamespacedRoleBinding

impl RoleBinding {
    /// delete collection of RoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedRoleBindingResponse`]`>` constructor, or [`DeleteCollectionNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_namespaced_role_binding(
        namespace: &str,
        optional: DeleteCollectionNamespacedRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedRoleBindingResponse>), crate::RequestError> {
        let DeleteCollectionNamespacedRoleBindingOptional {
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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

/// Optional parameters of [`RoleBinding::delete_collection_namespaced_role_binding`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionNamespacedRoleBindingOptional<'a> {
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

/// Use `<DeleteCollectionNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::delete_collection_namespaced_role_binding`]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedRoleBindingResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::api::rbac::v1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNamespacedRoleBindingResponse {
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
                    Ok((DeleteCollectionNamespacedRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedRoleBindingResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// delete a RoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedRoleBindingResponse`]`>` constructor, or [`DeleteNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RoleBinding
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_role_binding(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedRoleBindingResponse>), crate::RequestError> {
        let DeleteNamespacedRoleBindingOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::delete_namespaced_role_binding`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespacedRoleBindingOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::delete_namespaced_role_binding`]
#[derive(Debug)]
pub enum DeleteNamespacedRoleBindingResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::api::rbac::v1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespacedRoleBindingResponse {
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
                    Ok((DeleteNamespacedRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedRoleBindingResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// list or watch objects of kind RoleBinding
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedRoleBindingResponse`]`>` constructor, or [`ListNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_role_binding(
        namespace: &str,
        optional: crate::v1_8::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedRoleBindingResponse>), crate::RequestError> {
        let crate::v1_8::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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

/// Use `<ListNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::list_namespaced_role_binding`]
#[derive(Debug)]
pub enum ListNamespacedRoleBindingResponse {
    Ok(crate::v1_8::api::rbac::v1::RoleBindingList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ListNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1RoleBindingForAllNamespaces

impl RoleBinding {
    /// list or watch objects of kind RoleBinding
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListRoleBindingForAllNamespacesResponse`]`>` constructor, or [`ListRoleBindingForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_role_binding_for_all_namespaces(
        optional: crate::v1_8::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListRoleBindingForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_8::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/rbac.authorization.k8s.io/v1/rolebindings?".to_string();
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

/// Use `<ListRoleBindingForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::list_role_binding_for_all_namespaces`]
#[derive(Debug)]
pub enum ListRoleBindingForAllNamespacesResponse {
    Ok(crate::v1_8::api::rbac::v1::RoleBindingList),
    Unauthorized,
    Other,
}

impl crate::Response for ListRoleBindingForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListRoleBindingForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListRoleBindingForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListRoleBindingForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// partially update the specified RoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedRoleBindingResponse`]`>` constructor, or [`PatchNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RoleBinding
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
    pub fn patch_namespaced_role_binding(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedRoleBindingResponse>), crate::RequestError> {
        let PatchNamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::patch_namespaced_role_binding`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::patch_namespaced_role_binding`]
#[derive(Debug)]
pub enum PatchNamespacedRoleBindingResponse {
    Ok(crate::v1_8::api::rbac::v1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation readRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// read the specified RoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedRoleBindingResponse`]`>` constructor, or [`ReadNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RoleBinding
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_role_binding(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedRoleBindingResponse>), crate::RequestError> {
        let ReadNamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::read_namespaced_role_binding`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::read_namespaced_role_binding`]
#[derive(Debug)]
pub enum ReadNamespacedRoleBindingResponse {
    Ok(crate::v1_8::api::rbac::v1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// replace the specified RoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedRoleBindingResponse`]`>` constructor, or [`ReplaceNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RoleBinding
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
    pub fn replace_namespaced_role_binding(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::rbac::v1::RoleBinding,
        optional: ReplaceNamespacedRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedRoleBindingResponse>), crate::RequestError> {
        let ReplaceNamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::replace_namespaced_role_binding`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::replace_namespaced_role_binding`]
#[derive(Debug)]
pub enum ReplaceNamespacedRoleBindingResponse {
    Ok(crate::v1_8::api::rbac::v1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1NamespacedRoleBinding

impl RoleBinding {
    /// list or watch objects of kind RoleBinding
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedRoleBindingResponse`]`>` constructor, or [`WatchNamespacedRoleBindingResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_role_binding(
        namespace: &str,
        optional: crate::v1_8::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedRoleBindingResponse>), crate::RequestError> {
        let crate::v1_8::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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

/// Use `<WatchNamespacedRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::watch_namespaced_role_binding`]
#[derive(Debug)]
pub enum WatchNamespacedRoleBindingResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent<RoleBinding>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedRoleBindingResponse {
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
                Ok((WatchNamespacedRoleBindingResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1RoleBindingForAllNamespaces

impl RoleBinding {
    /// list or watch objects of kind RoleBinding
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchRoleBindingForAllNamespacesResponse`]`>` constructor, or [`WatchRoleBindingForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_role_binding_for_all_namespaces(
        optional: crate::v1_8::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchRoleBindingForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_8::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/rbac.authorization.k8s.io/v1/rolebindings?".to_string();
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

/// Use `<WatchRoleBindingForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`RoleBinding::watch_role_binding_for_all_namespaces`]
#[derive(Debug)]
pub enum WatchRoleBindingForAllNamespacesResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent<RoleBinding>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchRoleBindingForAllNamespacesResponse {
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
                Ok((WatchRoleBindingForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchRoleBindingForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchRoleBindingForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End rbac.authorization.k8s.io/v1/RoleBinding

impl crate::Resource for RoleBinding {
    fn api_version() -> &'static str {
        "rbac.authorization.k8s.io/v1"
    }

    fn group() -> &'static str {
        "rbac.authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "RoleBinding"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for RoleBinding {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for RoleBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_role_ref,
            Key_subjects,
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
                            "roleRef" => Field::Key_role_ref,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RoleBinding;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct RoleBinding")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_role_ref: Option<crate::v1_8::api::rbac::v1::RoleRef> = None;
                let mut value_subjects: Option<Vec<crate::v1_8::api::rbac::v1::Subject>> = None;

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
                        Field::Key_role_ref => value_role_ref = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subjects => value_subjects = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoleBinding {
                    metadata: value_metadata,
                    role_ref: value_role_ref.ok_or_else(|| serde::de::Error::missing_field("roleRef"))?,
                    subjects: value_subjects.ok_or_else(|| serde::de::Error::missing_field("subjects"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RoleBinding",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "roleRef",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RoleBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoleBinding",
            4 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "roleRef", &self.role_ref)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        serde::ser::SerializeStruct::end(state)
    }
}
