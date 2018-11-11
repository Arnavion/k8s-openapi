// Generated from definition io.k8s.kubernetes.pkg.api.v1.Service

/// Service is a named abstraction of software service (for example, mysql) consisting of local port (for example 3306) that the proxy listens on, and the selector that determines which pods will answer requests sent through the proxy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Service {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_7::kubernetes::pkg::api::v1::ServiceSpec>,

    /// Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<::v1_7::kubernetes::pkg::api::v1::ServiceStatus>,
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_delete_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_delete_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_delete_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1DeleteNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1DeleteNamespacedServiceProxyResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNamespacedServiceProxyResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_delete_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_delete_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_delete_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_get_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_get_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_get_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1GetNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1GetNamespacedServiceProxyResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedServiceProxyResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_get_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_get_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_get_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1GetNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_patch_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_patch_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_patch_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1PatchNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PatchNamespacedServiceProxyResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNamespacedServiceProxyResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_patch_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_patch_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_patch_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_post_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_post_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_post_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1PostNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PostNamespacedServiceProxyResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedServiceProxyResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_post_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_post_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_post_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1PostNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_put_namespaced_service_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_put_namespaced_service_proxy`](./struct.Service.html#method.connect_core_v1_put_namespaced_service_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedServiceProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1PutNamespacedServiceProxyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PutNamespacedServiceProxyResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNamespacedServiceProxyResponse::Unauthorized, 0)),
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
    ///     name of the Service
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
    pub fn connect_core_v1_put_namespaced_service_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::connect_core_v1_put_namespaced_service_proxy_with_path`](./struct.Service.html#method.connect_core_v1_put_namespaced_service_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedServiceProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ConnectCoreV1PutNamespacedServiceProxyWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_core_v1_namespaced_service(
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Service,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services?", namespace = namespace);
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

/// Parses the HTTP response of [`Service::create_core_v1_namespaced_service`](./struct.Service.html#method.create_core_v1_namespaced_service)
#[derive(Debug)]
pub enum CreateCoreV1NamespacedServiceResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for CreateCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateCoreV1NamespacedServiceResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn delete_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::delete_core_v1_namespaced_service`](./struct.Service.html#method.delete_core_v1_namespaced_service)
#[derive(Debug)]
pub enum DeleteCoreV1NamespacedServiceResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for DeleteCoreV1NamespacedServiceResponse {
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
                    Ok((DeleteCoreV1NamespacedServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCoreV1NamespacedServiceResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1NamespacedServiceResponse::Unauthorized, 0)),
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
    pub fn list_core_v1_namespaced_service(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::list_core_v1_namespaced_service`](./struct.Service.html#method.list_core_v1_namespaced_service)
#[derive(Debug)]
pub enum ListCoreV1NamespacedServiceResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::ServiceList),
    Unauthorized,
    Other,
}

