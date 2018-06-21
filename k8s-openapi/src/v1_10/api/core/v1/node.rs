// Generated from definition io.k8s.api.core.v1.Node

/// Node is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in etcd).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Node {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of a node. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_10::api::core::v1::NodeSpec>,

    /// Most recently observed status of the node. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<::v1_10::api::core::v1::NodeStatus>,
}

// Begin /v1/Node

// Generated from operation connectCoreV1DeleteNodeProxy

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNodeProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect DELETE requests to proxy of Node
    pub fn connect_core_v1_delete_node_proxy<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // Path is the URL path to use for the current proxy request to node.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1DeleteNodeProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy", name = name)).map_err(::Error::URL)?;
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
                ConnectCoreV1DeleteNodeProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNodeProxyResponse::Unauthorized(response),
            other => ConnectCoreV1DeleteNodeProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1DeleteNodeProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNodeProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect DELETE requests to proxy of Node
    pub fn connect_core_v1_delete_node_proxy_with_path<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to node.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1DeleteNodeProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy/{path}", name = name, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1DeleteNodeProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNodeProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1DeleteNodeProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1GetNodeProxy

#[derive(Debug)]
pub enum ConnectCoreV1GetNodeProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect GET requests to proxy of Node
    pub fn connect_core_v1_get_node_proxy<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // Path is the URL path to use for the current proxy request to node.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1GetNodeProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy", name = name)).map_err(::Error::URL)?;
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
                ConnectCoreV1GetNodeProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNodeProxyResponse::Unauthorized(response),
            other => ConnectCoreV1GetNodeProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1GetNodeProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1GetNodeProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect GET requests to proxy of Node
    pub fn connect_core_v1_get_node_proxy_with_path<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to node.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1GetNodeProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy/{path}", name = name, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1GetNodeProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNodeProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1GetNodeProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PatchNodeProxy

#[derive(Debug)]
pub enum ConnectCoreV1PatchNodeProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect PATCH requests to proxy of Node
    pub fn connect_core_v1_patch_node_proxy<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // Path is the URL path to use for the current proxy request to node.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PatchNodeProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy", name = name)).map_err(::Error::URL)?;
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
                ConnectCoreV1PatchNodeProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNodeProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PatchNodeProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PatchNodeProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PatchNodeProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect PATCH requests to proxy of Node
    pub fn connect_core_v1_patch_node_proxy_with_path<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to node.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PatchNodeProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy/{path}", name = name, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1PatchNodeProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNodeProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PatchNodeProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PostNodeProxy

#[derive(Debug)]
pub enum ConnectCoreV1PostNodeProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect POST requests to proxy of Node
    pub fn connect_core_v1_post_node_proxy<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // Path is the URL path to use for the current proxy request to node.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PostNodeProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy", name = name)).map_err(::Error::URL)?;
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
                ConnectCoreV1PostNodeProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNodeProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PostNodeProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PostNodeProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PostNodeProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect POST requests to proxy of Node
    pub fn connect_core_v1_post_node_proxy_with_path<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to node.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PostNodeProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy/{path}", name = name, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1PostNodeProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNodeProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PostNodeProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PutNodeProxy

#[derive(Debug)]
pub enum ConnectCoreV1PutNodeProxyResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect PUT requests to proxy of Node
    pub fn connect_core_v1_put_node_proxy<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // Path is the URL path to use for the current proxy request to node.
        path: Option<&str>,
    ) -> Result<ConnectCoreV1PutNodeProxyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy", name = name)).map_err(::Error::URL)?;
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
                ConnectCoreV1PutNodeProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNodeProxyResponse::Unauthorized(response),
            other => ConnectCoreV1PutNodeProxyResponse::Other(other, response),
        })
    }
}

// Generated from operation connectCoreV1PutNodeProxyWithPath

#[derive(Debug)]
pub enum ConnectCoreV1PutNodeProxyWithPathResponse<R> where R: ::std::io::Read {
    Ok(String),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// connect PUT requests to proxy of Node
    pub fn connect_core_v1_put_node_proxy_with_path<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to node.
        path_: Option<&str>,
    ) -> Result<ConnectCoreV1PutNodeProxyWithPathResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/proxy/{path}", name = name, path = path)).map_err(::Error::URL)?;
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
                ConnectCoreV1PutNodeProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNodeProxyWithPathResponse::Unauthorized(response),
            other => ConnectCoreV1PutNodeProxyWithPathResponse::Other(other, response),
        })
    }
}

// Generated from operation createCoreV1Node

