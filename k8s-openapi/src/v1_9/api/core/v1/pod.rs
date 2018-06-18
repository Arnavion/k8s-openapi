// Generated from definition io.k8s.api.core.v1.Pod

/// Pod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pod {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_9::api::core::v1::PodSpec>,

    /// Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<::v1_9::api::core::v1::PodStatus>,
}

// Generated from operation connectCoreV1DeleteNamespacedPodProxy

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedPodProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect DELETE requests to proxy of Pod
    pub fn connect_core_v1_delete_namespaced_pod_proxy<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1DeleteNamespacedPodProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ConnectCoreV1DeleteNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNamespacedPodProxyResponse::Unauthorized(response),
            other => ConnectCoreV1DeleteNamespacedPodProxyResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1DeleteNamespacedPodProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect DELETE requests to proxy of Pod
    pub fn connect_core_v1_delete_namespaced_pod_proxy_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1GetNamespacedPodAttach

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodAttachResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect GET requests to attach of Pod
    pub fn connect_core_v1_get_namespaced_pod_attach<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
        stderr: Option<bool>,
        // Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
        tty: Option<bool>,
    ) -> Result<ConnectCoreV1GetNamespacedPodAttachResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/attach", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(container) = container {
                __query_pairs.append_pair("container", &container);
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1GetNamespacedPodAttachResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodAttachResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedPodAttachResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1GetNamespacedPodExec

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodExecResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect GET requests to exec of Pod
    pub fn connect_core_v1_get_namespaced_pod_exec<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Command is the remote command to execute. argv array. Not executed within a shell.
        command: Option<&str>,
        // Container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Redirect the standard error stream of the pod for this call. Defaults to true.
        stderr: Option<bool>,
        // Redirect the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Redirect the standard output stream of the pod for this call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
        tty: Option<bool>,
    ) -> Result<ConnectCoreV1GetNamespacedPodExecResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/exec", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(command) = command {
                __query_pairs.append_pair("command", &command);
            }
            if let Some(container) = container {
                __query_pairs.append_pair("container", &container);
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1GetNamespacedPodExecResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodExecResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedPodExecResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1GetNamespacedPodPortforward

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodPortforwardResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect GET requests to portforward of Pod
    pub fn connect_core_v1_get_namespaced_pod_portforward<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // List of ports to forward Required when using WebSockets
        ports: Option<i64>,
    ) -> Result<ConnectCoreV1GetNamespacedPodPortforwardResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(ports) = ports {
                __query_pairs.append_pair("ports", &ports.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1GetNamespacedPodPortforwardResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodPortforwardResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedPodPortforwardResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1GetNamespacedPodProxy

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect GET requests to proxy of Pod
    pub fn connect_core_v1_get_namespaced_pod_proxy<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1GetNamespacedPodProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ConnectCoreV1GetNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodProxyResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedPodProxyResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1GetNamespacedPodProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect GET requests to proxy of Pod
    pub fn connect_core_v1_get_namespaced_pod_proxy_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1GetNamespacedPodProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PatchNamespacedPodProxy

#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedPodProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect PATCH requests to proxy of Pod
    pub fn connect_core_v1_patch_namespaced_pod_proxy<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PatchNamespacedPodProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ConnectCoreV1PatchNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNamespacedPodProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PatchNamespacedPodProxyResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PatchNamespacedPodProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedPodProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect PATCH requests to proxy of Pod
    pub fn connect_core_v1_patch_namespaced_pod_proxy_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PatchNamespacedPodProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PostNamespacedPodAttach

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodAttachResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect POST requests to attach of Pod
    pub fn connect_core_v1_post_namespaced_pod_attach<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
        stderr: Option<bool>,
        // Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
        tty: Option<bool>,
    ) -> Result<ConnectCoreV1PostNamespacedPodAttachResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/attach", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(container) = container {
                __query_pairs.append_pair("container", &container);
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
        }

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PostNamespacedPodAttachResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodAttachResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedPodAttachResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PostNamespacedPodExec

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodExecResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect POST requests to exec of Pod
    pub fn connect_core_v1_post_namespaced_pod_exec<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Command is the remote command to execute. argv array. Not executed within a shell.
        command: Option<&str>,
        // Container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Redirect the standard error stream of the pod for this call. Defaults to true.
        stderr: Option<bool>,
        // Redirect the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Redirect the standard output stream of the pod for this call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
        tty: Option<bool>,
    ) -> Result<ConnectCoreV1PostNamespacedPodExecResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/exec", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(command) = command {
                __query_pairs.append_pair("command", &command);
            }
            if let Some(container) = container {
                __query_pairs.append_pair("container", &container);
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
        }

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PostNamespacedPodExecResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodExecResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedPodExecResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PostNamespacedPodPortforward

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodPortforwardResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect POST requests to portforward of Pod
    pub fn connect_core_v1_post_namespaced_pod_portforward<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // List of ports to forward Required when using WebSockets
        ports: Option<i64>,
    ) -> Result<ConnectCoreV1PostNamespacedPodPortforwardResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(ports) = ports {
                __query_pairs.append_pair("ports", &ports.to_string());
            }
        }

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ConnectCoreV1PostNamespacedPodPortforwardResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodPortforwardResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedPodPortforwardResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PostNamespacedPodProxy

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect POST requests to proxy of Pod
    pub fn connect_core_v1_post_namespaced_pod_proxy<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PostNamespacedPodProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ConnectCoreV1PostNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedPodProxyResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PostNamespacedPodProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect POST requests to proxy of Pod
    pub fn connect_core_v1_post_namespaced_pod_proxy_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PostNamespacedPodProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PutNamespacedPodProxy

