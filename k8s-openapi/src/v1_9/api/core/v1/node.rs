// Generated from definition io.k8s.api.core.v1.Node

/// Node is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in etcd).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Node {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a node. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_9::api::core::v1::NodeSpec>,

    /// Most recently observed status of the node. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_9::api::core::v1::NodeStatus>,
}

// Begin /v1/Node

// Generated from operation connectCoreV1DeleteNodeProxy

impl Node {
    /// connect DELETE requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1DeleteNodeProxyResponse`](./enum.ConnectCoreV1DeleteNodeProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_delete_node_proxy(
        name: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Parses the HTTP response of [`Node::connect_core_v1_delete_node_proxy`](./struct.Node.html#method.connect_core_v1_delete_node_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1DeleteNodeProxyResponse {
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
                Ok((ConnectCoreV1DeleteNodeProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1DeleteNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1DeleteNodeProxyWithPath

impl Node {
    /// connect DELETE requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1DeleteNodeProxyWithPathResponse`](./enum.ConnectCoreV1DeleteNodeProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_delete_node_proxy_with_path(
        name: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Parses the HTTP response of [`Node::connect_core_v1_delete_node_proxy_with_path`](./struct.Node.html#method.connect_core_v1_delete_node_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1DeleteNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1DeleteNodeProxyWithPathResponse {
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
                Ok((ConnectCoreV1DeleteNodeProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1DeleteNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1DeleteNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNodeProxy

impl Node {
    /// connect GET requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1GetNodeProxyResponse`](./enum.ConnectCoreV1GetNodeProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_get_node_proxy(
        name: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Parses the HTTP response of [`Node::connect_core_v1_get_node_proxy`](./struct.Node.html#method.connect_core_v1_get_node_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1GetNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNodeProxyResponse {
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
                Ok((ConnectCoreV1GetNodeProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNodeProxyWithPath

impl Node {
    /// connect GET requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1GetNodeProxyWithPathResponse`](./enum.ConnectCoreV1GetNodeProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_get_node_proxy_with_path(
        name: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Parses the HTTP response of [`Node::connect_core_v1_get_node_proxy_with_path`](./struct.Node.html#method.connect_core_v1_get_node_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1GetNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1GetNodeProxyWithPathResponse {
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
                Ok((ConnectCoreV1GetNodeProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1GetNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1GetNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNodeProxy

impl Node {
    /// connect PATCH requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1PatchNodeProxyResponse`](./enum.ConnectCoreV1PatchNodeProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_patch_node_proxy(
        name: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Parses the HTTP response of [`Node::connect_core_v1_patch_node_proxy`](./struct.Node.html#method.connect_core_v1_patch_node_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PatchNodeProxyResponse {
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
                Ok((ConnectCoreV1PatchNodeProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PatchNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNodeProxyWithPath

impl Node {
    /// connect PATCH requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1PatchNodeProxyWithPathResponse`](./enum.ConnectCoreV1PatchNodeProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_patch_node_proxy_with_path(
        name: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Parses the HTTP response of [`Node::connect_core_v1_patch_node_proxy_with_path`](./struct.Node.html#method.connect_core_v1_patch_node_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PatchNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PatchNodeProxyWithPathResponse {
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
                Ok((ConnectCoreV1PatchNodeProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PatchNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PatchNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNodeProxy

impl Node {
    /// connect POST requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1PostNodeProxyResponse`](./enum.ConnectCoreV1PostNodeProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_post_node_proxy(
        name: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Parses the HTTP response of [`Node::connect_core_v1_post_node_proxy`](./struct.Node.html#method.connect_core_v1_post_node_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PostNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNodeProxyResponse {
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
                Ok((ConnectCoreV1PostNodeProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNodeProxyWithPath

impl Node {
    /// connect POST requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1PostNodeProxyWithPathResponse`](./enum.ConnectCoreV1PostNodeProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_post_node_proxy_with_path(
        name: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Parses the HTTP response of [`Node::connect_core_v1_post_node_proxy_with_path`](./struct.Node.html#method.connect_core_v1_post_node_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PostNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PostNodeProxyWithPathResponse {
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
                Ok((ConnectCoreV1PostNodeProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PostNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PostNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNodeProxy

impl Node {
    /// connect PUT requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1PutNodeProxyResponse`](./enum.ConnectCoreV1PutNodeProxyResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_put_node_proxy(
        name: &str,
        path: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Parses the HTTP response of [`Node::connect_core_v1_put_node_proxy`](./struct.Node.html#method.connect_core_v1_put_node_proxy)
#[derive(Debug)]
pub enum ConnectCoreV1PutNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PutNodeProxyResponse {
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
                Ok((ConnectCoreV1PutNodeProxyResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PutNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNodeProxyWithPath

impl Node {
    /// connect PUT requests to proxy of Node
    ///
    /// Use [`ConnectCoreV1PutNodeProxyWithPathResponse`](./enum.ConnectCoreV1PutNodeProxyWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `path_`
    ///
    ///     Path is the URL path to use for the current proxy request to node.
    pub fn connect_core_v1_put_node_proxy_with_path(
        name: &str,
        path: &str,
        path_: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Parses the HTTP response of [`Node::connect_core_v1_put_node_proxy_with_path`](./struct.Node.html#method.connect_core_v1_put_node_proxy_with_path)
#[derive(Debug)]
pub enum ConnectCoreV1PutNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectCoreV1PutNodeProxyWithPathResponse {
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
                Ok((ConnectCoreV1PutNodeProxyWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectCoreV1PutNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectCoreV1PutNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation createCoreV1Node

impl Node {
    /// create a Node
    ///
    /// Use [`CreateCoreV1NodeResponse`](./enum.CreateCoreV1NodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_core_v1_node(
        body: &crate::v1_9::api::core::v1::Node,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes?");
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

/// Parses the HTTP response of [`Node::create_core_v1_node`](./struct.Node.html#method.create_core_v1_node)
#[derive(Debug)]
pub enum CreateCoreV1NodeResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Created(crate::v1_9::api::core::v1::Node),
    Accepted(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCoreV1NodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NodeResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NodeResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((CreateCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNode

impl Node {
    /// delete collection of Node
    ///
    /// Use [`DeleteCoreV1CollectionNodeResponse`](./enum.DeleteCoreV1CollectionNodeResponse.html) to parse the HTTP response.
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
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn delete_core_v1_collection_node(
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
        let __url = format!("/api/v1/nodes?");
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

/// Parses the HTTP response of [`Node::delete_core_v1_collection_node`](./struct.Node.html#method.delete_core_v1_collection_node)
#[derive(Debug)]
pub enum DeleteCoreV1CollectionNodeResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1CollectionNodeResponse {
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
                    Ok((DeleteCoreV1CollectionNodeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1CollectionNodeResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1CollectionNodeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1CollectionNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1Node

impl Node {
    /// delete a Node
    ///
    /// Use [`DeleteCoreV1NodeResponse`](./enum.DeleteCoreV1NodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
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
    pub fn delete_core_v1_node(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}?", name = name);
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

/// Parses the HTTP response of [`Node::delete_core_v1_node`](./struct.Node.html#method.delete_core_v1_node)
#[derive(Debug)]
pub enum DeleteCoreV1NodeResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCoreV1NodeResponse {
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
                    Ok((DeleteCoreV1NodeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCoreV1NodeResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1Node

impl Node {
    /// list or watch objects of kind Node
    ///
    /// Use [`ListCoreV1NodeResponse`](./enum.ListCoreV1NodeResponse.html) to parse the HTTP response.
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
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_core_v1_node(
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
        let __url = format!("/api/v1/nodes?");
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

/// Parses the HTTP response of [`Node::list_core_v1_node`](./struct.Node.html#method.list_core_v1_node)
#[derive(Debug)]
pub enum ListCoreV1NodeResponse {
    Ok(crate::v1_9::api::core::v1::NodeList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1NodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1NodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1Node

impl Node {
    /// partially update the specified Node
    ///
    /// Use [`PatchCoreV1NodeResponse`](./enum.PatchCoreV1NodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_core_v1_node(
        name: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}?", name = name);
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

/// Parses the HTTP response of [`Node::patch_core_v1_node`](./struct.Node.html#method.patch_core_v1_node)
#[derive(Debug)]
pub enum PatchCoreV1NodeResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NodeStatus

impl Node {
    /// partially update status of the specified Node
    ///
    /// Use [`PatchCoreV1NodeStatusResponse`](./enum.PatchCoreV1NodeStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_core_v1_node_status(
        name: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/status?", name = name);
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

/// Parses the HTTP response of [`Node::patch_core_v1_node_status`](./struct.Node.html#method.patch_core_v1_node_status)
#[derive(Debug)]
pub enum PatchCoreV1NodeStatusResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NodeStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NodeStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NodeStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NodeStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1DELETENode

impl Node {
    /// proxy DELETE requests to Node
    ///
    /// Use [`ProxyCoreV1DELETENodeResponse`](./enum.ProxyCoreV1DELETENodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    pub fn proxy_core_v1_delete_node(
        name: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}", name = name);

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_delete_node`](./struct.Node.html#method.proxy_core_v1_delete_node)
#[derive(Debug)]
pub enum ProxyCoreV1DELETENodeResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1DELETENodeResponse {
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
                Ok((ProxyCoreV1DELETENodeResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1DELETENodeResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1DELETENodeResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1DELETENodeWithPath

impl Node {
    /// proxy DELETE requests to Node
    ///
    /// Use [`ProxyCoreV1DELETENodeWithPathResponse`](./enum.ProxyCoreV1DELETENodeWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_delete_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}", name = name, path = path);

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_delete_node_with_path`](./struct.Node.html#method.proxy_core_v1_delete_node_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1DELETENodeWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1DELETENodeWithPathResponse {
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
                Ok((ProxyCoreV1DELETENodeWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1DELETENodeWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1DELETENodeWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1GETNode

impl Node {
    /// proxy GET requests to Node
    ///
    /// Use [`ProxyCoreV1GETNodeResponse`](./enum.ProxyCoreV1GETNodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    pub fn proxy_core_v1_get_node(
        name: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}", name = name);

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_get_node`](./struct.Node.html#method.proxy_core_v1_get_node)
#[derive(Debug)]
pub enum ProxyCoreV1GETNodeResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1GETNodeResponse {
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
                Ok((ProxyCoreV1GETNodeResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1GETNodeResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1GETNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1GETNodeWithPath

impl Node {
    /// proxy GET requests to Node
    ///
    /// Use [`ProxyCoreV1GETNodeWithPathResponse`](./enum.ProxyCoreV1GETNodeWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_get_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}", name = name, path = path);

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_get_node_with_path`](./struct.Node.html#method.proxy_core_v1_get_node_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1GETNodeWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1GETNodeWithPathResponse {
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
                Ok((ProxyCoreV1GETNodeWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1GETNodeWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1GETNodeWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PATCHNode

impl Node {
    /// proxy PATCH requests to Node
    ///
    /// Use [`ProxyCoreV1PATCHNodeResponse`](./enum.ProxyCoreV1PATCHNodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    pub fn proxy_core_v1_patch_node(
        name: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}", name = name);

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_patch_node`](./struct.Node.html#method.proxy_core_v1_patch_node)
#[derive(Debug)]
pub enum ProxyCoreV1PATCHNodeResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1PATCHNodeResponse {
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
                Ok((ProxyCoreV1PATCHNodeResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PATCHNodeResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PATCHNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PATCHNodeWithPath

impl Node {
    /// proxy PATCH requests to Node
    ///
    /// Use [`ProxyCoreV1PATCHNodeWithPathResponse`](./enum.ProxyCoreV1PATCHNodeWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_patch_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}", name = name, path = path);

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_patch_node_with_path`](./struct.Node.html#method.proxy_core_v1_patch_node_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1PATCHNodeWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1PATCHNodeWithPathResponse {
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
                Ok((ProxyCoreV1PATCHNodeWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PATCHNodeWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PATCHNodeWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1POSTNode

impl Node {
    /// proxy POST requests to Node
    ///
    /// Use [`ProxyCoreV1POSTNodeResponse`](./enum.ProxyCoreV1POSTNodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    pub fn proxy_core_v1_post_node(
        name: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}", name = name);

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_post_node`](./struct.Node.html#method.proxy_core_v1_post_node)
#[derive(Debug)]
pub enum ProxyCoreV1POSTNodeResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1POSTNodeResponse {
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
                Ok((ProxyCoreV1POSTNodeResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1POSTNodeResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1POSTNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1POSTNodeWithPath

impl Node {
    /// proxy POST requests to Node
    ///
    /// Use [`ProxyCoreV1POSTNodeWithPathResponse`](./enum.ProxyCoreV1POSTNodeWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_post_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}", name = name, path = path);

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_post_node_with_path`](./struct.Node.html#method.proxy_core_v1_post_node_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1POSTNodeWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1POSTNodeWithPathResponse {
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
                Ok((ProxyCoreV1POSTNodeWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1POSTNodeWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1POSTNodeWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PUTNode

impl Node {
    /// proxy PUT requests to Node
    ///
    /// Use [`ProxyCoreV1PUTNodeResponse`](./enum.ProxyCoreV1PUTNodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    pub fn proxy_core_v1_put_node(
        name: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}", name = name);

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_put_node`](./struct.Node.html#method.proxy_core_v1_put_node)
#[derive(Debug)]
pub enum ProxyCoreV1PUTNodeResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1PUTNodeResponse {
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
                Ok((ProxyCoreV1PUTNodeResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PUTNodeResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PUTNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation proxyCoreV1PUTNodeWithPath

impl Node {
    /// proxy PUT requests to Node
    ///
    /// Use [`ProxyCoreV1PUTNodeWithPathResponse`](./enum.ProxyCoreV1PUTNodeWithPathResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `path`
    ///
    ///     path to the resource
    pub fn proxy_core_v1_put_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}", name = name, path = path);

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Node::proxy_core_v1_put_node_with_path`](./struct.Node.html#method.proxy_core_v1_put_node_with_path)
#[derive(Debug)]
pub enum ProxyCoreV1PUTNodeWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ProxyCoreV1PUTNodeWithPathResponse {
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
                Ok((ProxyCoreV1PUTNodeWithPathResponse::Ok(result), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ProxyCoreV1PUTNodeWithPathResponse::Unauthorized, 0)),
            _ => Ok((ProxyCoreV1PUTNodeWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1Node

impl Node {
    /// read the specified Node
    ///
    /// Use [`ReadCoreV1NodeResponse`](./enum.ReadCoreV1NodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
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
    pub fn read_core_v1_node(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}?", name = name);
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

/// Parses the HTTP response of [`Node::read_core_v1_node`](./struct.Node.html#method.read_core_v1_node)
#[derive(Debug)]
pub enum ReadCoreV1NodeResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NodeStatus

impl Node {
    /// read status of the specified Node
    ///
    /// Use [`ReadCoreV1NodeStatusResponse`](./enum.ReadCoreV1NodeStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_core_v1_node_status(
        name: &str,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/status?", name = name);
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

/// Parses the HTTP response of [`Node::read_core_v1_node_status`](./struct.Node.html#method.read_core_v1_node_status)
#[derive(Debug)]
pub enum ReadCoreV1NodeStatusResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NodeStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NodeStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NodeStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NodeStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1Node

impl Node {
    /// replace the specified Node
    ///
    /// Use [`ReplaceCoreV1NodeResponse`](./enum.ReplaceCoreV1NodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_core_v1_node(
        name: &str,
        body: &crate::v1_9::api::core::v1::Node,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}?", name = name);
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

/// Parses the HTTP response of [`Node::replace_core_v1_node`](./struct.Node.html#method.replace_core_v1_node)
#[derive(Debug)]
pub enum ReplaceCoreV1NodeResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Created(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NodeResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NodeStatus

impl Node {
    /// replace status of the specified Node
    ///
    /// Use [`ReplaceCoreV1NodeStatusResponse`](./enum.ReplaceCoreV1NodeStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_core_v1_node_status(
        name: &str,
        body: &crate::v1_9::api::core::v1::Node,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/status?", name = name);
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

/// Parses the HTTP response of [`Node::replace_core_v1_node_status`](./struct.Node.html#method.replace_core_v1_node_status)
#[derive(Debug)]
pub enum ReplaceCoreV1NodeStatusResponse {
    Ok(crate::v1_9::api::core::v1::Node),
    Created(crate::v1_9::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NodeStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NodeStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NodeStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NodeStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NodeStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1Node

impl Node {
    /// watch changes to an object of kind Node
    ///
    /// Use [`WatchCoreV1NodeResponse`](./enum.WatchCoreV1NodeResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
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
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_core_v1_node(
        name: &str,
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
        let __url = format!("/api/v1/watch/nodes/{name}?", name = name);
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

/// Parses the HTTP response of [`Node::watch_core_v1_node`](./struct.Node.html#method.watch_core_v1_node)
#[derive(Debug)]
pub enum WatchCoreV1NodeResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NodeResponse {
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
                Ok((WatchCoreV1NodeResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NodeResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NodeResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NodeList

impl Node {
    /// watch individual changes to a list of Node
    ///
    /// Use [`WatchCoreV1NodeListResponse`](./enum.WatchCoreV1NodeListResponse.html) to parse the HTTP response.
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
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_core_v1_node_list(
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
        let __url = format!("/api/v1/watch/nodes?");
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

/// Parses the HTTP response of [`Node::watch_core_v1_node_list`](./struct.Node.html#method.watch_core_v1_node_list)
#[derive(Debug)]
pub enum WatchCoreV1NodeListResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCoreV1NodeListResponse {
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
                Ok((WatchCoreV1NodeListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NodeListResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NodeListResponse::Other, 0)),
        }
    }
}

// End /v1/Node

impl crate::Resource for Node {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "Node"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl<'de> serde::Deserialize<'de> for Node {
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
            type Value = Node;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Node")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_9::api::core::v1::NodeSpec> = None;
                let mut value_status: Option<crate::v1_9::api::core::v1::NodeStatus> = None;

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

                Ok(Node {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Node",
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

impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Node",
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
