// Generated from definition io.k8s.api.core.v1.Node

/// Node is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in etcd).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Node {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a node. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_14::api::core::v1::NodeSpec>,

    /// Most recently observed status of the node. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_14::api::core::v1::NodeStatus>,
}

// Begin /v1/Node

// Generated from operation connectCoreV1DeleteNodeProxy

impl Node {
    /// connect DELETE requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectDeleteNodeProxyResponse`]`>` constructor, or [`ConnectDeleteNodeProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_delete_node_proxy(
        name: &str,
        optional: ConnectDeleteNodeProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectDeleteNodeProxyResponse>), crate::RequestError> {
        let ConnectDeleteNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Optional parameters of [`Node::connect_delete_node_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

/// Use `<ConnectDeleteNodeProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_delete_node_proxy`]
#[derive(Debug)]
pub enum ConnectDeleteNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectDeleteNodeProxyResponse {
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
                Ok((ConnectDeleteNodeProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectDeleteNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectDeleteNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1DeleteNodeProxyWithPath

impl Node {
    /// connect DELETE requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectDeleteNodeProxyWithPathResponse`]`>` constructor, or [`ConnectDeleteNodeProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_delete_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectDeleteNodeProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectDeleteNodeProxyWithPathResponse>), crate::RequestError> {
        let ConnectDeleteNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Optional parameters of [`Node::connect_delete_node_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectDeleteNodeProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_delete_node_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectDeleteNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectDeleteNodeProxyWithPathResponse {
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
                Ok((ConnectDeleteNodeProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectDeleteNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectDeleteNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNodeProxy

impl Node {
    /// connect GET requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNodeProxyResponse`]`>` constructor, or [`ConnectGetNodeProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_node_proxy(
        name: &str,
        optional: ConnectGetNodeProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNodeProxyResponse>), crate::RequestError> {
        let ConnectGetNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Optional parameters of [`Node::connect_get_node_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

/// Use `<ConnectGetNodeProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_get_node_proxy`]
#[derive(Debug)]
pub enum ConnectGetNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNodeProxyResponse {
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
                Ok((ConnectGetNodeProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1GetNodeProxyWithPath

impl Node {
    /// connect GET requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectGetNodeProxyWithPathResponse`]`>` constructor, or [`ConnectGetNodeProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_get_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectGetNodeProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectGetNodeProxyWithPathResponse>), crate::RequestError> {
        let ConnectGetNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Optional parameters of [`Node::connect_get_node_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectGetNodeProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_get_node_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectGetNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectGetNodeProxyWithPathResponse {
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
                Ok((ConnectGetNodeProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectGetNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectGetNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNodeProxy

impl Node {
    /// connect PATCH requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPatchNodeProxyResponse`]`>` constructor, or [`ConnectPatchNodeProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_patch_node_proxy(
        name: &str,
        optional: ConnectPatchNodeProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPatchNodeProxyResponse>), crate::RequestError> {
        let ConnectPatchNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Optional parameters of [`Node::connect_patch_node_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

/// Use `<ConnectPatchNodeProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_patch_node_proxy`]
#[derive(Debug)]
pub enum ConnectPatchNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPatchNodeProxyResponse {
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
                Ok((ConnectPatchNodeProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPatchNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectPatchNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PatchNodeProxyWithPath

impl Node {
    /// connect PATCH requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPatchNodeProxyWithPathResponse`]`>` constructor, or [`ConnectPatchNodeProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_patch_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectPatchNodeProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPatchNodeProxyWithPathResponse>), crate::RequestError> {
        let ConnectPatchNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Optional parameters of [`Node::connect_patch_node_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectPatchNodeProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_patch_node_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectPatchNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPatchNodeProxyWithPathResponse {
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
                Ok((ConnectPatchNodeProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPatchNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectPatchNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNodeProxy

impl Node {
    /// connect POST requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNodeProxyResponse`]`>` constructor, or [`ConnectPostNodeProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_node_proxy(
        name: &str,
        optional: ConnectPostNodeProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNodeProxyResponse>), crate::RequestError> {
        let ConnectPostNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Optional parameters of [`Node::connect_post_node_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

/// Use `<ConnectPostNodeProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_post_node_proxy`]
#[derive(Debug)]
pub enum ConnectPostNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNodeProxyResponse {
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
                Ok((ConnectPostNodeProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PostNodeProxyWithPath

impl Node {
    /// connect POST requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPostNodeProxyWithPathResponse`]`>` constructor, or [`ConnectPostNodeProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_post_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectPostNodeProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPostNodeProxyWithPathResponse>), crate::RequestError> {
        let ConnectPostNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Optional parameters of [`Node::connect_post_node_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectPostNodeProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_post_node_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectPostNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPostNodeProxyWithPathResponse {
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
                Ok((ConnectPostNodeProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPostNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectPostNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNodeProxy

impl Node {
    /// connect PUT requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPutNodeProxyResponse`]`>` constructor, or [`ConnectPutNodeProxyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_put_node_proxy(
        name: &str,
        optional: ConnectPutNodeProxyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPutNodeProxyResponse>), crate::RequestError> {
        let ConnectPutNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?", name = name);
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

/// Optional parameters of [`Node::connect_put_node_proxy`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

/// Use `<ConnectPutNodeProxyResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_put_node_proxy`]
#[derive(Debug)]
pub enum ConnectPutNodeProxyResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPutNodeProxyResponse {
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
                Ok((ConnectPutNodeProxyResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPutNodeProxyResponse::Unauthorized, 0)),
            _ => Ok((ConnectPutNodeProxyResponse::Other, 0)),
        }
    }
}

// Generated from operation connectCoreV1PutNodeProxyWithPath

impl Node {
    /// connect PUT requests to proxy of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ConnectPutNodeProxyWithPathResponse`]`>` constructor, or [`ConnectPutNodeProxyWithPathResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NodeProxyOptions
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn connect_put_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectPutNodeProxyWithPathOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ConnectPutNodeProxyWithPathResponse>), crate::RequestError> {
        let ConnectPutNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?", name = name, path = path);
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

/// Optional parameters of [`Node::connect_put_node_proxy_with_path`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

/// Use `<ConnectPutNodeProxyWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::connect_put_node_proxy_with_path`]
#[derive(Debug)]
pub enum ConnectPutNodeProxyWithPathResponse {
    Ok(String),
    Unauthorized,
    Other,
}

impl crate::Response for ConnectPutNodeProxyWithPathResponse {
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
                Ok((ConnectPutNodeProxyWithPathResponse::Ok(result.to_string()), len))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ConnectPutNodeProxyWithPathResponse::Unauthorized, 0)),
            _ => Ok((ConnectPutNodeProxyWithPathResponse::Other, 0)),
        }
    }
}

// Generated from operation createCoreV1Node

impl Node {
    /// create a Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNodeResponse`]`>` constructor, or [`CreateNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_node(
        body: &crate::v1_14::api::core::v1::Node,
        optional: CreateNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNodeResponse>), crate::RequestError> {
        let CreateNodeOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = "/api/v1/nodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
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

/// Optional parameters of [`Node::create_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::create_node`]
#[derive(Debug)]
pub enum CreateNodeResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Created(crate::v1_14::api::core::v1::Node),
    Accepted(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNodeResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNodeResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNodeResponse::Unauthorized, 0)),
            _ => Ok((CreateNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNode

impl Node {
    /// delete collection of Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNodeResponse`]`>` constructor, or [`DeleteCollectionNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_node(
        optional: DeleteCollectionNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNodeResponse>), crate::RequestError> {
        let DeleteCollectionNodeOptional {
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = "/api/v1/nodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
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

/// Optional parameters of [`Node::delete_collection_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionNodeOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
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

/// Use `<DeleteCollectionNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::delete_collection_node`]
#[derive(Debug)]
pub enum DeleteCollectionNodeResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNodeResponse {
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
                    Ok((DeleteCollectionNodeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNodeResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNodeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1Node

impl Node {
    /// delete a Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNodeResponse`]`>` constructor, or [`DeleteNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_node(
        name: &str,
        optional: DeleteNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNodeResponse>), crate::RequestError> {
        let DeleteNodeOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}?", name = name);
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

/// Optional parameters of [`Node::delete_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNodeOptional<'a> {
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

/// Use `<DeleteNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::delete_node`]
#[derive(Debug)]
pub enum DeleteNodeResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::core::v1::Node),
    Accepted(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNodeResponse {
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
                    Ok((DeleteNodeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNodeResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNodeResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNodeResponse::Unauthorized, 0)),
            _ => Ok((DeleteNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1Node

impl Node {
    /// list or watch objects of kind Node
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNodeResponse`]`>` constructor, or [`ListNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_node(
        optional: crate::v1_14::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNodeResponse>), crate::RequestError> {
        let crate::v1_14::ListOptional {
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/nodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
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

/// Use `<ListNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::list_node`]
#[derive(Debug)]
pub enum ListNodeResponse {
    Ok(crate::v1_14::api::core::v1::NodeList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNodeResponse::Unauthorized, 0)),
            _ => Ok((ListNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1Node

impl Node {
    /// partially update the specified Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNodeResponse`]`>` constructor, or [`PatchNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_node(
        name: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNodeResponse>), crate::RequestError> {
        let PatchNodeOptional {
            dry_run,
            field_manager,
            force,
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", &force.to_string());
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

/// Optional parameters of [`Node::patch_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::patch_node`]
#[derive(Debug)]
pub enum PatchNodeResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNodeResponse::Unauthorized, 0)),
            _ => Ok((PatchNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NodeStatus

impl Node {
    /// partially update status of the specified Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNodeStatusResponse`]`>` constructor, or [`PatchNodeStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_node_status(
        name: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNodeStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNodeStatusResponse>), crate::RequestError> {
        let PatchNodeStatusOptional {
            dry_run,
            field_manager,
            force,
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", &force.to_string());
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

/// Optional parameters of [`Node::patch_node_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNodeStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNodeStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::patch_node_status`]
#[derive(Debug)]
pub enum PatchNodeStatusResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNodeStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNodeStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNodeStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNodeStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1Node

impl Node {
    /// read the specified Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNodeResponse`]`>` constructor, or [`ReadNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_node(
        name: &str,
        optional: ReadNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNodeResponse>), crate::RequestError> {
        let ReadNodeOptional {
            exact,
            export,
            pretty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Node::read_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNodeOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::read_node`]
#[derive(Debug)]
pub enum ReadNodeResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNodeResponse::Unauthorized, 0)),
            _ => Ok((ReadNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NodeStatus

impl Node {
    /// read status of the specified Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNodeStatusResponse`]`>` constructor, or [`ReadNodeStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_node_status(
        name: &str,
        optional: ReadNodeStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNodeStatusResponse>), crate::RequestError> {
        let ReadNodeStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/status?", name = name);
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

/// Optional parameters of [`Node::read_node_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNodeStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNodeStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::read_node_status`]
#[derive(Debug)]
pub enum ReadNodeStatusResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNodeStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNodeStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNodeStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNodeStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1Node

impl Node {
    /// replace the specified Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNodeResponse`]`>` constructor, or [`ReplaceNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_node(
        name: &str,
        body: &crate::v1_14::api::core::v1::Node,
        optional: ReplaceNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNodeResponse>), crate::RequestError> {
        let ReplaceNodeOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
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

/// Optional parameters of [`Node::replace_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::replace_node`]
#[derive(Debug)]
pub enum ReplaceNodeResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Created(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNodeResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNodeResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNodeResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NodeStatus

impl Node {
    /// replace status of the specified Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNodeStatusResponse`]`>` constructor, or [`ReplaceNodeStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_node_status(
        name: &str,
        body: &crate::v1_14::api::core::v1::Node,
        optional: ReplaceNodeStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNodeStatusResponse>), crate::RequestError> {
        let ReplaceNodeStatusOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
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

/// Optional parameters of [`Node::replace_node_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNodeStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNodeStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::replace_node_status`]
#[derive(Debug)]
pub enum ReplaceNodeStatusResponse {
    Ok(crate::v1_14::api::core::v1::Node),
    Created(crate::v1_14::api::core::v1::Node),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNodeStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNodeStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNodeStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNodeStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNodeStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1Node

impl Node {
    /// list or watch objects of kind Node
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNodeResponse`]`>` constructor, or [`WatchNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_node(
        optional: crate::v1_14::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNodeResponse>), crate::RequestError> {
        let crate::v1_14::WatchOptional {
            field_selector,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/nodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
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

/// Use `<WatchNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::watch_node`]
#[derive(Debug)]
pub enum WatchNodeResponse {
    Ok(crate::v1_14::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNodeResponse {
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
                Ok((WatchNodeResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNodeResponse::Unauthorized, 0)),
            _ => Ok((WatchNodeResponse::Other, 0)),
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

impl crate::Metadata for Node {
    type Ty = crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
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
                let mut value_metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_14::api::core::v1::NodeSpec> = None;
                let mut value_status: Option<crate::v1_14::api::core::v1::NodeStatus> = None;

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