impl ::Response for ListCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1NamespacedServiceResponse::Unauthorized, 0)),
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
    pub fn list_core_v1_service_for_all_namespaces(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/services?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::list_core_v1_service_for_all_namespaces`](./struct.Service.html#method.list_core_v1_service_for_all_namespaces)
#[derive(Debug)]
pub enum ListCoreV1ServiceForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::ServiceList),
    Unauthorized,
    Other,
}

impl ::Response for ListCoreV1ServiceForAllNamespacesResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListCoreV1ServiceForAllNamespacesResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1ServiceForAllNamespacesResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Service::patch_core_v1_namespaced_service`](./struct.Service.html#method.patch_core_v1_namespaced_service)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedServiceResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for PatchCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedServiceResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_core_v1_namespaced_service_status(
        name: &str,
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Service::patch_core_v1_namespaced_service_status`](./struct.Service.html#method.patch_core_v1_namespaced_service_status)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedServiceStatusResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for PatchCoreV1NamespacedServiceStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1DELETENamespacedService

impl Service {
    /// proxy DELETE requests to Service
    ///
    /// Use [`ProxyCoreV1DELETENamespacedServiceResponse`](./enum.ProxyCoreV1DELETENamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn proxy_core_v1_delete_namespaced_service(
        name: &str,
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_delete_namespaced_service`](./struct.Service.html#method.proxy_core_v1_delete_namespaced_service)
#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedServiceResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1DELETENamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1DELETENamespacedServiceResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1DELETENamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1DELETENamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1DELETENamespacedServiceWithPath

impl Service {
    /// proxy DELETE requests to Service
    ///
    /// Use [`ProxyCoreV1DELETENamespacedServiceWithPathResponse`](./enum.ProxyCoreV1DELETENamespacedServiceWithPathResponse.html) to parse the HTTP response.
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
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_delete_namespaced_service_with_path(
        name: &str,
        namespace: &str,
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_delete_namespaced_service_with_path`](./struct.Service.html#method.proxy_core_v1_delete_namespaced_service_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedServiceWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1DELETENamespacedServiceWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1DELETENamespacedServiceWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1DELETENamespacedServiceWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1DELETENamespacedServiceWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1GETNamespacedService

impl Service {
    /// proxy GET requests to Service
    ///
    /// Use [`ProxyCoreV1GETNamespacedServiceResponse`](./enum.ProxyCoreV1GETNamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn proxy_core_v1_get_namespaced_service(
        name: &str,
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_get_namespaced_service`](./struct.Service.html#method.proxy_core_v1_get_namespaced_service)
#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedServiceResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1GETNamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1GETNamespacedServiceResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1GETNamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1GETNamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1GETNamespacedServiceWithPath

impl Service {
    /// proxy GET requests to Service
    ///
    /// Use [`ProxyCoreV1GETNamespacedServiceWithPathResponse`](./enum.ProxyCoreV1GETNamespacedServiceWithPathResponse.html) to parse the HTTP response.
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
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_get_namespaced_service_with_path(
        name: &str,
        namespace: &str,
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_get_namespaced_service_with_path`](./struct.Service.html#method.proxy_core_v1_get_namespaced_service_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedServiceWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1GETNamespacedServiceWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1GETNamespacedServiceWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1GETNamespacedServiceWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1GETNamespacedServiceWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PATCHNamespacedService

impl Service {
    /// proxy PATCH requests to Service
    ///
    /// Use [`ProxyCoreV1PATCHNamespacedServiceResponse`](./enum.ProxyCoreV1PATCHNamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn proxy_core_v1_patch_namespaced_service(
        name: &str,
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_patch_namespaced_service`](./struct.Service.html#method.proxy_core_v1_patch_namespaced_service)
#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedServiceResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1PATCHNamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1PATCHNamespacedServiceResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PATCHNamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PATCHNamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PATCHNamespacedServiceWithPath

impl Service {
    /// proxy PATCH requests to Service
    ///
    /// Use [`ProxyCoreV1PATCHNamespacedServiceWithPathResponse`](./enum.ProxyCoreV1PATCHNamespacedServiceWithPathResponse.html) to parse the HTTP response.
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
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_patch_namespaced_service_with_path(
        name: &str,
        namespace: &str,
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_patch_namespaced_service_with_path`](./struct.Service.html#method.proxy_core_v1_patch_namespaced_service_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedServiceWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1PATCHNamespacedServiceWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1PATCHNamespacedServiceWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PATCHNamespacedServiceWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PATCHNamespacedServiceWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1POSTNamespacedService

impl Service {
    /// proxy POST requests to Service
    ///
    /// Use [`ProxyCoreV1POSTNamespacedServiceResponse`](./enum.ProxyCoreV1POSTNamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn proxy_core_v1_post_namespaced_service(
        name: &str,
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_post_namespaced_service`](./struct.Service.html#method.proxy_core_v1_post_namespaced_service)
#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedServiceResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1POSTNamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1POSTNamespacedServiceResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1POSTNamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1POSTNamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1POSTNamespacedServiceWithPath

impl Service {
    /// proxy POST requests to Service
    ///
    /// Use [`ProxyCoreV1POSTNamespacedServiceWithPathResponse`](./enum.ProxyCoreV1POSTNamespacedServiceWithPathResponse.html) to parse the HTTP response.
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
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_post_namespaced_service_with_path(
        name: &str,
        namespace: &str,
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_post_namespaced_service_with_path`](./struct.Service.html#method.proxy_core_v1_post_namespaced_service_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedServiceWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1POSTNamespacedServiceWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1POSTNamespacedServiceWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1POSTNamespacedServiceWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1POSTNamespacedServiceWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PUTNamespacedService

impl Service {
    /// proxy PUT requests to Service
    ///
    /// Use [`ProxyCoreV1PUTNamespacedServiceResponse`](./enum.ProxyCoreV1PUTNamespacedServiceResponse.html) to parse the HTTP response.
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
    pub fn proxy_core_v1_put_namespaced_service(
        name: &str,
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_put_namespaced_service`](./struct.Service.html#method.proxy_core_v1_put_namespaced_service)
#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedServiceResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1PUTNamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1PUTNamespacedServiceResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PUTNamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PUTNamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PUTNamespacedServiceWithPath

impl Service {
    /// proxy PUT requests to Service
    ///
    /// Use [`ProxyCoreV1PUTNamespacedServiceWithPathResponse`](./enum.ProxyCoreV1PUTNamespacedServiceWithPathResponse.html) to parse the HTTP response.
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
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_put_namespaced_service_with_path(
        name: &str,
        namespace: &str,
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::proxy_core_v1_put_namespaced_service_with_path`](./struct.Service.html#method.proxy_core_v1_put_namespaced_service_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedServiceWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl ::Response for ProxyCoreV1PUTNamespacedServiceWithPathResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                let result = result.to_string();
                let len = result.len();
                Ok((ProxyCoreV1PUTNamespacedServiceWithPathResponse::Ok(result), len))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PUTNamespacedServiceWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PUTNamespacedServiceWithPathResponse::Other, 0)),
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
    pub fn read_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::read_core_v1_namespaced_service`](./struct.Service.html#method.read_core_v1_namespaced_service)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedServiceResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for ReadCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedServiceResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_core_v1_namespaced_service_status(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Service::read_core_v1_namespaced_service_status`](./struct.Service.html#method.read_core_v1_namespaced_service_status)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedServiceStatusResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for ReadCoreV1NamespacedServiceStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedServiceStatusResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Service,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Service::replace_core_v1_namespaced_service`](./struct.Service.html#method.replace_core_v1_namespaced_service)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedServiceResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceCoreV1NamespacedServiceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedServiceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedServiceResponse::Unauthorized, 0)),
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
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_core_v1_namespaced_service_status(
        name: &str,
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Service,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/services/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Service::replace_core_v1_namespaced_service_status`](./struct.Service.html#method.replace_core_v1_namespaced_service_status)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedServiceStatusResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceCoreV1NamespacedServiceStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedService

impl Service {
    /// watch changes to an object of kind Service
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
    pub fn watch_core_v1_namespaced_service(
        name: &str,
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/services/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::watch_core_v1_namespaced_service`](./struct.Service.html#method.watch_core_v1_namespaced_service)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedServiceResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCoreV1NamespacedServiceResponse {
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
                Ok((WatchCoreV1NamespacedServiceResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedServiceResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedServiceList

impl Service {
    /// watch individual changes to a list of Service
    ///
    /// Use [`WatchCoreV1NamespacedServiceListResponse`](./enum.WatchCoreV1NamespacedServiceListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn watch_core_v1_namespaced_service_list(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/services?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::watch_core_v1_namespaced_service_list`](./struct.Service.html#method.watch_core_v1_namespaced_service_list)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedServiceListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCoreV1NamespacedServiceListResponse {
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
                Ok((WatchCoreV1NamespacedServiceListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedServiceListResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedServiceListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1ServiceListForAllNamespaces

impl Service {
    /// watch individual changes to a list of Service
    ///
    /// Use [`WatchCoreV1ServiceListForAllNamespacesResponse`](./enum.WatchCoreV1ServiceListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
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
    pub fn watch_core_v1_service_list_for_all_namespaces(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/watch/services?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Service::watch_core_v1_service_list_for_all_namespaces`](./struct.Service.html#method.watch_core_v1_service_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchCoreV1ServiceListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCoreV1ServiceListForAllNamespacesResponse {
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
                Ok((WatchCoreV1ServiceListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1ServiceListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1ServiceListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End /v1/Service

impl ::Resource for Service {
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

impl<'de> ::serde::Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Service;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Service")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::api::v1::ServiceSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::api::v1::ServiceStatus> = None;

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
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Service",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
