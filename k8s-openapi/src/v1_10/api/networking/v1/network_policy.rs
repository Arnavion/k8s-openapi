// Generated from definition io.k8s.api.networking.v1.NetworkPolicy

/// NetworkPolicy describes what network traffic is allowed for a set of Pods
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicy {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior for this NetworkPolicy.
    pub spec: Option<::v1_10::api::networking::v1::NetworkPolicySpec>,
}

// Begin networking.k8s.io/v1/NetworkPolicy

// Generated from operation createNetworkingV1NamespacedNetworkPolicy

#[derive(Debug)]
pub enum CreateNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::networking::v1::NetworkPolicy),
    Created(::v1_10::api::networking::v1::NetworkPolicy),
    Accepted(::v1_10::api::networking::v1::NetworkPolicy),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// create a NetworkPolicy
    pub fn create_networking_v1_namespaced_network_policy<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_10::api::networking::v1::NetworkPolicy,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies", namespace = namespace)).map_err(::Error::URL)?;
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
                CreateNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateNetworkingV1NamespacedNetworkPolicyResponse::Created(result)
            },
            ::http::StatusCode::ACCEPTED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateNetworkingV1NamespacedNetworkPolicyResponse::Accepted(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => CreateNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteNetworkingV1CollectionNamespacedNetworkPolicy

#[derive(Debug)]
pub enum DeleteNetworkingV1CollectionNamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// delete collection of NetworkPolicy
    pub fn delete_networking_v1_collection_namespaced_network_policy<C>(
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<DeleteNetworkingV1CollectionNamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies", namespace = namespace)).map_err(::Error::URL)?;
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
                DeleteNetworkingV1CollectionNamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteNetworkingV1CollectionNamespacedNetworkPolicyResponse::Unauthorized(response),
            other => DeleteNetworkingV1CollectionNamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteNetworkingV1NamespacedNetworkPolicy

#[derive(Debug)]
pub enum DeleteNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// delete a NetworkPolicy
    pub fn delete_networking_v1_namespaced_network_policy<C>(
        __client: &C,
        // name of the NetworkPolicy
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
    ) -> Result<DeleteNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                DeleteNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => DeleteNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation listNetworkingV1NamespacedNetworkPolicy

#[derive(Debug)]
pub enum ListNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::networking::v1::NetworkPolicyList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// list or watch objects of kind NetworkPolicy
    pub fn list_networking_v1_namespaced_network_policy<C>(
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies", namespace = namespace)).map_err(::Error::URL)?;
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
                ListNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => ListNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation listNetworkingV1NetworkPolicyForAllNamespaces

#[derive(Debug)]
pub enum ListNetworkingV1NetworkPolicyForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::networking::v1::NetworkPolicyList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// list or watch objects of kind NetworkPolicy
    pub fn list_networking_v1_network_policy_for_all_namespaces<C>(
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
    ) -> Result<ListNetworkingV1NetworkPolicyForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/networkpolicies")).map_err(::Error::URL)?;
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
                ListNetworkingV1NetworkPolicyForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListNetworkingV1NetworkPolicyForAllNamespacesResponse::Unauthorized(response),
            other => ListNetworkingV1NetworkPolicyForAllNamespacesResponse::Other(other, response),
        })
    }
}

// Generated from operation patchNetworkingV1NamespacedNetworkPolicy

#[derive(Debug)]
pub enum PatchNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::networking::v1::NetworkPolicy),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// partially update the specified NetworkPolicy
    pub fn patch_networking_v1_namespaced_network_policy<C>(
        __client: &C,
        // name of the NetworkPolicy
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => PatchNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation readNetworkingV1NamespacedNetworkPolicy

#[derive(Debug)]
pub enum ReadNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::networking::v1::NetworkPolicy),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// read the specified NetworkPolicy
    pub fn read_networking_v1_namespaced_network_policy<C>(
        __client: &C,
        // name of the NetworkPolicy
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => ReadNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceNetworkingV1NamespacedNetworkPolicy

#[derive(Debug)]
pub enum ReplaceNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::v1_10::api::networking::v1::NetworkPolicy),
    Created(::v1_10::api::networking::v1::NetworkPolicy),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// replace the specified NetworkPolicy
    pub fn replace_networking_v1_namespaced_network_policy<C>(
        __client: &C,
        // name of the NetworkPolicy
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_10::api::networking::v1::NetworkPolicy,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceNetworkingV1NamespacedNetworkPolicyResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => ReplaceNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation watchNetworkingV1NamespacedNetworkPolicy

pub enum WatchNetworkingV1NamespacedNetworkPolicyResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// watch changes to an object of kind NetworkPolicy
    pub fn watch_networking_v1_namespaced_network_policy<C>(
        __client: &C,
        // name of the NetworkPolicy
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchNetworkingV1NamespacedNetworkPolicyResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/watch/namespaces/{namespace}/networkpolicies/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                WatchNetworkingV1NamespacedNetworkPolicyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchNetworkingV1NamespacedNetworkPolicyResponse::Unauthorized(response),
            other => WatchNetworkingV1NamespacedNetworkPolicyResponse::Other(other, response),
        })
    }
}

// Generated from operation watchNetworkingV1NamespacedNetworkPolicyList

pub enum WatchNetworkingV1NamespacedNetworkPolicyListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// watch individual changes to a list of NetworkPolicy
    pub fn watch_networking_v1_namespaced_network_policy_list<C>(
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
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchNetworkingV1NamespacedNetworkPolicyListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/watch/namespaces/{namespace}/networkpolicies", namespace = namespace)).map_err(::Error::URL)?;
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
                WatchNetworkingV1NamespacedNetworkPolicyListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchNetworkingV1NamespacedNetworkPolicyListResponse::Unauthorized(response),
            other => WatchNetworkingV1NamespacedNetworkPolicyListResponse::Other(other, response),
        })
    }
}

// Generated from operation watchNetworkingV1NetworkPolicyListForAllNamespaces

pub enum WatchNetworkingV1NetworkPolicyListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl NetworkPolicy {
    /// watch individual changes to a list of NetworkPolicy
    pub fn watch_networking_v1_network_policy_list_for_all_namespaces<C>(
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
    ) -> Result<WatchNetworkingV1NetworkPolicyListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/watch/networkpolicies")).map_err(::Error::URL)?;
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
                WatchNetworkingV1NetworkPolicyListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchNetworkingV1NetworkPolicyListForAllNamespacesResponse::Unauthorized(response),
            other => WatchNetworkingV1NetworkPolicyListForAllNamespacesResponse::Other(other, response),
        })
    }
}

// End networking.k8s.io/v1/NetworkPolicy

impl<'de> ::serde::Deserialize<'de> for NetworkPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicy;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NetworkPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_10::api::networking::v1::NetworkPolicySpec> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicy {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicy",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NetworkPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicy",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1),
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
        ::serde::ser::SerializeStruct::end(state)
    }
}
