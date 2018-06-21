// Generated from definition io.k8s.kubernetes.pkg.api.v1.Service

/// Service is a named abstraction of software service (for example, mysql) consisting of local port (for example 3306) that the proxy listens on, and the selector that determines which pods will answer requests sent through the proxy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Service {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_7::kubernetes::pkg::api::v1::ServiceSpec>,

    /// Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<::v1_7::kubernetes::pkg::api::v1::ServiceStatus>,
}

// Begin /v1/Service

// Generated from operation connectCoreV1DeleteNamespacedServiceProxy

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedServiceProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect DELETE requests to proxy of Service
    pub fn connect_core_v1_delete_namespaced_service_proxy<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1DeleteNamespacedServiceProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path) = path {
                __query_pairs.append_pair("path", &path);
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1DeleteNamespacedServiceProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNamespacedServiceProxyResponse::Unauthorized(response),
            other => ConnectCoreV1DeleteNamespacedServiceProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1DeleteNamespacedServiceProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect DELETE requests to proxy of Service
    pub fn connect_core_v1_delete_namespaced_service_proxy_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path_) = path_ {
                __query_pairs.append_pair("path", &path_);
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1DeleteNamespacedServiceProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedServiceProxy

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedServiceProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect GET requests to proxy of Service
    pub fn connect_core_v1_get_namespaced_service_proxy<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1GetNamespacedServiceProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path) = path {
                __query_pairs.append_pair("path", &path);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1GetNamespacedServiceProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedServiceProxyResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedServiceProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedServiceProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedServiceProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect GET requests to proxy of Service
    pub fn connect_core_v1_get_namespaced_service_proxy_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1GetNamespacedServiceProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path_) = path_ {
                __query_pairs.append_pair("path", &path_);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedServiceProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PatchNamespacedServiceProxy

#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedServiceProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect PATCH requests to proxy of Service
    pub fn connect_core_v1_patch_namespaced_service_proxy<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PatchNamespacedServiceProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path) = path {
                __query_pairs.append_pair("path", &path);
            }
        }

        let response = __client.patch(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PatchNamespacedServiceProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNamespacedServiceProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PatchNamespacedServiceProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PatchNamespacedServiceProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect PATCH requests to proxy of Service
    pub fn connect_core_v1_patch_namespaced_service_proxy_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path_) = path_ {
                __query_pairs.append_pair("path", &path_);
            }
        }

        let response = __client.patch(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PatchNamespacedServiceProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedServiceProxy

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedServiceProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect POST requests to proxy of Service
    pub fn connect_core_v1_post_namespaced_service_proxy<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PostNamespacedServiceProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path) = path {
                __query_pairs.append_pair("path", &path);
            }
        }

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PostNamespacedServiceProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedServiceProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedServiceProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedServiceProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedServiceProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect POST requests to proxy of Service
    pub fn connect_core_v1_post_namespaced_service_proxy_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PostNamespacedServiceProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path_) = path_ {
                __query_pairs.append_pair("path", &path_);
            }
        }

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedServiceProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PutNamespacedServiceProxy

