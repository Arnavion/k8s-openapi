// Generated from definition io.k8s.api.core.v1.Node

/// Node is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in etcd).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Node {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a node. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::api::core::v1::NodeSpec>,

    /// Most recently observed status of the node. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::api::core::v1::NodeStatus>,
}

// Begin /v1/Node

// Generated from operation connectCoreV1DeleteNodeProxy

impl Node {
    /// connect DELETE requests to proxy of Node
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
    #[cfg(feature = "api")]
    pub fn connect_delete_node_proxy(
        name: &str,
        optional: ConnectDeleteNodeProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectDeleteNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_delete_node_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1DeleteNodeProxyWithPath

impl Node {
    /// connect DELETE requests to proxy of Node
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_delete_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectDeleteNodeProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectDeleteNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_delete_node_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeleteNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1GetNodeProxy

impl Node {
    /// connect GET requests to proxy of Node
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
    #[cfg(feature = "api")]
    pub fn connect_get_node_proxy(
        name: &str,
        optional: ConnectGetNodeProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_get_node_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1GetNodeProxyWithPath

impl Node {
    /// connect GET requests to proxy of Node
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_get_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectGetNodeProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_get_node_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PatchNodeProxy

impl Node {
    /// connect PATCH requests to proxy of Node
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
    #[cfg(feature = "api")]
    pub fn connect_patch_node_proxy(
        name: &str,
        optional: ConnectPatchNodeProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPatchNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_patch_node_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PatchNodeProxyWithPath

impl Node {
    /// connect PATCH requests to proxy of Node
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_patch_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectPatchNodeProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPatchNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_patch_node_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PostNodeProxy

impl Node {
    /// connect POST requests to proxy of Node
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
    #[cfg(feature = "api")]
    pub fn connect_post_node_proxy(
        name: &str,
        optional: ConnectPostNodeProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_post_node_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PostNodeProxyWithPath

impl Node {
    /// connect POST requests to proxy of Node
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_post_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectPostNodeProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_post_node_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PutNodeProxy

impl Node {
    /// connect PUT requests to proxy of Node
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
    #[cfg(feature = "api")]
    pub fn connect_put_node_proxy(
        name: &str,
        optional: ConnectPutNodeProxyOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPutNodeProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_put_node_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNodeProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PutNodeProxyWithPath

impl Node {
    /// connect PUT requests to proxy of Node
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_put_node_proxy_with_path(
        name: &str,
        path: &str,
        optional: ConnectPutNodeProxyWithPathOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPutNodeProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Node::connect_put_node_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutNodeProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to node.
    pub path_: Option<&'a str>,
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
    #[cfg(feature = "api")]
    pub fn create_node(
        body: &crate::api::core::v1::Node,
        optional: CreateNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNodeResponse>), crate::RequestError> {
        let CreateNodeOptional {
            pretty,
        } = optional;
        let __url = "/api/v1/nodes?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Node::create_node`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNodeOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::create_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNodeResponse {
    Ok(crate::api::core::v1::Node),
    Created(crate::api::core::v1::Node),
    Accepted(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((CreateNodeResponse::Other(result), read))
            },
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
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_node(
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNodeResponse>), crate::RequestError> {
        let __url = "/api/v1/nodes?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&delete_optional).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<DeleteCollectionNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::delete_collection_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionNodeResponse {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::api::core::v1::NodeList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((DeleteCollectionNodeResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn delete_node(
        name: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<DeleteNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::delete_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteNodeResponse {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((DeleteNodeResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn list_node(
        optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNodeResponse>), crate::RequestError> {
        let __url = "/api/v1/nodes?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ListNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::list_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListNodeResponse {
    Ok(crate::api::core::v1::NodeList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ListNodeResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn patch_node(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::patch_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNodeResponse {
    Ok(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((PatchNodeResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn patch_node_status(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNodeStatusResponse>), crate::RequestError> {
        let __url = format!("/api/v1/nodes/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNodeStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::patch_node_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNodeStatusResponse {
    Ok(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((PatchNodeStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1DELETENode

impl Node {
    /// proxy DELETE requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyDELETENodeResponse`]`>` constructor, or [`ProxyDELETENodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    #[cfg(feature = "api")]
    pub fn proxy_delete_node(
        name: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyDELETENodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyDELETENodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_delete_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyDELETENodeResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyDELETENodeResponse {
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
                Ok((ProxyDELETENodeResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyDELETENodeResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1DELETENodeWithPath

impl Node {
    /// proxy DELETE requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyDELETENodeWithPathResponse`]`>` constructor, or [`ProxyDELETENodeWithPathResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn proxy_delete_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyDELETENodeWithPathResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyDELETENodeWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_delete_node_with_path`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyDELETENodeWithPathResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyDELETENodeWithPathResponse {
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
                Ok((ProxyDELETENodeWithPathResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyDELETENodeWithPathResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1GETNode

impl Node {
    /// proxy GET requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyGETNodeResponse`]`>` constructor, or [`ProxyGETNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    #[cfg(feature = "api")]
    pub fn proxy_get_node(
        name: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyGETNodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyGETNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_get_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyGETNodeResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyGETNodeResponse {
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
                Ok((ProxyGETNodeResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyGETNodeResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1GETNodeWithPath

impl Node {
    /// proxy GET requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyGETNodeWithPathResponse`]`>` constructor, or [`ProxyGETNodeWithPathResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn proxy_get_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyGETNodeWithPathResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyGETNodeWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_get_node_with_path`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyGETNodeWithPathResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyGETNodeWithPathResponse {
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
                Ok((ProxyGETNodeWithPathResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyGETNodeWithPathResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1PATCHNode

impl Node {
    /// proxy PATCH requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyPATCHNodeResponse`]`>` constructor, or [`ProxyPATCHNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    #[cfg(feature = "api")]
    pub fn proxy_patch_node(
        name: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyPATCHNodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyPATCHNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_patch_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyPATCHNodeResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyPATCHNodeResponse {
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
                Ok((ProxyPATCHNodeResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyPATCHNodeResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1PATCHNodeWithPath

impl Node {
    /// proxy PATCH requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyPATCHNodeWithPathResponse`]`>` constructor, or [`ProxyPATCHNodeWithPathResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn proxy_patch_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyPATCHNodeWithPathResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::patch(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyPATCHNodeWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_patch_node_with_path`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyPATCHNodeWithPathResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyPATCHNodeWithPathResponse {
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
                Ok((ProxyPATCHNodeWithPathResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyPATCHNodeWithPathResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1POSTNode

impl Node {
    /// proxy POST requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyPOSTNodeResponse`]`>` constructor, or [`ProxyPOSTNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    #[cfg(feature = "api")]
    pub fn proxy_post_node(
        name: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyPOSTNodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyPOSTNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_post_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyPOSTNodeResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyPOSTNodeResponse {
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
                Ok((ProxyPOSTNodeResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyPOSTNodeResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1POSTNodeWithPath

impl Node {
    /// proxy POST requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyPOSTNodeWithPathResponse`]`>` constructor, or [`ProxyPOSTNodeWithPathResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn proxy_post_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyPOSTNodeWithPathResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::post(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyPOSTNodeWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_post_node_with_path`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyPOSTNodeWithPathResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyPOSTNodeWithPathResponse {
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
                Ok((ProxyPOSTNodeWithPathResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyPOSTNodeWithPathResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1PUTNode

impl Node {
    /// proxy PUT requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyPUTNodeResponse`]`>` constructor, or [`ProxyPUTNodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Node
    #[cfg(feature = "api")]
    pub fn proxy_put_node(
        name: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyPUTNodeResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyPUTNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_put_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyPUTNodeResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyPUTNodeResponse {
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
                Ok((ProxyPUTNodeResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyPUTNodeResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation proxyCoreV1PUTNodeWithPath

impl Node {
    /// proxy PUT requests to Node
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ProxyPUTNodeWithPathResponse`]`>` constructor, or [`ProxyPUTNodeWithPathResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn proxy_put_node_with_path(
        name: &str,
        path: &str,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ProxyPUTNodeWithPathResponse>), crate::RequestError> {
        let __url = format!("/api/v1/proxy/nodes/{name}/{path}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::put(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ProxyPUTNodeWithPathResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::proxy_put_node_with_path`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ProxyPUTNodeWithPathResponse {
    Ok(String),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ProxyPUTNodeWithPathResponse {
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
                Ok((ProxyPUTNodeWithPathResponse::Ok(result.to_owned()), len))
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
                Ok((ProxyPUTNodeWithPathResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn read_node(
        name: &str,
        optional: ReadNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNodeResponse>), crate::RequestError> {
        let ReadNodeOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Node::read_node`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNodeOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::read_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNodeResponse {
    Ok(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ReadNodeResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn read_node_status(
        name: &str,
        optional: ReadNodeStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNodeStatusResponse>), crate::RequestError> {
        let ReadNodeStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Node::read_node_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNodeStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNodeStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::read_node_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNodeStatusResponse {
    Ok(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ReadNodeStatusResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn replace_node(
        name: &str,
        body: &crate::api::core::v1::Node,
        optional: ReplaceNodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNodeResponse>), crate::RequestError> {
        let ReplaceNodeOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Node::replace_node`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNodeOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::replace_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNodeResponse {
    Ok(crate::api::core::v1::Node),
    Created(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ReplaceNodeResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn replace_node_status(
        name: &str,
        body: &crate::api::core::v1::Node,
        optional: ReplaceNodeStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNodeStatusResponse>), crate::RequestError> {
        let ReplaceNodeStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/nodes/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Node::replace_node_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNodeStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNodeStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::replace_node_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNodeStatusResponse {
    Ok(crate::api::core::v1::Node),
    Created(crate::api::core::v1::Node),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ReplaceNodeStatusResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn watch_node(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNodeResponse>), crate::RequestError> {
        let __url = "/api/v1/nodes?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<WatchNodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Node::watch_node`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchNodeResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::WatchEvent<Node>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((WatchNodeResponse::Other(result), read))
            },
        }
    }
}

// End /v1/Node

impl crate::Resource for Node {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "Node";
    const VERSION: &'static str = "v1";
}

impl crate::ListableResource for Node {
    const LIST_KIND: &'static str = "NodeList";
}

impl crate::Metadata for Node {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::core::v1::NodeSpec> = None;
                let mut value_status: Option<crate::api::core::v1::NodeStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
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
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
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