#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedPodProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect PUT requests to proxy of Pod
    pub fn connect_core_v1_put_namespaced_pod_proxy<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PutNamespacedPodProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ConnectCoreV1PutNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNamespacedPodProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PutNamespacedPodProxyResponse::Other(other, response),
        })
    }

}

// Generated from operation connectCoreV1PutNamespacedPodProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedPodProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// connect PUT requests to proxy of Pod
    pub fn connect_core_v1_put_namespaced_pod_proxy_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PutNamespacedPodProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation createCoreV1NamespacedPod

#[derive(Debug)]
pub enum CreateCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Created(::v1_9::api::core::v1::Pod),
    Accepted(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// create a Pod
    pub fn create_core_v1_namespaced_pod<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::core::v1::Pod,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods", namespace = namespace)).map_err(::Error::URL)?;
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
                CreateCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCoreV1NamespacedPodResponse::Created(result)
            },
            ::http::StatusCode::ACCEPTED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCoreV1NamespacedPodResponse::Accepted(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCoreV1NamespacedPodResponse::Unauthorized(response),
            other => CreateCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation deleteCoreV1CollectionNamespacedPod

#[derive(Debug)]
pub enum DeleteCoreV1CollectionNamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// delete collection of Pod
    pub fn delete_core_v1_collection_namespaced_pod<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<DeleteCoreV1CollectionNamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCoreV1CollectionNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1CollectionNamespacedPodResponse::Unauthorized(response),
            other => DeleteCoreV1CollectionNamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation deleteCoreV1NamespacedPod

#[derive(Debug)]
pub enum DeleteCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// delete a Pod
    pub fn delete_core_v1_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(grace_period_seconds) = grace_period_seconds {
                __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
            }
            if let Some(orphan_dependents) = orphan_dependents {
                __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(propagation_policy) = propagation_policy {
                __query_pairs.append_pair("propagationPolicy", &propagation_policy);
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1NamespacedPodResponse::Unauthorized(response),
            other => DeleteCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation listCoreV1NamespacedPod

#[derive(Debug)]
pub enum ListCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::PodList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// list or watch objects of kind Pod
    pub fn list_core_v1_namespaced_pod<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                ListCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1NamespacedPodResponse::Unauthorized(response),
            other => ListCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation listCoreV1PodForAllNamespaces

#[derive(Debug)]
pub enum ListCoreV1PodForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::PodList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// list or watch objects of kind Pod
    pub fn list_core_v1_pod_for_all_namespaces<C>(
        __client: &C,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1PodForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/pods")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                ListCoreV1PodForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1PodForAllNamespacesResponse::Unauthorized(response),
            other => ListCoreV1PodForAllNamespacesResponse::Other(other, response),
        })
    }

}

