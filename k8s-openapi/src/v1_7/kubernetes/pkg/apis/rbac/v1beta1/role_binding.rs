// Generated from definition io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBinding

/// RoleBinding references a role, but does not contain it.  It can reference a Role in the same namespace or a ClusterRole in the global namespace. It adds who information via Subjects and namespace information by which namespace it exists in.  RoleBindings in a given namespace only have effect in that namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoleBinding {
    /// Standard object's metadata.
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
    pub role_ref: crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleRef,

    /// Subjects holds references to the objects the role applies to.
    pub subjects: Vec<crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::Subject>,
}

// Begin rbac.authorization.k8s.io/v1beta1/RoleBinding

// Generated from operation createRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// create a RoleBinding
    ///
    /// Use [`CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn create_rbac_authorization_v1beta1_namespaced_role_binding(
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding,
        optional: CreateRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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

/// Optional parameters of [`RoleBinding::create_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.create_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct CreateRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`RoleBinding::create_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.create_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((CreateRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1beta1CollectionNamespacedRoleBinding

impl RoleBinding {
    /// delete collection of RoleBinding
    ///
    /// Use [`DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse`](./enum.DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn delete_rbac_authorization_v1beta1_collection_namespaced_role_binding(
        namespace: &str,
        optional: DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`RoleBinding::delete_rbac_authorization_v1beta1_collection_namespaced_role_binding`](./struct.RoleBinding.html#method.delete_rbac_authorization_v1beta1_collection_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`RoleBinding::delete_rbac_authorization_v1beta1_collection_namespaced_role_binding`](./struct.RoleBinding.html#method.delete_rbac_authorization_v1beta1_collection_namespaced_role_binding)
#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse {
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
                    Ok((DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((DeleteRbacAuthorizationV1beta1CollectionNamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// delete a RoleBinding
    ///
    /// Use [`DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn delete_rbac_authorization_v1beta1_namespaced_role_binding(
        name: &str,
        namespace: &str,
        optional: DeleteRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::delete_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.delete_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct DeleteRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`RoleBinding::delete_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.delete_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
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
                    Ok((DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((DeleteRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// list or watch objects of kind RoleBinding
    ///
    /// Use [`ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn list_rbac_authorization_v1beta1_namespaced_role_binding(
        namespace: &str,
        optional: ListRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings?", namespace = namespace);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`RoleBinding::list_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.list_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct ListRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`RoleBinding::list_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.list_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBindingList),
    Unauthorized,
    Other,
}

impl crate::Response for ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ListRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1beta1RoleBindingForAllNamespaces

impl RoleBinding {
    /// list or watch objects of kind RoleBinding
    ///
    /// Use [`ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse`](./enum.ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_rbac_authorization_v1beta1_role_binding_for_all_namespaces(
        optional: ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/rolebindings?");
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`RoleBinding::list_rbac_authorization_v1beta1_role_binding_for_all_namespaces`](./struct.RoleBinding.html#method.list_rbac_authorization_v1beta1_role_binding_for_all_namespaces)
#[derive(Debug, Default)]
pub struct ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`RoleBinding::list_rbac_authorization_v1beta1_role_binding_for_all_namespaces`](./struct.RoleBinding.html#method.list_rbac_authorization_v1beta1_role_binding_for_all_namespaces)
#[derive(Debug)]
pub enum ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBindingList),
    Unauthorized,
    Other,
}

impl crate::Response for ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListRbacAuthorizationV1beta1RoleBindingForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// partially update the specified RoleBinding
    ///
    /// Use [`PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn patch_rbac_authorization_v1beta1_namespaced_role_binding(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::patch_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.patch_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct PatchRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`RoleBinding::patch_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.patch_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((PatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation readRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// read the specified RoleBinding
    ///
    /// Use [`ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn read_rbac_authorization_v1beta1_namespaced_role_binding(
        name: &str,
        namespace: &str,
        optional: ReadRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::read_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.read_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct ReadRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`RoleBinding::read_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.read_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ReadRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// replace the specified RoleBinding
    ///
    /// Use [`ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn replace_rbac_authorization_v1beta1_namespaced_role_binding(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding,
        optional: ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`RoleBinding::replace_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.replace_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`RoleBinding::replace_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.replace_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ReplaceRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1beta1NamespacedRoleBinding

impl RoleBinding {
    /// watch changes to an object of kind RoleBinding
    ///
    /// Use [`WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse`](./enum.WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse.html) to parse the HTTP response.
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
    pub fn watch_rbac_authorization_v1beta1_namespaced_role_binding(
        name: &str,
        namespace: &str,
        optional: WatchRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchRbacAuthorizationV1beta1NamespacedRoleBindingOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/watch/namespaces/{namespace}/rolebindings/{name}?", name = name, namespace = namespace);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`RoleBinding::watch_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.watch_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug, Default)]
pub struct WatchRbacAuthorizationV1beta1NamespacedRoleBindingOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`RoleBinding::watch_rbac_authorization_v1beta1_namespaced_role_binding`](./struct.RoleBinding.html#method.watch_rbac_authorization_v1beta1_namespaced_role_binding)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse {
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
                Ok((WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1beta1NamespacedRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1beta1NamespacedRoleBindingList

impl RoleBinding {
    /// watch individual changes to a list of RoleBinding
    ///
    /// Use [`WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse`](./enum.WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse.html) to parse the HTTP response.
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
    pub fn watch_rbac_authorization_v1beta1_namespaced_role_binding_list(
        namespace: &str,
        optional: WatchRbacAuthorizationV1beta1NamespacedRoleBindingListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchRbacAuthorizationV1beta1NamespacedRoleBindingListOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/watch/namespaces/{namespace}/rolebindings?", namespace = namespace);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`RoleBinding::watch_rbac_authorization_v1beta1_namespaced_role_binding_list`](./struct.RoleBinding.html#method.watch_rbac_authorization_v1beta1_namespaced_role_binding_list)
#[derive(Debug, Default)]
pub struct WatchRbacAuthorizationV1beta1NamespacedRoleBindingListOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`RoleBinding::watch_rbac_authorization_v1beta1_namespaced_role_binding_list`](./struct.RoleBinding.html#method.watch_rbac_authorization_v1beta1_namespaced_role_binding_list)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse {
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
                Ok((WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1beta1NamespacedRoleBindingListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1beta1RoleBindingListForAllNamespaces

impl RoleBinding {
    /// watch individual changes to a list of RoleBinding
    ///
    /// Use [`WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse`](./enum.WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces(
        optional: WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/watch/rolebindings?");
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`RoleBinding::watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces`](./struct.RoleBinding.html#method.watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces)
#[derive(Debug, Default)]
pub struct WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesOptional<'a> {
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`RoleBinding::watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces`](./struct.RoleBinding.html#method.watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse {
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
                Ok((WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1beta1RoleBindingListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End rbac.authorization.k8s.io/v1beta1/RoleBinding

impl crate::Resource for RoleBinding {
    fn api_version() -> &'static str {
        "rbac.authorization.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "rbac.authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "RoleBinding"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for RoleBinding {
    type Ty = crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_role_ref: Option<crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::RoleRef> = None;
                let mut value_subjects: Option<Vec<crate::v1_7::kubernetes::pkg::apis::rbac::v1beta1::Subject>> = None;

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
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1 +
            1,
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