#[derive(Debug)]
pub enum CreateCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Created(::v1_10::api::core::v1::Node),
    Accepted(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// create a Node
    pub fn create_core_v1_node<C>(
        __client: &C,
        body: &::v1_10::api::core::v1::Node,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes")).map_err(::Error::URL)?;
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
                CreateCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCoreV1NodeResponse::Created(result)
            },
            ::http::StatusCode::ACCEPTED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCoreV1NodeResponse::Accepted(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCoreV1NodeResponse::Unauthorized(response),
            other => CreateCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1CollectionNode

#[derive(Debug)]
pub enum DeleteCoreV1CollectionNodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// delete collection of Node
    pub fn delete_core_v1_collection_node<C>(
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<DeleteCoreV1CollectionNodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes")).map_err(::Error::URL)?;
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
                DeleteCoreV1CollectionNodeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1CollectionNodeResponse::Unauthorized(response),
            other => DeleteCoreV1CollectionNodeResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1Node

#[derive(Debug)]
pub enum DeleteCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// delete a Node
    pub fn delete_core_v1_node<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}", name = name)).map_err(::Error::URL)?;
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
                DeleteCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1NodeResponse::Unauthorized(response),
            other => DeleteCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation listCoreV1Node

#[derive(Debug)]
pub enum ListCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::NodeList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// list or watch objects of kind Node
    pub fn list_core_v1_node<C>(
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes")).map_err(::Error::URL)?;
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
                ListCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1NodeResponse::Unauthorized(response),
            other => ListCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1Node

#[derive(Debug)]
pub enum PatchCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// partially update the specified Node
    pub fn patch_core_v1_node<C>(
        __client: &C,
        // name of the Node
        name: &str,
        body: &::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}", name = name)).map_err(::Error::URL)?;
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
                PatchCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NodeResponse::Unauthorized(response),
            other => PatchCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1NodeStatus

#[derive(Debug)]
pub enum PatchCoreV1NodeStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// partially update status of the specified Node
    pub fn patch_core_v1_node_status<C>(
        __client: &C,
        // name of the Node
        name: &str,
        body: &::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NodeStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/status", name = name)).map_err(::Error::URL)?;
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
                PatchCoreV1NodeStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NodeStatusResponse::Unauthorized(response),
            other => PatchCoreV1NodeStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1Node

#[derive(Debug)]
pub enum ReadCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// read the specified Node
    pub fn read_core_v1_node<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}", name = name)).map_err(::Error::URL)?;
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
                ReadCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NodeResponse::Unauthorized(response),
            other => ReadCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1NodeStatus

#[derive(Debug)]
pub enum ReadCoreV1NodeStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// read status of the specified Node
    pub fn read_core_v1_node_status<C>(
        __client: &C,
        // name of the Node
        name: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NodeStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/status", name = name)).map_err(::Error::URL)?;
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
                ReadCoreV1NodeStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NodeStatusResponse::Unauthorized(response),
            other => ReadCoreV1NodeStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1Node

#[derive(Debug)]
pub enum ReplaceCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Created(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// replace the specified Node
    pub fn replace_core_v1_node<C>(
        __client: &C,
        // name of the Node
        name: &str,
        body: &::v1_10::api::core::v1::Node,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}", name = name)).map_err(::Error::URL)?;
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
                ReplaceCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NodeResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NodeResponse::Unauthorized(response),
            other => ReplaceCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1NodeStatus

#[derive(Debug)]
pub enum ReplaceCoreV1NodeStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::core::v1::Node),
    Created(::v1_10::api::core::v1::Node),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// replace status of the specified Node
    pub fn replace_core_v1_node_status<C>(
        __client: &C,
        // name of the Node
        name: &str,
        body: &::v1_10::api::core::v1::Node,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NodeStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/nodes/{name}/status", name = name)).map_err(::Error::URL)?;
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
                ReplaceCoreV1NodeStatusResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NodeStatusResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NodeStatusResponse::Unauthorized(response),
            other => ReplaceCoreV1NodeStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1Node

pub enum WatchCoreV1NodeResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// watch changes to an object of kind Node
    pub fn watch_core_v1_node<C>(
        __client: &C,
        // name of the Node
        name: &str,
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NodeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/nodes/{name}", name = name)).map_err(::Error::URL)?;
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
                WatchCoreV1NodeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NodeResponse::Unauthorized(response),
            other => WatchCoreV1NodeResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1NodeList

pub enum WatchCoreV1NodeListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Node {
    /// watch individual changes to a list of Node
    pub fn watch_core_v1_node_list<C>(
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NodeListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/nodes")).map_err(::Error::URL)?;
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
                WatchCoreV1NodeListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NodeListResponse::Unauthorized(response),
            other => WatchCoreV1NodeListResponse::Other(other, response),
        })
    }
}

// End /v1/Node

impl<'de> ::serde::Deserialize<'de> for Node {
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
            type Value = Node;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Node")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_10::api::core::v1::NodeSpec> = None;
                let mut value_status: Option<::v1_10::api::core::v1::NodeStatus> = None;

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

                Ok(Node {
                    api_version: value_api_version,
                    kind: value_kind,
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

impl ::serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Node",
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