// Generated from operation patchCoreV1NamespacedPod

#[derive(Debug)]
pub enum PatchCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// partially update the specified Pod
    pub fn patch_core_v1_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedPodResponse::Unauthorized(response),
            other => PatchCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation patchCoreV1NamespacedPodStatus

#[derive(Debug)]
pub enum PatchCoreV1NamespacedPodStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// partially update status of the specified Pod
    pub fn patch_core_v1_namespaced_pod_status<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NamespacedPodStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchCoreV1NamespacedPodStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedPodStatusResponse::Unauthorized(response),
            other => PatchCoreV1NamespacedPodStatusResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1DELETENamespacedPod

#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy DELETE requests to Pod
    pub fn proxy_core_v1_delete_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1DELETENamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1DELETENamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1DELETENamespacedPodResponse::Unauthorized(response),
            other => ProxyCoreV1DELETENamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1DELETENamespacedPodWithPath

#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedPodWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy DELETE requests to Pod
    pub fn proxy_core_v1_delete_namespaced_pod_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1DELETENamespacedPodWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1DELETENamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1DELETENamespacedPodWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1DELETENamespacedPodWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1GETNamespacedPod

#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy GET requests to Pod
    pub fn proxy_core_v1_get_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1GETNamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1GETNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1GETNamespacedPodResponse::Unauthorized(response),
            other => ProxyCoreV1GETNamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1GETNamespacedPodWithPath

#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedPodWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy GET requests to Pod
    pub fn proxy_core_v1_get_namespaced_pod_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1GETNamespacedPodWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1GETNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1GETNamespacedPodWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1GETNamespacedPodWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1PATCHNamespacedPod

#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy PATCH requests to Pod
    pub fn proxy_core_v1_patch_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1PATCHNamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.patch(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PATCHNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PATCHNamespacedPodResponse::Unauthorized(response),
            other => ProxyCoreV1PATCHNamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1PATCHNamespacedPodWithPath

#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedPodWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy PATCH requests to Pod
    pub fn proxy_core_v1_patch_namespaced_pod_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1PATCHNamespacedPodWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.patch(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PATCHNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PATCHNamespacedPodWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1PATCHNamespacedPodWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1POSTNamespacedPod

#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy POST requests to Pod
    pub fn proxy_core_v1_post_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1POSTNamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1POSTNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1POSTNamespacedPodResponse::Unauthorized(response),
            other => ProxyCoreV1POSTNamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1POSTNamespacedPodWithPath

#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedPodWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy POST requests to Pod
    pub fn proxy_core_v1_post_namespaced_pod_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1POSTNamespacedPodWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.post(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1POSTNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1POSTNamespacedPodWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1POSTNamespacedPodWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1PUTNamespacedPod

#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy PUT requests to Pod
    pub fn proxy_core_v1_put_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<ProxyCoreV1PUTNamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;

        let response = __client.put(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PUTNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PUTNamespacedPodResponse::Unauthorized(response),
            other => ProxyCoreV1PUTNamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation proxyCoreV1PUTNamespacedPodWithPath

#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedPodWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// proxy PUT requests to Pod
    pub fn proxy_core_v1_put_namespaced_pod_with_path<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<ProxyCoreV1PUTNamespacedPodWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let __url = __client.base_url().join(&format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path)).map_err(::Error::URL)?;

        let response = __client.put(__url, &()).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ProxyCoreV1PUTNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PUTNamespacedPodWithPathResponse::Unauthorized(response),
            other => ProxyCoreV1PUTNamespacedPodWithPathResponse::Other(other, response),
        })
    }

}

// Generated from operation readCoreV1NamespacedPod

#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// read the specified Pod
    pub fn read_core_v1_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedPodResponse::Unauthorized(response),
            other => ReadCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation readCoreV1NamespacedPodLog