#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedServiceProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect PUT requests to proxy of Service
    pub fn connect_core_v1_put_namespaced_service_proxy<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PutNamespacedServiceProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path) = path {
                __query_pairs.append_pair("path", &path);
            }
        }

        let response = __client.put(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PutNamespacedServiceProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNamespacedServiceProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PutNamespacedServiceProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PutNamespacedServiceProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedServiceProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// connect PUT requests to proxy of Service
    pub fn connect_core_v1_put_namespaced_service_proxy_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the part of URLs that include service endpoints, suffixes, and parameters to use for the current proxy request to service. For example, the whole request URL is http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy. Path is _search?q=user:kimchy.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PutNamespacedServiceProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(path_) = path_ {
                __query_pairs.append_pair("path", &path_);
            }
        }

        let response = __client.put(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PutNamespacedServiceProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation createCoreV1NamespacedService

#[derive(Debug)]
pub enum CreateCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// create a Service
    pub fn create_core_v1_namespaced_service<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Service,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.post(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => CreateCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1NamespacedService

#[derive(Debug)]
pub enum DeleteCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// delete a Service
    pub fn delete_core_v1_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<DeleteCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => DeleteCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation listCoreV1NamespacedService

#[derive(Debug)]
pub enum ListCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::ServiceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// list or watch objects of kind Service
    pub fn list_core_v1_namespaced_service<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => ListCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation listCoreV1ServiceForAllNamespaces

#[derive(Debug)]
pub enum ListCoreV1ServiceForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::ServiceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// list or watch objects of kind Service
    pub fn list_core_v1_service_for_all_namespaces<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1ServiceForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/services")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListCoreV1ServiceForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1ServiceForAllNamespacesResponse::Unauthorized(response),
            other => ListCoreV1ServiceForAllNamespacesResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1NamespacedService

#[derive(Debug)]
pub enum PatchCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// partially update the specified Service
    pub fn patch_core_v1_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => PatchCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1NamespacedServiceStatus

#[derive(Debug)]
pub enum PatchCoreV1NamespacedServiceStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// partially update status of the specified Service
    pub fn patch_core_v1_namespaced_service_status<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NamespacedServiceStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchCoreV1NamespacedServiceStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedServiceStatusResponse::Unauthorized(response),
            other => PatchCoreV1NamespacedServiceStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1DELETENamespacedService

#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy DELETE requests to Service
    pub fn proxy_core_v1_delete_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1DELETENamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1DELETENamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1DELETENamespacedServiceResponse::Unauthorized(response),
            other => ProxyCoreV1DELETENamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1DELETENamespacedServiceWithPath

#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedServiceWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy DELETE requests to Service
    pub fn proxy_core_v1_delete_namespaced_service_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1DELETENamespacedServiceWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1DELETENamespacedServiceWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1DELETENamespacedServiceWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1DELETENamespacedServiceWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1GETNamespacedService

#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy GET requests to Service
    pub fn proxy_core_v1_get_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1GETNamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1GETNamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1GETNamespacedServiceResponse::Unauthorized(response),
            other => ProxyCoreV1GETNamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1GETNamespacedServiceWithPath

#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedServiceWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy GET requests to Service
    pub fn proxy_core_v1_get_namespaced_service_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1GETNamespacedServiceWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1GETNamespacedServiceWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1GETNamespacedServiceWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1GETNamespacedServiceWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1PATCHNamespacedService

#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy PATCH requests to Service
    pub fn proxy_core_v1_patch_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1PATCHNamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.patch(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PATCHNamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PATCHNamespacedServiceResponse::Unauthorized(response),
            other => ProxyCoreV1PATCHNamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1PATCHNamespacedServiceWithPath

#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedServiceWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy PATCH requests to Service
    pub fn proxy_core_v1_patch_namespaced_service_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1PATCHNamespacedServiceWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.patch(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PATCHNamespacedServiceWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PATCHNamespacedServiceWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1PATCHNamespacedServiceWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1POSTNamespacedService

#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy POST requests to Service
    pub fn proxy_core_v1_post_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1POSTNamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1POSTNamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1POSTNamespacedServiceResponse::Unauthorized(response),
            other => ProxyCoreV1POSTNamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1POSTNamespacedServiceWithPath

#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedServiceWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy POST requests to Service
    pub fn proxy_core_v1_post_namespaced_service_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1POSTNamespacedServiceWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1POSTNamespacedServiceWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1POSTNamespacedServiceWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1POSTNamespacedServiceWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1PUTNamespacedService

#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy PUT requests to Service
    pub fn proxy_core_v1_put_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1PUTNamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.put(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PUTNamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PUTNamespacedServiceResponse::Unauthorized(response),
            other => ProxyCoreV1PUTNamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation proxyCoreV1PUTNamespacedServiceWithPath

#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedServiceWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// proxy PUT requests to Service
    pub fn proxy_core_v1_put_namespaced_service_with_path<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1PUTNamespacedServiceWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.put(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PUTNamespacedServiceWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PUTNamespacedServiceWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1PUTNamespacedServiceWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1NamespacedService

#[derive(Debug)]
pub enum ReadCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// read the specified Service
    pub fn read_core_v1_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(exact) = exact {
                __query_pairs.append_pair("exact", &exact.to_string());
            }
            if let Some(export) = export {
                __query_pairs.append_pair("export", &export.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => ReadCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1NamespacedServiceStatus

#[derive(Debug)]
pub enum ReadCoreV1NamespacedServiceStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// read status of the specified Service
    pub fn read_core_v1_namespaced_service_status<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NamespacedServiceStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadCoreV1NamespacedServiceStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedServiceStatusResponse::Unauthorized(response),
            other => ReadCoreV1NamespacedServiceStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1NamespacedService

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// replace the specified Service
    pub fn replace_core_v1_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Service,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => ReplaceCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1NamespacedServiceStatus

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedServiceStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Service),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// replace status of the specified Service
    pub fn replace_core_v1_namespaced_service_status<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Service,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NamespacedServiceStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/services/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NamespacedServiceStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedServiceStatusResponse::Unauthorized(response),
            other => ReplaceCoreV1NamespacedServiceStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1NamespacedService

pub enum WatchCoreV1NamespacedServiceResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// watch changes to an object of kind Service
    pub fn watch_core_v1_namespaced_service<C>(
        __client: &C,
        // name of the Service
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NamespacedServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/namespaces/{namespace}/services/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCoreV1NamespacedServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedServiceResponse::Unauthorized(response),
            other => WatchCoreV1NamespacedServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1NamespacedServiceList

pub enum WatchCoreV1NamespacedServiceListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// watch individual changes to a list of Service
    pub fn watch_core_v1_namespaced_service_list<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NamespacedServiceListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/namespaces/{namespace}/services", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCoreV1NamespacedServiceListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedServiceListResponse::Unauthorized(response),
            other => WatchCoreV1NamespacedServiceListResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1ServiceListForAllNamespaces

pub enum WatchCoreV1ServiceListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Service {
    /// watch individual changes to a list of Service
    pub fn watch_core_v1_service_list_for_all_namespaces<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1ServiceListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/services")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCoreV1ServiceListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1ServiceListForAllNamespacesResponse::Unauthorized(response),
            other => WatchCoreV1ServiceListForAllNamespacesResponse::Other(other, response),
        })
    }
}

// End /v1/Service

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
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::api::v1::ServiceSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::api::v1::ServiceStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Service {
                    api_version: value_api_version,
                    kind: value_kind,
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
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
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
