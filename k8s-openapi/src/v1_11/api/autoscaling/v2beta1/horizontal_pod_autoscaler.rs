// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscaler

/// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource implementing the scale subresource based on the metrics specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscaler {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// metadata is the standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// spec is the specification for the behaviour of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec>,

    /// status is the current information about the autoscaler.
    pub status: Option<::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus>,
}

// Begin autoscaling/v2beta1/HorizontalPodAutoscaler

// Generated from operation createAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// create a HorizontalPodAutoscaler
    ///
    /// Use [`CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn create_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::create_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.create_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Created(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Accepted(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::ACCEPTED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Accepted(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((CreateAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete collection of HorizontalPodAutoscaler
    ///
    /// Use [`DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse`](./enum.DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn delete_autoscaling_v2beta1_collection_namespaced_horizontal_pod_autoscaler(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::delete_autoscaling_v2beta1_collection_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_autoscaling_v2beta1_collection_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse {
    OkStatus(::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse {
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
                    Ok((DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((DeleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete a HorizontalPodAutoscaler
    ///
    /// Use [`DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn delete_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
        // name of the HorizontalPodAutoscaler
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::delete_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    OkStatus(::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
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
                    Ok((DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((DeleteAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation listAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespaces

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// Use [`ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse`](./enum.ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse.html) to parse the HTTP response.
    pub fn list_autoscaling_v2beta1_horizontal_pod_autoscaler_for_all_namespaces(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/horizontalpodautoscalers?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::list_autoscaling_v2beta1_horizontal_pod_autoscaler_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.list_autoscaling_v2beta1_horizontal_pod_autoscaler_for_all_namespaces)
#[derive(Debug)]
pub enum ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscalerList),
    Unauthorized,
    Other,
}

impl ::Response for ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation listAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// Use [`ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn list_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::list_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.list_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscalerList),
    Unauthorized,
    Other,
}

impl ::Response for ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ListAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// partially update the specified HorizontalPodAutoscaler
    ///
    /// Use [`PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn patch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::patch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.patch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// partially update status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse`](./enum.PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    pub fn patch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status(
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::patch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.patch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// read the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn read_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::read_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.read_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation readAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// read status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse`](./enum.ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    pub fn read_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status(
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::read_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.read_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// replace the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn replace_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::replace_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.replace_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Created(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// replace status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse`](./enum.ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    pub fn replace_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status(
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::replace_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.replace_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Created(::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespaces

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    ///
    /// Use [`WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse`](./enum.WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse.html) to parse the HTTP response.
    pub fn watch_autoscaling_v2beta1_horizontal_pod_autoscaler_list_for_all_namespaces(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/watch/horizontalpodautoscalers?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_autoscaling_v2beta1_horizontal_pod_autoscaler_list_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.watch_autoscaling_v2beta1_horizontal_pod_autoscaler_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse {
    Ok(::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse {
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
                Ok((WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchAutoscalingV2beta1HorizontalPodAutoscalerListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// watch changes to an object of kind HorizontalPodAutoscaler
    ///
    /// Use [`WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse`](./enum.WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    pub fn watch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler(
        // name of the HorizontalPodAutoscaler
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/watch/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.watch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse {
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
                Ok((WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerList

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    ///
    /// Use [`WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse`](./enum.WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse.html) to parse the HTTP response.
    pub fn watch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_list(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/watch/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_list`](./struct.HorizontalPodAutoscaler.html#method.watch_autoscaling_v2beta1_namespaced_horizontal_pod_autoscaler_list)
#[derive(Debug)]
pub enum WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse {
    Ok(::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse {
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
                Ok((WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse::Unauthorized, 0)),
            _ => Ok((WatchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerListResponse::Other, 0)),
        }
    }
}

// End autoscaling/v2beta1/HorizontalPodAutoscaler

impl<'de> ::serde::Deserialize<'de> for HorizontalPodAutoscaler {
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
            type Value = HorizontalPodAutoscaler;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct HorizontalPodAutoscaler")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec> = None;
                let mut value_status: Option<::v1_11::api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus> = None;

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

                Ok(HorizontalPodAutoscaler {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscaler",
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

impl ::serde::Serialize for HorizontalPodAutoscaler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscaler",
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