#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodLogResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// read log of the specified Pod
    pub fn read_core_v1_namespaced_pod_log<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The container for which to stream logs. Defaults to only container if there is one container in the pod.
        container: Option<&str>,
        // Follow the log stream of the pod. Defaults to false.
        follow: Option<bool>,
        // If set, the number of bytes to read from the server before terminating the log output. This may not display a complete final line of logging, and may return slightly more or slightly less than the specified limit.
        limit_bytes: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Return previous terminated container logs. Defaults to false.
        previous: Option<bool>,
        // A relative time in seconds before the current time from which to show logs. If this value precedes the time a pod was started, only logs since the pod start will be returned. If this value is in the future, no logs will be returned. Only one of sinceSeconds or sinceTime may be specified.
        since_seconds: Option<i64>,
        // If set, the number of lines from the end of the logs to show. If not specified, logs are shown from the creation of the container or sinceSeconds or sinceTime
        tail_lines: Option<i64>,
        // If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line of log output. Defaults to false.
        timestamps: Option<bool>,
    ) -> Result<ReadCoreV1NamespacedPodLogResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/log", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(container) = container {
                __query_pairs.append_pair("container", &container);
            }
            if let Some(follow) = follow {
                __query_pairs.append_pair("follow", &follow.to_string());
            }
            if let Some(limit_bytes) = limit_bytes {
                __query_pairs.append_pair("limitBytes", &limit_bytes.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let mut response = response;
                let mut result = String::new();
                ::std::io::Read::read_to_string(&mut response, &mut result).map_err(::Error::IO)?;
                ReadCoreV1NamespacedPodLogResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedPodLogResponse::Unauthorized(response),
            other => ReadCoreV1NamespacedPodLogResponse::Other(other, response),
        })
    }

}

// Generated from operation readCoreV1NamespacedPodStatus

#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// read status of the specified Pod
    pub fn read_core_v1_namespaced_pod_status<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NamespacedPodStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadCoreV1NamespacedPodStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedPodStatusResponse::Unauthorized(response),
            other => ReadCoreV1NamespacedPodStatusResponse::Other(other, response),
        })
    }

}

// Generated from operation replaceCoreV1NamespacedPod

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Created(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// replace the specified Pod
    pub fn replace_core_v1_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::core::v1::Pod,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NamespacedPodResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedPodResponse::Unauthorized(response),
            other => ReplaceCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation replaceCoreV1NamespacedPodStatus

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPodStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::core::v1::Pod),
    Created(::v1_9::api::core::v1::Pod),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// replace status of the specified Pod
    pub fn replace_core_v1_namespaced_pod_status<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::core::v1::Pod,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NamespacedPodStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/pods/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceCoreV1NamespacedPodStatusResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NamespacedPodStatusResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedPodStatusResponse::Unauthorized(response),
            other => ReplaceCoreV1NamespacedPodStatusResponse::Other(other, response),
        })
    }

}

// Generated from operation watchCoreV1NamespacedPod

pub enum WatchCoreV1NamespacedPodResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// watch changes to an object of kind Pod
    pub fn watch_core_v1_namespaced_pod<C>(
        __client: &C,
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NamespacedPodResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                WatchCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedPodResponse::Unauthorized(response),
            other => WatchCoreV1NamespacedPodResponse::Other(other, response),
        })
    }

}

// Generated from operation watchCoreV1NamespacedPodList

pub enum WatchCoreV1NamespacedPodListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// watch individual changes to a list of Pod
    pub fn watch_core_v1_namespaced_pod_list<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NamespacedPodListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/namespaces/{namespace}/pods", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                WatchCoreV1NamespacedPodListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedPodListResponse::Unauthorized(response),
            other => WatchCoreV1NamespacedPodListResponse::Other(other, response),
        })
    }

}

// Generated from operation watchCoreV1PodListForAllNamespaces

pub enum WatchCoreV1PodListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Pod {
    /// watch individual changes to a list of Pod
    pub fn watch_core_v1_pod_list_for_all_namespaces<C>(
        __client: &C,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1PodListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/pods")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                WatchCoreV1PodListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1PodListForAllNamespacesResponse::Unauthorized(response),
            other => WatchCoreV1PodListForAllNamespacesResponse::Other(other, response),
        })
    }

}

impl<'de> ::serde::Deserialize<'de> for Pod {
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
            type Value = Pod;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Pod")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_9::api::core::v1::PodSpec> = None;
                let mut value_status: Option<::v1_9::api::core::v1::PodStatus> = None;

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

                Ok(Pod {
                    api_version: value_api_version,
                    kind: value_kind,
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

impl ::serde::Serialize for Pod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Pod",
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
