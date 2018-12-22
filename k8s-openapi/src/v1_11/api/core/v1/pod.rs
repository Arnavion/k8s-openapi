// Generated from definition io.k8s.api.core.v1.Pod

/// Pod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pod {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_11::api::core::v1::PodSpec>,

    /// Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_11::api::core::v1::PodStatus>,
}

// Begin /v1/Pod

// Generated from operation connectCoreV1DeleteNamespacedPodProxy

impl Pod {
    /// connect DELETE requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1DeleteNamespacedPodProxyResponse`](./enum.ConnectCoreV1DeleteNamespacedPodProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_delete_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_delete_namespaced_pod_proxy`](./struct.Pod.html#method.connect_core_v1_delete_namespaced_pod_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1DeleteNamespacedPodProxyResponse {
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
                Ok((ConnectCoreV1DeleteNamespacedPodProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1DeleteNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1DeleteNamespacedPodProxyWithPath

impl Pod {
    /// connect DELETE requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse`](./enum.ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_delete_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_delete_namespaced_pod_proxy_with_path`](./struct.Pod.html#method.connect_core_v1_delete_namespaced_pod_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse {
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
                Ok((ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodAttach

impl Pod {
    /// connect GET requests to attach of Pod
    ///
    /// Use [`ConnectCoreV1GetNamespacedPodAttachResponse`](./enum.ConnectCoreV1GetNamespacedPodAttachResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `container`
    ///
    ///     The container in which to execute the command. Defaults to only container if there is only one container in the pod.
    ///
    /// * `stderr`
    ///
    ///     Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
    ///
    /// * `stdin`
    ///
    ///     Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
    ///
    /// * `stdout`
    ///
    ///     Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
    ///
    /// * `tty`
    ///
    ///     TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
    pub fn connect_core_v1_get_namespaced_pod_attach(
        name: &str,
        namespace: &str,
        container: Option<&str>,
        stderr: Option<bool>,
        stdin: Option<bool>,
        stdout: Option<bool>,
        tty: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/attach?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::connect_core_v1_get_namespaced_pod_attach`](./struct.Pod.html#method.connect_core_v1_get_namespaced_pod_attach)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodAttachResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedPodAttachResponse {
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
                Ok((ConnectCoreV1GetNamespacedPodAttachResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedPodAttachResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedPodAttachResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodExec

impl Pod {
    /// connect GET requests to exec of Pod
    ///
    /// Use [`ConnectCoreV1GetNamespacedPodExecResponse`](./enum.ConnectCoreV1GetNamespacedPodExecResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `command`
    ///
    ///     Command is the remote command to execute. argv array. Not executed within a shell.
    ///
    /// * `container`
    ///
    ///     Container in which to execute the command. Defaults to only container if there is only one container in the pod.
    ///
    /// * `stderr`
    ///
    ///     Redirect the standard error stream of the pod for this call. Defaults to true.
    ///
    /// * `stdin`
    ///
    ///     Redirect the standard input stream of the pod for this call. Defaults to false.
    ///
    /// * `stdout`
    ///
    ///     Redirect the standard output stream of the pod for this call. Defaults to true.
    ///
    /// * `tty`
    ///
    ///     TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
    pub fn connect_core_v1_get_namespaced_pod_exec(
        name: &str,
        namespace: &str,
        command: Option<&str>,
        container: Option<&str>,
        stderr: Option<bool>,
        stdin: Option<bool>,
        stdout: Option<bool>,
        tty: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/exec?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(command) = command {
            __query_pairs.append_pair("command", command);
        }
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::connect_core_v1_get_namespaced_pod_exec`](./struct.Pod.html#method.connect_core_v1_get_namespaced_pod_exec)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodExecResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedPodExecResponse {
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
                Ok((ConnectCoreV1GetNamespacedPodExecResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedPodExecResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedPodExecResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodPortforward

impl Pod {
    /// connect GET requests to portforward of Pod
    ///
    /// Use [`ConnectCoreV1GetNamespacedPodPortforwardResponse`](./enum.ConnectCoreV1GetNamespacedPodPortforwardResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `ports`
    ///
    ///     List of ports to forward Required when using WebSockets
    pub fn connect_core_v1_get_namespaced_pod_portforward(
        name: &str,
        namespace: &str,
        ports: Option<i64>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::connect_core_v1_get_namespaced_pod_portforward`](./struct.Pod.html#method.connect_core_v1_get_namespaced_pod_portforward)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodPortforwardResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedPodPortforwardResponse {
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
                Ok((ConnectCoreV1GetNamespacedPodPortforwardResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedPodPortforwardResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedPodPortforwardResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodProxy

impl Pod {
    /// connect GET requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1GetNamespacedPodProxyResponse`](./enum.ConnectCoreV1GetNamespacedPodProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_get_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_get_namespaced_pod_proxy`](./struct.Pod.html#method.connect_core_v1_get_namespaced_pod_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedPodProxyResponse {
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
                Ok((ConnectCoreV1GetNamespacedPodProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodProxyWithPath

impl Pod {
    /// connect GET requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1GetNamespacedPodProxyWithPathResponse`](./enum.ConnectCoreV1GetNamespacedPodProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_get_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_get_namespaced_pod_proxy_with_path`](./struct.Pod.html#method.connect_core_v1_get_namespaced_pod_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNamespacedPodProxyWithPathResponse {
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
                Ok((ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNamespacedPodProxy

impl Pod {
    /// connect PATCH requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1PatchNamespacedPodProxyResponse`](./enum.ConnectCoreV1PatchNamespacedPodProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_patch_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_patch_namespaced_pod_proxy`](./struct.Pod.html#method.connect_core_v1_patch_namespaced_pod_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PatchNamespacedPodProxyResponse {
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
                Ok((ConnectCoreV1PatchNamespacedPodProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PatchNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNamespacedPodProxyWithPath

impl Pod {
    /// connect PATCH requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1PatchNamespacedPodProxyWithPathResponse`](./enum.ConnectCoreV1PatchNamespacedPodProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_patch_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_patch_namespaced_pod_proxy_with_path`](./struct.Pod.html#method.connect_core_v1_patch_namespaced_pod_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PatchNamespacedPodProxyWithPathResponse {
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
                Ok((ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodAttach

impl Pod {
    /// connect POST requests to attach of Pod
    ///
    /// Use [`ConnectCoreV1PostNamespacedPodAttachResponse`](./enum.ConnectCoreV1PostNamespacedPodAttachResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `container`
    ///
    ///     The container in which to execute the command. Defaults to only container if there is only one container in the pod.
    ///
    /// * `stderr`
    ///
    ///     Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
    ///
    /// * `stdin`
    ///
    ///     Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
    ///
    /// * `stdout`
    ///
    ///     Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
    ///
    /// * `tty`
    ///
    ///     TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
    pub fn connect_core_v1_post_namespaced_pod_attach(
        name: &str,
        namespace: &str,
        container: Option<&str>,
        stderr: Option<bool>,
        stdin: Option<bool>,
        stdout: Option<bool>,
        tty: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/attach?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::connect_core_v1_post_namespaced_pod_attach`](./struct.Pod.html#method.connect_core_v1_post_namespaced_pod_attach)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodAttachResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedPodAttachResponse {
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
                Ok((ConnectCoreV1PostNamespacedPodAttachResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedPodAttachResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedPodAttachResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodExec

impl Pod {
    /// connect POST requests to exec of Pod
    ///
    /// Use [`ConnectCoreV1PostNamespacedPodExecResponse`](./enum.ConnectCoreV1PostNamespacedPodExecResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `command`
    ///
    ///     Command is the remote command to execute. argv array. Not executed within a shell.
    ///
    /// * `container`
    ///
    ///     Container in which to execute the command. Defaults to only container if there is only one container in the pod.
    ///
    /// * `stderr`
    ///
    ///     Redirect the standard error stream of the pod for this call. Defaults to true.
    ///
    /// * `stdin`
    ///
    ///     Redirect the standard input stream of the pod for this call. Defaults to false.
    ///
    /// * `stdout`
    ///
    ///     Redirect the standard output stream of the pod for this call. Defaults to true.
    ///
    /// * `tty`
    ///
    ///     TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
    pub fn connect_core_v1_post_namespaced_pod_exec(
        name: &str,
        namespace: &str,
        command: Option<&str>,
        container: Option<&str>,
        stderr: Option<bool>,
        stdin: Option<bool>,
        stdout: Option<bool>,
        tty: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/exec?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(command) = command {
            __query_pairs.append_pair("command", command);
        }
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::connect_core_v1_post_namespaced_pod_exec`](./struct.Pod.html#method.connect_core_v1_post_namespaced_pod_exec)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodExecResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedPodExecResponse {
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
                Ok((ConnectCoreV1PostNamespacedPodExecResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedPodExecResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedPodExecResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodPortforward

impl Pod {
    /// connect POST requests to portforward of Pod
    ///
    /// Use [`ConnectCoreV1PostNamespacedPodPortforwardResponse`](./enum.ConnectCoreV1PostNamespacedPodPortforwardResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `ports`
    ///
    ///     List of ports to forward Required when using WebSockets
    pub fn connect_core_v1_post_namespaced_pod_portforward(
        name: &str,
        namespace: &str,
        ports: Option<i64>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::connect_core_v1_post_namespaced_pod_portforward`](./struct.Pod.html#method.connect_core_v1_post_namespaced_pod_portforward)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodPortforwardResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedPodPortforwardResponse {
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
                Ok((ConnectCoreV1PostNamespacedPodPortforwardResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedPodPortforwardResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedPodPortforwardResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodProxy

impl Pod {
    /// connect POST requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1PostNamespacedPodProxyResponse`](./enum.ConnectCoreV1PostNamespacedPodProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_post_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_post_namespaced_pod_proxy`](./struct.Pod.html#method.connect_core_v1_post_namespaced_pod_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedPodProxyResponse {
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
                Ok((ConnectCoreV1PostNamespacedPodProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodProxyWithPath

impl Pod {
    /// connect POST requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1PostNamespacedPodProxyWithPathResponse`](./enum.ConnectCoreV1PostNamespacedPodProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_post_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_post_namespaced_pod_proxy_with_path`](./struct.Pod.html#method.connect_core_v1_post_namespaced_pod_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNamespacedPodProxyWithPathResponse {
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
                Ok((ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNamespacedPodProxy

impl Pod {
    /// connect PUT requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1PutNamespacedPodProxyResponse`](./enum.ConnectCoreV1PutNamespacedPodProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_put_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_put_namespaced_pod_proxy`](./struct.Pod.html#method.connect_core_v1_put_namespaced_pod_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PutNamespacedPodProxyResponse {
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
                Ok((ConnectCoreV1PutNamespacedPodProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PutNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNamespacedPodProxyWithPath

impl Pod {
    /// connect PUT requests to proxy of Pod
    ///
    /// Use [`ConnectCoreV1PutNamespacedPodProxyWithPathResponse`](./enum.ConnectCoreV1PutNamespacedPodProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    ///     Path is the URL path to use for the current proxy request to pod.
    pub fn connect_core_v1_put_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
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

/// Parses the HTTP response of [`Pod::connect_core_v1_put_namespaced_pod_proxy_with_path`](./struct.Pod.html#method.connect_core_v1_put_namespaced_pod_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PutNamespacedPodProxyWithPathResponse {
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
                Ok((ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation createCoreV1NamespacedPod

impl Pod {
    /// create a Pod
    ///
    /// Use [`CreateCoreV1NamespacedPodResponse`](./enum.CreateCoreV1NamespacedPodResponse.html) to parse the HTTP response.
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
    pub fn create_core_v1_namespaced_pod(
        namespace: &str,
        body: &crate::v1_11::api::core::v1::Pod,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
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

/// Parses the HTTP response of [`Pod::create_core_v1_namespaced_pod`](./struct.Pod.html#method.create_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum CreateCoreV1NamespacedPodResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Created(crate::v1_11::api::core::v1::Pod),
    Accepted(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCoreV1NamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedPodResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedPodResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((CreateCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPod

impl Pod {
    /// delete collection of Pod
    ///
    /// Use [`DeleteCoreV1CollectionNamespacedPodResponse`](./enum.DeleteCoreV1CollectionNamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn delete_core_v1_collection_namespaced_pod(
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
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

/// Parses the HTTP response of [`Pod::delete_core_v1_collection_namespaced_pod`](./struct.Pod.html#method.delete_core_v1_collection_namespaced_pod)
#[derive(Debug)]
pub enum DeleteCoreV1CollectionNamespacedPodResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1CollectionNamespacedPodResponse {
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
                    Ok((DeleteCoreV1CollectionNamespacedPodResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1CollectionNamespacedPodResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1CollectionNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1CollectionNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedPod

impl Pod {
    /// delete a Pod
    ///
    /// Use [`DeleteCoreV1NamespacedPodResponse`](./enum.DeleteCoreV1NamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn delete_core_v1_namespaced_pod(
        name: &str,
        namespace: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::delete_core_v1_namespaced_pod`](./struct.Pod.html#method.delete_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum DeleteCoreV1NamespacedPodResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1NamespacedPodResponse {
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
                    Ok((DeleteCoreV1NamespacedPodResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1NamespacedPodResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1NamespacedPod

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// Use [`ListCoreV1NamespacedPodResponse`](./enum.ListCoreV1NamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn list_core_v1_namespaced_pod(
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
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

/// Parses the HTTP response of [`Pod::list_core_v1_namespaced_pod`](./struct.Pod.html#method.list_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum ListCoreV1NamespacedPodResponse {
    Ok(crate::v1_11::api::core::v1::PodList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1NamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1NamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1PodForAllNamespaces

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// Use [`ListCoreV1PodForAllNamespacesResponse`](./enum.ListCoreV1PodForAllNamespacesResponse.html) to parse the HTTP response.
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
    pub fn list_core_v1_pod_for_all_namespaces(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/pods?");
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

/// Parses the HTTP response of [`Pod::list_core_v1_pod_for_all_namespaces`](./struct.Pod.html#method.list_core_v1_pod_for_all_namespaces)
#[derive(Debug)]
pub enum ListCoreV1PodForAllNamespacesResponse {
    Ok(crate::v1_11::api::core::v1::PodList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1PodForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1PodForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1PodForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1PodForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPod

impl Pod {
    /// partially update the specified Pod
    ///
    /// Use [`PatchCoreV1NamespacedPodResponse`](./enum.PatchCoreV1NamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    pub fn patch_core_v1_namespaced_pod(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::patch_core_v1_namespaced_pod`](./struct.Pod.html#method.patch_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedPodResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPodStatus

impl Pod {
    /// partially update status of the specified Pod
    ///
    /// Use [`PatchCoreV1NamespacedPodStatusResponse`](./enum.PatchCoreV1NamespacedPodStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    pub fn patch_core_v1_namespaced_pod_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::patch_core_v1_namespaced_pod_status`](./struct.Pod.html#method.patch_core_v1_namespaced_pod_status)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedPodStatusResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedPodStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedPodStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedPodStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedPodStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPod

impl Pod {
    /// read the specified Pod
    ///
    /// Use [`ReadCoreV1NamespacedPodResponse`](./enum.ReadCoreV1NamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    pub fn read_core_v1_namespaced_pod(
        name: &str,
        namespace: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::read_core_v1_namespaced_pod`](./struct.Pod.html#method.read_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPodLog

impl Pod {
    /// read log of the specified Pod
    ///
    /// Use [`ReadCoreV1NamespacedPodLogResponse`](./enum.ReadCoreV1NamespacedPodLogResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `container`
    ///
    ///     The container for which to stream logs. Defaults to only container if there is one container in the pod.
    ///
    /// * `follow`
    ///
    ///     Follow the log stream of the pod. Defaults to false.
    ///
    /// * `limit_bytes`
    ///
    ///     If set, the number of bytes to read from the server before terminating the log output. This may not display a complete final line of logging, and may return slightly more or slightly less than the specified limit.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `previous`
    ///
    ///     Return previous terminated container logs. Defaults to false.
    ///
    /// * `since_seconds`
    ///
    ///     A relative time in seconds before the current time from which to show logs. If this value precedes the time a pod was started, only logs since the pod start will be returned. If this value is in the future, no logs will be returned. Only one of sinceSeconds or sinceTime may be specified.
    ///
    /// * `tail_lines`
    ///
    ///     If set, the number of lines from the end of the logs to show. If not specified, logs are shown from the creation of the container or sinceSeconds or sinceTime
    ///
    /// * `timestamps`
    ///
    ///     If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line of log output. Defaults to false.
    pub fn read_core_v1_namespaced_pod_log(
        name: &str,
        namespace: &str,
        container: Option<&str>,
        follow: Option<bool>,
        limit_bytes: Option<i64>,
        pretty: Option<&str>,
        previous: Option<bool>,
        since_seconds: Option<i64>,
        tail_lines: Option<i64>,
        timestamps: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/log?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(follow) = follow {
            __query_pairs.append_pair("follow", &follow.to_string());
        }
        if let Some(limit_bytes) = limit_bytes {
            __query_pairs.append_pair("limitBytes", &limit_bytes.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(previous) = previous {
            __query_pairs.append_pair("previous", &previous.to_string());
        }
        if let Some(since_seconds) = since_seconds {
            __query_pairs.append_pair("sinceSeconds", &since_seconds.to_string());
        }
        if let Some(tail_lines) = tail_lines {
            __query_pairs.append_pair("tailLines", &tail_lines.to_string());
        }
        if let Some(timestamps) = timestamps {
            __query_pairs.append_pair("timestamps", &timestamps.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Pod::read_core_v1_namespaced_pod_log`](./struct.Pod.html#method.read_core_v1_namespaced_pod_log)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodLogResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedPodLogResponse {
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
                Ok((ReadCoreV1NamespacedPodLogResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedPodLogResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedPodLogResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPodStatus

impl Pod {
    /// read status of the specified Pod
    ///
    /// Use [`ReadCoreV1NamespacedPodStatusResponse`](./enum.ReadCoreV1NamespacedPodStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_core_v1_namespaced_pod_status(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::read_core_v1_namespaced_pod_status`](./struct.Pod.html#method.read_core_v1_namespaced_pod_status)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodStatusResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedPodStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedPodStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedPodStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedPodStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPod

impl Pod {
    /// replace the specified Pod
    ///
    /// Use [`ReplaceCoreV1NamespacedPodResponse`](./enum.ReplaceCoreV1NamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    pub fn replace_core_v1_namespaced_pod(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::api::core::v1::Pod,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::replace_core_v1_namespaced_pod`](./struct.Pod.html#method.replace_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPodResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Created(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPodResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPodStatus

impl Pod {
    /// replace status of the specified Pod
    ///
    /// Use [`ReplaceCoreV1NamespacedPodStatusResponse`](./enum.ReplaceCoreV1NamespacedPodStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
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
    pub fn replace_core_v1_namespaced_pod_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::api::core::v1::Pod,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::replace_core_v1_namespaced_pod_status`](./struct.Pod.html#method.replace_core_v1_namespaced_pod_status)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPodStatusResponse {
    Ok(crate::v1_11::api::core::v1::Pod),
    Created(crate::v1_11::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedPodStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPodStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedPodStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedPodStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedPodStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPod

impl Pod {
    /// watch changes to an object of kind Pod
    ///
    /// Use [`WatchCoreV1NamespacedPodResponse`](./enum.WatchCoreV1NamespacedPodResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn watch_core_v1_namespaced_pod(
        name: &str,
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Pod::watch_core_v1_namespaced_pod`](./struct.Pod.html#method.watch_core_v1_namespaced_pod)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedPodResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NamespacedPodResponse {
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
                Ok((WatchCoreV1NamespacedPodResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPodList

impl Pod {
    /// watch individual changes to a list of Pod
    ///
    /// Use [`WatchCoreV1NamespacedPodListResponse`](./enum.WatchCoreV1NamespacedPodListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn watch_core_v1_namespaced_pod_list(
        namespace: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/pods?", namespace = namespace);
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

/// Parses the HTTP response of [`Pod::watch_core_v1_namespaced_pod_list`](./struct.Pod.html#method.watch_core_v1_namespaced_pod_list)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedPodListResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NamespacedPodListResponse {
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
                Ok((WatchCoreV1NamespacedPodListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedPodListResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedPodListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1PodListForAllNamespaces

impl Pod {
    /// watch individual changes to a list of Pod
    ///
    /// Use [`WatchCoreV1PodListForAllNamespacesResponse`](./enum.WatchCoreV1PodListForAllNamespacesResponse.html) to parse the HTTP response.
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
    pub fn watch_core_v1_pod_list_for_all_namespaces(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/watch/pods?");
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

/// Parses the HTTP response of [`Pod::watch_core_v1_pod_list_for_all_namespaces`](./struct.Pod.html#method.watch_core_v1_pod_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchCoreV1PodListForAllNamespacesResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1PodListForAllNamespacesResponse {
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
                Ok((WatchCoreV1PodListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1PodListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1PodListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End /v1/Pod

impl crate::Resource for Pod {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "Pod"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for Pod {
    type Ty = crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&Self::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Pod {
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
            type Value = Pod;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Pod")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_11::api::core::v1::PodSpec> = None;
                let mut value_status: Option<crate::v1_11::api::core::v1::PodStatus> = None;

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

                Ok(Pod {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Pod",
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

impl serde::Serialize for Pod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Pod",
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
