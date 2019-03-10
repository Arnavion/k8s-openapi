// Generated from definition io.k8s.api.core.v1.Pod

/// Pod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pod {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_13::api::core::v1::PodSpec>,

    /// Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_13::api::core::v1::PodStatus>,
}

// Begin /v1/Pod

// Generated from operation connectCoreV1DeleteNamespacedPodProxy

impl Pod {
    /// connect DELETE requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectDeleteNamespacedPodProxyResponse`]`>` constructor, or [`ConnectDeleteNamespacedPodProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_delete_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectDeleteNamespacedPodProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectDeleteNamespacedPodProxyResponse>), crate::RequestError> {
        let ConnectDeleteNamespacedPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
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

/// Optional parameters of [`Pod::connect_delete_namespaced_pod_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNamespacedPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

/// Use `<ConnectDeleteNamespacedPodProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_delete_namespaced_pod_proxy`]
#[derive(Debug)]
pub enum ConnectDeleteNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectDeleteNamespacedPodProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectDeleteNamespacedPodProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectDeleteNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectDeleteNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1DeleteNamespacedPodProxyWithPath

impl Pod {
    /// connect DELETE requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectDeleteNamespacedPodProxyWithPathResponse`]`>` constructor, or [`ConnectDeleteNamespacedPodProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
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
    pub fn connect_delete_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectDeleteNamespacedPodProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectDeleteNamespacedPodProxyWithPathResponse>), crate::RequestError> {
        let ConnectDeleteNamespacedPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
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

/// Optional parameters of [`Pod::connect_delete_namespaced_pod_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNamespacedPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectDeleteNamespacedPodProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_delete_namespaced_pod_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectDeleteNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectDeleteNamespacedPodProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectDeleteNamespacedPodProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectDeleteNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectDeleteNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodAttach

impl Pod {
    /// connect GET requests to attach of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNamespacedPodAttachResponse`]`>` constructor, or [`ConnectGetNamespacedPodAttachResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodAttachOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_namespaced_pod_attach(
        name: &str,
        namespace: &str,
        optional: ConnectGetNamespacedPodAttachOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNamespacedPodAttachResponse>), crate::RequestError> {
        let ConnectGetNamespacedPodAttachOptional {
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_get_namespaced_pod_attach`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedPodAttachOptional<'a> {
    /// The container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
    pub stderr: Option<bool>,
    /// Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
    pub tty: Option<bool>,
}

/// Use `<ConnectGetNamespacedPodAttachResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_get_namespaced_pod_attach`]
#[derive(Debug)]
pub enum ConnectGetNamespacedPodAttachResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNamespacedPodAttachResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectGetNamespacedPodAttachResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNamespacedPodAttachResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNamespacedPodAttachResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodExec

impl Pod {
    /// connect GET requests to exec of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNamespacedPodExecResponse`]`>` constructor, or [`ConnectGetNamespacedPodExecResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodExecOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_namespaced_pod_exec(
        name: &str,
        namespace: &str,
        optional: ConnectGetNamespacedPodExecOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNamespacedPodExecResponse>), crate::RequestError> {
        let ConnectGetNamespacedPodExecOptional {
            command,
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_get_namespaced_pod_exec`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedPodExecOptional<'a> {
    /// Command is the remote command to execute. argv array. Not executed within a shell.
    pub command: Option<&'a str>,
    /// Container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Redirect the standard error stream of the pod for this call. Defaults to true.
    pub stderr: Option<bool>,
    /// Redirect the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Redirect the standard output stream of the pod for this call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
    pub tty: Option<bool>,
}

/// Use `<ConnectGetNamespacedPodExecResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_get_namespaced_pod_exec`]
#[derive(Debug)]
pub enum ConnectGetNamespacedPodExecResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNamespacedPodExecResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectGetNamespacedPodExecResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNamespacedPodExecResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNamespacedPodExecResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodPortforward

impl Pod {
    /// connect GET requests to portforward of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNamespacedPodPortforwardResponse`]`>` constructor, or [`ConnectGetNamespacedPodPortforwardResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodPortForwardOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_namespaced_pod_portforward(
        name: &str,
        namespace: &str,
        optional: ConnectGetNamespacedPodPortforwardOptional,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNamespacedPodPortforwardResponse>), crate::RequestError> {
        let ConnectGetNamespacedPodPortforwardOptional {
            ports,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
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

/// Optional parameters of [`Pod::connect_get_namespaced_pod_portforward`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedPodPortforwardOptional {
    /// List of ports to forward Required when using WebSockets
    pub ports: Option<i64>,
}

/// Use `<ConnectGetNamespacedPodPortforwardResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_get_namespaced_pod_portforward`]
#[derive(Debug)]
pub enum ConnectGetNamespacedPodPortforwardResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNamespacedPodPortforwardResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectGetNamespacedPodPortforwardResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNamespacedPodPortforwardResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNamespacedPodPortforwardResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodProxy

impl Pod {
    /// connect GET requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNamespacedPodProxyResponse`]`>` constructor, or [`ConnectGetNamespacedPodProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectGetNamespacedPodProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNamespacedPodProxyResponse>), crate::RequestError> {
        let ConnectGetNamespacedPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
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

/// Optional parameters of [`Pod::connect_get_namespaced_pod_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

/// Use `<ConnectGetNamespacedPodProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_get_namespaced_pod_proxy`]
#[derive(Debug)]
pub enum ConnectGetNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNamespacedPodProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectGetNamespacedPodProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNamespacedPodProxyWithPath

impl Pod {
    /// connect GET requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNamespacedPodProxyWithPathResponse`]`>` constructor, or [`ConnectGetNamespacedPodProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
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
    pub fn connect_get_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectGetNamespacedPodProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNamespacedPodProxyWithPathResponse>), crate::RequestError> {
        let ConnectGetNamespacedPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
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

/// Optional parameters of [`Pod::connect_get_namespaced_pod_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNamespacedPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectGetNamespacedPodProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_get_namespaced_pod_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectGetNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNamespacedPodProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectGetNamespacedPodProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNamespacedPodProxy

impl Pod {
    /// connect PATCH requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPatchNamespacedPodProxyResponse`]`>` constructor, or [`ConnectPatchNamespacedPodProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_patch_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPatchNamespacedPodProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPatchNamespacedPodProxyResponse>), crate::RequestError> {
        let ConnectPatchNamespacedPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_patch_namespaced_pod_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNamespacedPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

/// Use `<ConnectPatchNamespacedPodProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_patch_namespaced_pod_proxy`]
#[derive(Debug)]
pub enum ConnectPatchNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPatchNamespacedPodProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPatchNamespacedPodProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPatchNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectPatchNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNamespacedPodProxyWithPath

impl Pod {
    /// connect PATCH requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPatchNamespacedPodProxyWithPathResponse`]`>` constructor, or [`ConnectPatchNamespacedPodProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
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
    pub fn connect_patch_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPatchNamespacedPodProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPatchNamespacedPodProxyWithPathResponse>), crate::RequestError> {
        let ConnectPatchNamespacedPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_patch_namespaced_pod_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNamespacedPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectPatchNamespacedPodProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_patch_namespaced_pod_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectPatchNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPatchNamespacedPodProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPatchNamespacedPodProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPatchNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectPatchNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodAttach

impl Pod {
    /// connect POST requests to attach of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNamespacedPodAttachResponse`]`>` constructor, or [`ConnectPostNamespacedPodAttachResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodAttachOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_namespaced_pod_attach(
        name: &str,
        namespace: &str,
        optional: ConnectPostNamespacedPodAttachOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNamespacedPodAttachResponse>), crate::RequestError> {
        let ConnectPostNamespacedPodAttachOptional {
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_post_namespaced_pod_attach`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedPodAttachOptional<'a> {
    /// The container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
    pub stderr: Option<bool>,
    /// Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
    pub tty: Option<bool>,
}

/// Use `<ConnectPostNamespacedPodAttachResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_post_namespaced_pod_attach`]
#[derive(Debug)]
pub enum ConnectPostNamespacedPodAttachResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNamespacedPodAttachResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPostNamespacedPodAttachResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNamespacedPodAttachResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNamespacedPodAttachResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodExec

impl Pod {
    /// connect POST requests to exec of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNamespacedPodExecResponse`]`>` constructor, or [`ConnectPostNamespacedPodExecResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodExecOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_namespaced_pod_exec(
        name: &str,
        namespace: &str,
        optional: ConnectPostNamespacedPodExecOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNamespacedPodExecResponse>), crate::RequestError> {
        let ConnectPostNamespacedPodExecOptional {
            command,
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_post_namespaced_pod_exec`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedPodExecOptional<'a> {
    /// Command is the remote command to execute. argv array. Not executed within a shell.
    pub command: Option<&'a str>,
    /// Container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Redirect the standard error stream of the pod for this call. Defaults to true.
    pub stderr: Option<bool>,
    /// Redirect the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Redirect the standard output stream of the pod for this call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
    pub tty: Option<bool>,
}

/// Use `<ConnectPostNamespacedPodExecResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_post_namespaced_pod_exec`]
#[derive(Debug)]
pub enum ConnectPostNamespacedPodExecResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNamespacedPodExecResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPostNamespacedPodExecResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNamespacedPodExecResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNamespacedPodExecResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodPortforward

impl Pod {
    /// connect POST requests to portforward of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNamespacedPodPortforwardResponse`]`>` constructor, or [`ConnectPostNamespacedPodPortforwardResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodPortForwardOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_namespaced_pod_portforward(
        name: &str,
        namespace: &str,
        optional: ConnectPostNamespacedPodPortforwardOptional,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNamespacedPodPortforwardResponse>), crate::RequestError> {
        let ConnectPostNamespacedPodPortforwardOptional {
            ports,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_post_namespaced_pod_portforward`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedPodPortforwardOptional {
    /// List of ports to forward Required when using WebSockets
    pub ports: Option<i64>,
}

/// Use `<ConnectPostNamespacedPodPortforwardResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_post_namespaced_pod_portforward`]
#[derive(Debug)]
pub enum ConnectPostNamespacedPodPortforwardResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNamespacedPodPortforwardResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPostNamespacedPodPortforwardResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNamespacedPodPortforwardResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNamespacedPodPortforwardResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodProxy

impl Pod {
    /// connect POST requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNamespacedPodProxyResponse`]`>` constructor, or [`ConnectPostNamespacedPodProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPostNamespacedPodProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNamespacedPodProxyResponse>), crate::RequestError> {
        let ConnectPostNamespacedPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_post_namespaced_pod_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

/// Use `<ConnectPostNamespacedPodProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_post_namespaced_pod_proxy`]
#[derive(Debug)]
pub enum ConnectPostNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNamespacedPodProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPostNamespacedPodProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNamespacedPodProxyWithPath

impl Pod {
    /// connect POST requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNamespacedPodProxyWithPathResponse`]`>` constructor, or [`ConnectPostNamespacedPodProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
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
    pub fn connect_post_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPostNamespacedPodProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNamespacedPodProxyWithPathResponse>), crate::RequestError> {
        let ConnectPostNamespacedPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_post_namespaced_pod_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNamespacedPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectPostNamespacedPodProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_post_namespaced_pod_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectPostNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNamespacedPodProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPostNamespacedPodProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNamespacedPodProxy

impl Pod {
    /// connect PUT requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPutNamespacedPodProxyResponse`]`>` constructor, or [`ConnectPutNamespacedPodProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_put_namespaced_pod_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPutNamespacedPodProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPutNamespacedPodProxyResponse>), crate::RequestError> {
        let ConnectPutNamespacedPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_put_namespaced_pod_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNamespacedPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

/// Use `<ConnectPutNamespacedPodProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_put_namespaced_pod_proxy`]
#[derive(Debug)]
pub enum ConnectPutNamespacedPodProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPutNamespacedPodProxyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPutNamespacedPodProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPutNamespacedPodProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectPutNamespacedPodProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNamespacedPodProxyWithPath

impl Pod {
    /// connect PUT requests to proxy of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPutNamespacedPodProxyWithPathResponse`]`>` constructor, or [`ConnectPutNamespacedPodProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
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
    pub fn connect_put_namespaced_pod_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPutNamespacedPodProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPutNamespacedPodProxyWithPathResponse>), crate::RequestError> {
        let ConnectPutNamespacedPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::connect_put_namespaced_pod_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNamespacedPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectPutNamespacedPodProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::connect_put_namespaced_pod_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectPutNamespacedPodProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPutNamespacedPodProxyWithPathResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ConnectPutNamespacedPodProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPutNamespacedPodProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectPutNamespacedPodProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation createCoreV1NamespacedPod

impl Pod {
    /// create a Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedPodResponse`]`>` constructor, or [`CreateNamespacedPodResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_pod(
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Pod,
        optional: CreateNamespacedPodOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedPodResponse>), crate::RequestError> {
        let CreateNamespacedPodOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
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

/// Optional parameters of [`Pod::create_namespaced_pod`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedPodOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::create_namespaced_pod`]
#[derive(Debug)]
pub enum CreateNamespacedPodResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Created(crate::v1_13::api::core::v1::Pod),
    Accepted(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPod

impl Pod {
    /// delete collection of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedPodResponse`]`>` constructor, or [`DeleteCollectionNamespacedPodResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_namespaced_pod(
        namespace: &str,
        optional: DeleteCollectionNamespacedPodOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedPodResponse>), crate::RequestError> {
        let DeleteCollectionNamespacedPodOptional {
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::delete_collection_namespaced_pod`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionNamespacedPodOptional<'a> {
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

/// Use `<DeleteCollectionNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::delete_collection_namespaced_pod`]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedPodResponse {
    OkStatus(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNamespacedPodResponse {
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
                    Ok((DeleteCollectionNamespacedPodResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedPodResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedPod

impl Pod {
    /// delete a Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedPodResponse`]`>` constructor, or [`DeleteNamespacedPodResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_pod(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedPodOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedPodResponse>), crate::RequestError> {
        let DeleteNamespacedPodOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`Pod::delete_namespaced_pod`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespacedPodOptional<'a> {
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

/// Use `<DeleteNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::delete_namespaced_pod`]
#[derive(Debug)]
pub enum DeleteNamespacedPodResponse {
    OkStatus(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_13::api::core::v1::Pod),
    Accepted(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespacedPodResponse {
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
                    Ok((DeleteNamespacedPodResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedPodResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespacedPodResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1NamespacedPod

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedPodResponse`]`>` constructor, or [`ListNamespacedPodResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_pod(
        namespace: &str,
        optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedPodResponse>), crate::RequestError> {
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
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ListNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::list_namespaced_pod`]
#[derive(Debug)]
pub enum ListNamespacedPodResponse {
    Ok(crate::v1_13::api::core::v1::PodList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((ListNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1PodForAllNamespaces

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListPodForAllNamespacesResponse`]`>` constructor, or [`ListPodForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_pod_for_all_namespaces(
        optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListPodForAllNamespacesResponse>), crate::RequestError> {
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
        let __url = "/api/v1/pods?".to_string();
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

/// Use `<ListPodForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::list_pod_for_all_namespaces`]
#[derive(Debug)]
pub enum ListPodForAllNamespacesResponse {
    Ok(crate::v1_13::api::core::v1::PodList),
    Unauthorized,
    Other,
}

impl crate::Response for ListPodForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListPodForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListPodForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListPodForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPod

impl Pod {
    /// partially update the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedPodResponse`]`>` constructor, or [`PatchNamespacedPodResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_pod(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPodOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedPodResponse>), crate::RequestError> {
        let PatchNamespacedPodOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`Pod::patch_namespaced_pod`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedPodOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::patch_namespaced_pod`]
#[derive(Debug)]
pub enum PatchNamespacedPodResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPodStatus

impl Pod {
    /// partially update status of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedPodStatusResponse`]`>` constructor, or [`PatchNamespacedPodStatusResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_pod_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPodStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedPodStatusResponse>), crate::RequestError> {
        let PatchNamespacedPodStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`Pod::patch_namespaced_pod_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedPodStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedPodStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::patch_namespaced_pod_status`]
#[derive(Debug)]
pub enum PatchNamespacedPodStatusResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedPodStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPodStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedPodStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedPodStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPod

impl Pod {
    /// read the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedPodResponse`]`>` constructor, or [`ReadNamespacedPodResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedPodResponse>), crate::RequestError> {
        let ReadNamespacedPodOptional {
            exact,
            export,
            pretty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::read_namespaced_pod`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedPodOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read_namespaced_pod`]
#[derive(Debug)]
pub enum ReadNamespacedPodResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPodLog

impl Pod {
    /// read log of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedPodLogResponse`]`>` constructor, or [`ReadNamespacedPodLogResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod_log(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodLogOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedPodLogResponse>), crate::RequestError> {
        let ReadNamespacedPodLogOptional {
            container,
            follow,
            limit_bytes,
            pretty,
            previous,
            since_seconds,
            tail_lines,
            timestamps,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::read_namespaced_pod_log`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedPodLogOptional<'a> {
    /// The container for which to stream logs. Defaults to only container if there is one container in the pod.
    pub container: Option<&'a str>,
    /// Follow the log stream of the pod. Defaults to false.
    pub follow: Option<bool>,
    /// If set, the number of bytes to read from the server before terminating the log output. This may not display a complete final line of logging, and may return slightly more or slightly less than the specified limit.
    pub limit_bytes: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Return previous terminated container logs. Defaults to false.
    pub previous: Option<bool>,
    /// A relative time in seconds before the current time from which to show logs. If this value precedes the time a pod was started, only logs since the pod start will be returned. If this value is in the future, no logs will be returned. Only one of sinceSeconds or sinceTime may be specified.
    pub since_seconds: Option<i64>,
    /// If set, the number of lines from the end of the logs to show. If not specified, logs are shown from the creation of the container or sinceSeconds or sinceTime
    pub tail_lines: Option<i64>,
    /// If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line of log output. Defaults to false.
    pub timestamps: Option<bool>,
}

/// Use `<ReadNamespacedPodLogResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read_namespaced_pod_log`]
#[derive(Debug)]
pub enum ReadNamespacedPodLogResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPodLogResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ReadNamespacedPodLogResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPodLogResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPodLogResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPodStatus

impl Pod {
    /// read status of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedPodStatusResponse`]`>` constructor, or [`ReadNamespacedPodStatusResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedPodStatusResponse>), crate::RequestError> {
        let ReadNamespacedPodStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`Pod::read_namespaced_pod_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedPodStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedPodStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read_namespaced_pod_status`]
#[derive(Debug)]
pub enum ReadNamespacedPodStatusResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPodStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPodStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPodStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPodStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPod

impl Pod {
    /// replace the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedPodResponse`]`>` constructor, or [`ReplaceNamespacedPodResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_pod(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Pod,
        optional: ReplaceNamespacedPodOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedPodResponse>), crate::RequestError> {
        let ReplaceNamespacedPodOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`Pod::replace_namespaced_pod`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedPodOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::replace_namespaced_pod`]
#[derive(Debug)]
pub enum ReplaceNamespacedPodResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Created(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedPodResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPodStatus

impl Pod {
    /// replace status of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedPodStatusResponse`]`>` constructor, or [`ReplaceNamespacedPodStatusResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_pod_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::core::v1::Pod,
        optional: ReplaceNamespacedPodStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedPodStatusResponse>), crate::RequestError> {
        let ReplaceNamespacedPodStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`Pod::replace_namespaced_pod_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedPodStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedPodStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::replace_namespaced_pod_status`]
#[derive(Debug)]
pub enum ReplaceNamespacedPodStatusResponse {
    Ok(crate::v1_13::api::core::v1::Pod),
    Created(crate::v1_13::api::core::v1::Pod),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedPodStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedPodStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedPodStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPod

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedPodResponse`]`>` constructor, or [`WatchNamespacedPodResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_pod(
        namespace: &str,
        optional: crate::v1_13::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedPodResponse>), crate::RequestError> {
        let crate::v1_13::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
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

/// Use `<WatchNamespacedPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::watch_namespaced_pod`]
#[derive(Debug)]
pub enum WatchNamespacedPodResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedPodResponse {
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
                Ok((WatchNamespacedPodResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedPodResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedPodResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1PodForAllNamespaces

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchPodForAllNamespacesResponse`]`>` constructor, or [`WatchPodForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_pod_for_all_namespaces(
        optional: crate::v1_13::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchPodForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_13::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/pods?".to_string();
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

/// Use `<WatchPodForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::watch_pod_for_all_namespaces`]
#[derive(Debug)]
pub enum WatchPodForAllNamespacesResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchPodForAllNamespacesResponse {
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
                Ok((WatchPodForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchPodForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchPodForAllNamespacesResponse::Other, 0)),
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
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
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
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_13::api::core::v1::PodSpec> = None;
                let mut value_status: Option<crate::v1_13::api::core::v1::PodStatus> = None;

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
