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
    /// Use [`ConnectCoreV1DeleteNamespacedServiceProxyResponse`](./enum.ConnectCoreV1DeleteNamespacedServiceProxyResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_delete_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectCoreV1DeleteNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1DeleteNamespacedServiceProxyOptional {
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

/// Optional parameters of [`Service::connect_core_v1_delete_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_delete_namespaced_service_proxy)
#[derive(Debug, Default)]
pub struct ConnectCoreV1DeleteNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_delete_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_delete_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1DeleteNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1DeleteNamespacedServiceProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNamespacedServiceProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1DeleteNamespacedServiceProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1DeleteNamespacedServiceProxyWithPath

impl Service {
    /// connect DELETE requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse`](./enum.ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_delete_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectCoreV1DeleteNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1DeleteNamespacedServiceProxyWithPathOptional {
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

/// Optional parameters of [`Service::connect_core_v1_delete_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_delete_namespaced_service_proxy_with_path)
#[derive(Debug, Default)]
pub struct ConnectCoreV1DeleteNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_delete_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_delete_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedServiceProxy

impl Service {
    /// connect GET requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1GetNamespacedServiceProxyResponse`](./enum.ConnectCoreV1GetNamespacedServiceProxyResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_get_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectCoreV1GetNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1GetNamespacedServiceProxyOptional {
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

/// Optional parameters of [`Service::connect_core_v1_get_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_get_namespaced_service_proxy)
#[derive(Debug, Default)]
pub struct ConnectCoreV1GetNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_get_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_get_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1GetNamespacedServiceProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedServiceProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedServiceProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedServiceProxyWithPath

impl Service {
    /// connect GET requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1GetNamespacedServiceProxyWithPathResponse`](./enum.ConnectCoreV1GetNamespacedServiceProxyWithPathResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_get_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectCoreV1GetNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1GetNamespacedServiceProxyWithPathOptional {
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

/// Optional parameters of [`Service::connect_core_v1_get_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_get_namespaced_service_proxy_with_path)
#[derive(Debug, Default)]
pub struct ConnectCoreV1GetNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_get_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_get_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNamespacedServiceProxy

impl Service {
    /// connect PATCH requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1PatchNamespacedServiceProxyResponse`](./enum.ConnectCoreV1PatchNamespacedServiceProxyResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_patch_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectCoreV1PatchNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1PatchNamespacedServiceProxyOptional {
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

/// Optional parameters of [`Service::connect_core_v1_patch_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_patch_namespaced_service_proxy)
#[derive(Debug, Default)]
pub struct ConnectCoreV1PatchNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_patch_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_patch_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PatchNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PatchNamespacedServiceProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNamespacedServiceProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PatchNamespacedServiceProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNamespacedServiceProxyWithPath

impl Service {
    /// connect PATCH requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse`](./enum.ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_patch_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectCoreV1PatchNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1PatchNamespacedServiceProxyWithPathOptional {
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

/// Optional parameters of [`Service::connect_core_v1_patch_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_patch_namespaced_service_proxy_with_path)
#[derive(Debug, Default)]
pub struct ConnectCoreV1PatchNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_patch_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_patch_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedServiceProxy

impl Service {
    /// connect POST requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1PostNamespacedServiceProxyResponse`](./enum.ConnectCoreV1PostNamespacedServiceProxyResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_post_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectCoreV1PostNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1PostNamespacedServiceProxyOptional {
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

/// Optional parameters of [`Service::connect_core_v1_post_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_post_namespaced_service_proxy)
#[derive(Debug, Default)]
pub struct ConnectCoreV1PostNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_post_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_post_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PostNamespacedServiceProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedServiceProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedServiceProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedServiceProxyWithPath

impl Service {
    /// connect POST requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1PostNamespacedServiceProxyWithPathResponse`](./enum.ConnectCoreV1PostNamespacedServiceProxyWithPathResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_post_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectCoreV1PostNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1PostNamespacedServiceProxyWithPathOptional {
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

/// Optional parameters of [`Service::connect_core_v1_post_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_post_namespaced_service_proxy_with_path)
#[derive(Debug, Default)]
pub struct ConnectCoreV1PostNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_post_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_post_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNamespacedServiceProxy

impl Service {
    /// connect PUT requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1PutNamespacedServiceProxyResponse`](./enum.ConnectCoreV1PutNamespacedServiceProxyResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_put_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectCoreV1PutNamespacedServiceProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1PutNamespacedServiceProxyOptional {
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

/// Optional parameters of [`Service::connect_core_v1_put_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_put_namespaced_service_proxy)
#[derive(Debug, Default)]
pub struct ConnectCoreV1PutNamespacedServiceProxyOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_put_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_put_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PutNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PutNamespacedServiceProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNamespacedServiceProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PutNamespacedServiceProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNamespacedServiceProxyWithPath

impl Service {
    /// connect PUT requests to proxy of Service
    ///
    /// Use [`ConnectCoreV1PutNamespacedServiceProxyWithPathResponse`](./enum.ConnectCoreV1PutNamespacedServiceProxyWithPathResponse.html) to parse the HTTP response.
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
    pub fn connect_core_v1_put_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectCoreV1PutNamespacedServiceProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectCoreV1PutNamespacedServiceProxyWithPathOptional {
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

/// Optional parameters of [`Service::connect_core_v1_put_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_put_namespaced_service_proxy_with_path)
#[derive(Debug, Default)]
pub struct ConnectCoreV1PutNamespacedServiceProxyWithPathOptional<'a> {
    /// Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub path_: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::connect_core_v1_put_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_put_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PutNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(crate::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation createCoreV1NamespacedService

impl Service {
    /// create a Service
    ///
    /// Use [`CreateCoreV1NamespacedServiceResponse`](./enum.CreateCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn create_core_v1_namespaced_service(
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Service,
        optional: CreateCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateCoreV1NamespacedServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::create_core_v1_namespaced_service`](./struct.Service.html#method.create_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct CreateCoreV1NamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If IncludeUninitialized is specified, the object may be returned without completing initialization.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::create_core_v1_namespaced_service`](./struct.Service.html#method.create_core_v1_namespaced_service)
#[derive(Debug)]
pub enum CreateCoreV1NamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Created(crate::v1_13::api::core::v1::Service),
    Accepted(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedServiceResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedServiceResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((CreateCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedService

impl Service {
    /// delete a Service
    ///
    /// Use [`DeleteCoreV1NamespacedServiceResponse`](./enum.DeleteCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn delete_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        optional: DeleteCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCoreV1NamespacedServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::delete_core_v1_namespaced_service`](./struct.Service.html#method.delete_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct DeleteCoreV1NamespacedServiceOptional<'a> {
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

/// Parses the HTTP response of [`Service::delete_core_v1_namespaced_service`](./struct.Service.html#method.delete_core_v1_namespaced_service)
#[derive(Debug)]
pub enum DeleteCoreV1NamespacedServiceResponse {
    OkStatus(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_13::api::core::v1::Service),
    Accepted(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1NamespacedServiceResponse {
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
                    Ok((DeleteCoreV1NamespacedServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1NamespacedServiceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteCoreV1NamespacedServiceResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1NamespacedService

impl Service {
    /// list or watch objects of kind Service
    ///
    /// Use [`ListCoreV1NamespacedServiceResponse`](./enum.ListCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn list_core_v1_namespaced_service(
        namespace: &str,
        optional: ListCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListCoreV1NamespacedServiceOptional {
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::list_core_v1_namespaced_service`](./struct.Service.html#method.list_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct ListCoreV1NamespacedServiceOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If IncludeUninitialized is specified, the object may be returned without completing initialization.
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

/// Parses the HTTP response of [`Service::list_core_v1_namespaced_service`](./struct.Service.html#method.list_core_v1_namespaced_service)
#[derive(Debug)]
pub enum ListCoreV1NamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::ServiceList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1ServiceForAllNamespaces

impl Service {
    /// list or watch objects of kind Service
    ///
    /// Use [`ListCoreV1ServiceForAllNamespacesResponse`](./enum.ListCoreV1ServiceForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_core_v1_service_for_all_namespaces(
        optional: ListCoreV1ServiceForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListCoreV1ServiceForAllNamespacesOptional {
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
        let __url = format!("/api/v1/services?");
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

/// Optional parameters of [`Service::list_core_v1_service_for_all_namespaces`](./struct.Service.html#method.list_core_v1_service_for_all_namespaces)
#[derive(Debug, Default)]
pub struct ListCoreV1ServiceForAllNamespacesOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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

/// Parses the HTTP response of [`Service::list_core_v1_service_for_all_namespaces`](./struct.Service.html#method.list_core_v1_service_for_all_namespaces)
#[derive(Debug)]
pub enum ListCoreV1ServiceForAllNamespacesResponse {
    Ok(crate::v1_13::api::core::v1::ServiceList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1ServiceForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1ServiceForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1ServiceForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1ServiceForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedService

impl Service {
    /// partially update the specified Service
    ///
    /// Use [`PatchCoreV1NamespacedServiceResponse`](./enum.PatchCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn patch_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchCoreV1NamespacedServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::patch_core_v1_namespaced_service`](./struct.Service.html#method.patch_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct PatchCoreV1NamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::patch_core_v1_namespaced_service`](./struct.Service.html#method.patch_core_v1_namespaced_service)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedServiceStatus

impl Service {
    /// partially update status of the specified Service
    ///
    /// Use [`PatchCoreV1NamespacedServiceStatusResponse`](./enum.PatchCoreV1NamespacedServiceStatusResponse.html) to parse the HTTP response.
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
    pub fn patch_core_v1_namespaced_service_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchCoreV1NamespacedServiceStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchCoreV1NamespacedServiceStatusOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::patch_core_v1_namespaced_service_status`](./struct.Service.html#method.patch_core_v1_namespaced_service_status)
#[derive(Debug, Default)]
pub struct PatchCoreV1NamespacedServiceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::patch_core_v1_namespaced_service_status`](./struct.Service.html#method.patch_core_v1_namespaced_service_status)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedServiceStatusResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedService

impl Service {
    /// read the specified Service
    ///
    /// Use [`ReadCoreV1NamespacedServiceResponse`](./enum.ReadCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn read_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        optional: ReadCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadCoreV1NamespacedServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::read_core_v1_namespaced_service`](./struct.Service.html#method.read_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct ReadCoreV1NamespacedServiceOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::read_core_v1_namespaced_service`](./struct.Service.html#method.read_core_v1_namespaced_service)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedServiceStatus

impl Service {
    /// read status of the specified Service
    ///
    /// Use [`ReadCoreV1NamespacedServiceStatusResponse`](./enum.ReadCoreV1NamespacedServiceStatusResponse.html) to parse the HTTP response.
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
    pub fn read_core_v1_namespaced_service_status(
        name: &str,
        namespace: &str,
        optional: ReadCoreV1NamespacedServiceStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadCoreV1NamespacedServiceStatusOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::read_core_v1_namespaced_service_status`](./struct.Service.html#method.read_core_v1_namespaced_service_status)
#[derive(Debug, Default)]
pub struct ReadCoreV1NamespacedServiceStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::read_core_v1_namespaced_service_status`](./struct.Service.html#method.read_core_v1_namespaced_service_status)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedServiceStatusResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedService

impl Service {
    /// replace the specified Service
    ///
    /// Use [`ReplaceCoreV1NamespacedServiceResponse`](./enum.ReplaceCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn replace_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Service,
        optional: ReplaceCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceCoreV1NamespacedServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::replace_core_v1_namespaced_service`](./struct.Service.html#method.replace_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct ReplaceCoreV1NamespacedServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::replace_core_v1_namespaced_service`](./struct.Service.html#method.replace_core_v1_namespaced_service)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedServiceResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Created(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedServiceResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedServiceStatus

impl Service {
    /// replace status of the specified Service
    ///
    /// Use [`ReplaceCoreV1NamespacedServiceStatusResponse`](./enum.ReplaceCoreV1NamespacedServiceStatusResponse.html) to parse the HTTP response.
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
    pub fn replace_core_v1_namespaced_service_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Service,
        optional: ReplaceCoreV1NamespacedServiceStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceCoreV1NamespacedServiceStatusOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Service::replace_core_v1_namespaced_service_status`](./struct.Service.html#method.replace_core_v1_namespaced_service_status)
#[derive(Debug, Default)]
pub struct ReplaceCoreV1NamespacedServiceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Service::replace_core_v1_namespaced_service_status`](./struct.Service.html#method.replace_core_v1_namespaced_service_status)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedServiceStatusResponse {
    Ok(crate::v1_13::api::core::v1::Service),
    Created(crate::v1_13::api::core::v1::Service),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedService

impl Service {
    /// watch changes to an object of kind Service. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use [`WatchCoreV1NamespacedServiceResponse`](./enum.WatchCoreV1NamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn watch_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        optional: WatchCoreV1NamespacedServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchCoreV1NamespacedServiceOptional {
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
        let __url = format!("/api/v1/watch/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`Service::watch_core_v1_namespaced_service`](./struct.Service.html#method.watch_core_v1_namespaced_service)
#[derive(Debug, Default)]
pub struct WatchCoreV1NamespacedServiceOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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

/// Parses the HTTP response of [`Service::watch_core_v1_namespaced_service`](./struct.Service.html#method.watch_core_v1_namespaced_service)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedServiceResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NamespacedServiceResponse {
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
                Ok((WatchCoreV1NamespacedServiceResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedServiceList

impl Service {
    /// watch individual changes to a list of Service. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchCoreV1NamespacedServiceListResponse`](./enum.WatchCoreV1NamespacedServiceListResponse.html) to parse the HTTP response.
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
    pub fn watch_core_v1_namespaced_service_list(
        namespace: &str,
        optional: WatchCoreV1NamespacedServiceListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchCoreV1NamespacedServiceListOptional {
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
        let __url = format!("/api/v1/watch/namespaces/{namespace}/services?", namespace = namespace);
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

/// Optional parameters of [`Service::watch_core_v1_namespaced_service_list`](./struct.Service.html#method.watch_core_v1_namespaced_service_list)
#[derive(Debug, Default)]
pub struct WatchCoreV1NamespacedServiceListOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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

/// Parses the HTTP response of [`Service::watch_core_v1_namespaced_service_list`](./struct.Service.html#method.watch_core_v1_namespaced_service_list)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedServiceListResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NamespacedServiceListResponse {
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
                Ok((WatchCoreV1NamespacedServiceListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedServiceListResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedServiceListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1ServiceListForAllNamespaces

impl Service {
    /// watch individual changes to a list of Service. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchCoreV1ServiceListForAllNamespacesResponse`](./enum.WatchCoreV1ServiceListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_core_v1_service_list_for_all_namespaces(
        optional: WatchCoreV1ServiceListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchCoreV1ServiceListForAllNamespacesOptional {
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
        let __url = format!("/api/v1/watch/services?");
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

/// Optional parameters of [`Service::watch_core_v1_service_list_for_all_namespaces`](./struct.Service.html#method.watch_core_v1_service_list_for_all_namespaces)
#[derive(Debug, Default)]
pub struct WatchCoreV1ServiceListForAllNamespacesOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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

/// Parses the HTTP response of [`Service::watch_core_v1_service_list_for_all_namespaces`](./struct.Service.html#method.watch_core_v1_service_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchCoreV1ServiceListForAllNamespacesResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1ServiceListForAllNamespacesResponse {
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
                Ok((WatchCoreV1ServiceListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1ServiceListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1ServiceListForAllNamespacesResponse::Other, 0)),
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
