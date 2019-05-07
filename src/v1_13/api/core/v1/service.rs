// Generated from definition io.k8s.api.core.v1.Service

/// Service is a named abstraction of software service (for example, mysql) consisting of local port (for example 3306) that the proxy listens on, and the selector that determines which pods will answer requests sent through the proxy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Service {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_13::api::core::v1::ServiceSpec>,

    /// Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_13::api::core::v1::ServiceStatus>,
}

// Begin /v1/Service

// Generated from operation connectCoreV1DeleteNamespacedServiceProxy

impl Service {
    /// connect DELETE requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_delete_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectDeleteNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectDeleteNamespacedServiceProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_delete_namespaced_service_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1DeleteNamespacedServiceProxyWithPath

impl Service {
    /// connect DELETE requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_delete_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectDeleteNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectDeleteNamespacedServiceProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_delete_namespaced_service_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1GetNamespacedServiceProxy

impl Service {
    /// connect GET requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectGetNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetNamespacedServiceProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_get_namespaced_service_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1GetNamespacedServiceProxyWithPath

impl Service {
    /// connect GET requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectGetNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetNamespacedServiceProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_get_namespaced_service_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PatchNamespacedServiceProxy

impl Service {
    /// connect PATCH requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_patch_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPatchNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPatchNamespacedServiceProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_patch_namespaced_service_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PatchNamespacedServiceProxyWithPath

impl Service {
    /// connect PATCH requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_patch_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPatchNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPatchNamespacedServiceProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_patch_namespaced_service_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PostNamespacedServiceProxy

impl Service {
    /// connect POST requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPostNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostNamespacedServiceProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_post_namespaced_service_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PostNamespacedServiceProxyWithPath

impl Service {
    /// connect POST requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPostNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostNamespacedServiceProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_post_namespaced_service_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PutNamespacedServiceProxy

impl Service {
    /// connect PUT requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_put_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPutNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPutNamespacedServiceProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_put_namespaced_service_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PutNamespacedServiceProxyWithPath

impl Service {
    /// connect PUT requests to proxy of Service
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_put_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPutNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPutNamespacedServiceProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::connect_put_namespaced_service_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

// Generated from operation createCoreV1NamespacedService

impl Service {
    /// create a Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedServiceResponse`]`>` constructor, or [`CreateNamespacedServiceResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_service(
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Service,
        optional: CreateNamespacedServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedServiceResponse>), crate::RequestError> {
        let CreateNamespacedServiceOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
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

/// Optional parameters of [`Service::create_namespaced_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If IncludeUninitialized is specified, the object may be returned without completing initialization.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::create_namespaced_service`]
#[derive(Debug)]
pub enum CreateNamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Created(crate::v1_13::api::core::v1::Service),
    Accepted(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for CreateNamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedServiceResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedServiceResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((CreateNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreV1NamespacedService

impl Service {
    /// delete a Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedServiceResponse`]`>` constructor, or [`DeleteNamespacedServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_service(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedServiceResponse>), crate::RequestError> {
        let DeleteNamespacedServiceOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`Service::delete_namespaced_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::delete_namespaced_service`]
#[derive(Debug)]
pub enum DeleteNamespacedServiceResponse {
    OkStatus(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_13::api::core::v1::Service),
    Accepted(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for DeleteNamespacedServiceResponse {
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
                    Ok((DeleteNamespacedServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedServiceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespacedServiceResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((DeleteNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreV1NamespacedService

impl Service {
    /// list or watch objects of kind Service
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedServiceResponse`]`>` constructor, or [`ListNamespacedServiceResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_service(
        namespace: &str,
        optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedServiceResponse>), crate::RequestError> {
        let crate::v1_13::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services?", namespace = namespace);
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

/// Use `<ListNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::list_namespaced_service`]
#[derive(Debug)]
pub enum ListNamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::ServiceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ListNamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedServiceResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreV1ServiceForAllNamespaces

impl Service {
    /// list or watch objects of kind Service
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListServiceForAllNamespacesResponse`]`>` constructor, or [`ListServiceForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_service_for_all_namespaces(
        optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListServiceForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_13::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/services?".to_string();
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

/// Use `<ListServiceForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::list_service_for_all_namespaces`]
#[derive(Debug)]
pub enum ListServiceForAllNamespacesResponse {
    Ok(crate::v1_13::api::core::v1::ServiceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ListServiceForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListServiceForAllNamespacesResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListServiceForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreV1NamespacedService

impl Service {
    /// partially update the specified Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedServiceResponse`]`>` constructor, or [`PatchNamespacedServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
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
    pub fn patch_namespaced_service(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedServiceResponse>), crate::RequestError> {
        let PatchNamespacedServiceOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`Service::patch_namespaced_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::patch_namespaced_service`]
#[derive(Debug)]
pub enum PatchNamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for PatchNamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedServiceResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((PatchNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreV1NamespacedServiceStatus

impl Service {
    /// partially update status of the specified Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedServiceStatusResponse`]`>` constructor, or [`PatchNamespacedServiceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
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
    pub fn patch_namespaced_service_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedServiceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedServiceStatusResponse>), crate::RequestError> {
        let PatchNamespacedServiceStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`Service::patch_namespaced_service_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedServiceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::patch_namespaced_service_status`]
#[derive(Debug)]
pub enum PatchNamespacedServiceStatusResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for PatchNamespacedServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((PatchNamespacedServiceStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreV1NamespacedService

impl Service {
    /// read the specified Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedServiceResponse`]`>` constructor, or [`ReadNamespacedServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_service(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedServiceResponse>), crate::RequestError> {
        let ReadNamespacedServiceOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`Service::read_namespaced_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedServiceOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::read_namespaced_service`]
#[derive(Debug)]
pub enum ReadNamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReadNamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedServiceResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreV1NamespacedServiceStatus

impl Service {
    /// read status of the specified Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedServiceStatusResponse`]`>` constructor, or [`ReadNamespacedServiceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_service_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedServiceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedServiceStatusResponse>), crate::RequestError> {
        let ReadNamespacedServiceStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`Service::read_namespaced_service_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedServiceStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::read_namespaced_service_status`]
#[derive(Debug)]
pub enum ReadNamespacedServiceStatusResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReadNamespacedServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedServiceStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreV1NamespacedService

impl Service {
    /// replace the specified Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedServiceResponse`]`>` constructor, or [`ReplaceNamespacedServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
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
    pub fn replace_namespaced_service(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Service,
        optional: ReplaceNamespacedServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedServiceResponse>), crate::RequestError> {
        let ReplaceNamespacedServiceOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`Service::replace_namespaced_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::replace_namespaced_service`]
#[derive(Debug)]
pub enum ReplaceNamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Created(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReplaceNamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedServiceResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReplaceNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreV1NamespacedServiceStatus

impl Service {
    /// replace status of the specified Service
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedServiceStatusResponse`]`>` constructor, or [`ReplaceNamespacedServiceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Service
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
    pub fn replace_namespaced_service_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Service,
        optional: ReplaceNamespacedServiceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedServiceStatusResponse>), crate::RequestError> {
        let ReplaceNamespacedServiceStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`Service::replace_namespaced_service_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedServiceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::replace_namespaced_service_status`]
#[derive(Debug)]
pub enum ReplaceNamespacedServiceStatusResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Created(crate::v1_13::api::core::v1::Service),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReplaceNamespacedServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedServiceStatusResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReplaceNamespacedServiceStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreV1NamespacedService

impl Service {
    /// list or watch objects of kind Service
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedServiceResponse`]`>` constructor, or [`WatchNamespacedServiceResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_service(
        namespace: &str,
        optional: crate::v1_13::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedServiceResponse>), crate::RequestError> {
        let crate::v1_13::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/services?", namespace = namespace);
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

/// Use `<WatchNamespacedServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::watch_namespaced_service`]
#[derive(Debug)]
pub enum WatchNamespacedServiceResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent<Service>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for WatchNamespacedServiceResponse {
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
                Ok((WatchNamespacedServiceResponse::Ok(result), byte_offset))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchNamespacedServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreV1ServiceForAllNamespaces

impl Service {
    /// list or watch objects of kind Service
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchServiceForAllNamespacesResponse`]`>` constructor, or [`WatchServiceForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_service_for_all_namespaces(
        optional: crate::v1_13::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchServiceForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_13::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/services?".to_string();
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

/// Use `<WatchServiceForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`Service::watch_service_for_all_namespaces`]
#[derive(Debug)]
pub enum WatchServiceForAllNamespacesResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent<Service>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for WatchServiceForAllNamespacesResponse {
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
                Ok((WatchServiceForAllNamespacesResponse::Ok(result), byte_offset))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchServiceForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// End /v1/Service

impl crate::Resource for Service {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "Service"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for Service {
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Service {
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
            type Value = Service;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Service")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_13::api::core::v1::ServiceSpec> = None;
                let mut value_status: Option<crate::v1_13::api::core::v1::ServiceStatus> = None;

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

                Ok(Service {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Service",
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

impl serde::Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Service",
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
