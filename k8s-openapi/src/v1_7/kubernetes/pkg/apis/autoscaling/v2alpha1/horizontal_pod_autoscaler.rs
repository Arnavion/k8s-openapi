// Generated from definition io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.HorizontalPodAutoscaler

/// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource implementing the scale subresource based on the metrics specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscaler {
    /// metadata is the standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// spec is the specification for the behaviour of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscalerSpec>,

    /// status is the current information about the autoscaler.
    pub status: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscalerStatus>,
}

// Begin autoscaling/v2alpha1/HorizontalPodAutoscaler

// Generated from operation createAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// create a HorizontalPodAutoscaler
    ///
    /// Use [`CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
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
    pub fn create_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::create_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.create_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((CreateAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete collection of HorizontalPodAutoscaler
    ///
    /// Use [`DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse`](./enum.DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn delete_autoscaling_v2alpha1_collection_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::delete_autoscaling_v2alpha1_collection_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_autoscaling_v2alpha1_collection_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse {
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
                    Ok((DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((DeleteAutoscalingV2alpha1CollectionNamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete a HorizontalPodAutoscaler
    ///
    /// Use [`DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    ///     Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub fn delete_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::delete_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.delete_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
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
                    Ok((DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((DeleteAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation listAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespaces

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// Use [`ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse`](./enum.ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
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
    pub fn list_autoscaling_v2alpha1_horizontal_pod_autoscaler_for_all_namespaces(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/horizontalpodautoscalers?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::list_autoscaling_v2alpha1_horizontal_pod_autoscaler_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.list_autoscaling_v2alpha1_horizontal_pod_autoscaler_for_all_namespaces)
#[derive(Debug)]
pub enum ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscalerList),
    Unauthorized,
    Other,
}

impl ::Response for ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListAutoscalingV2alpha1HorizontalPodAutoscalerForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation listAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// Use [`ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn list_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::list_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.list_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscalerList),
    Unauthorized,
    Other,
}

impl ::Response for ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ListAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// partially update the specified HorizontalPodAutoscaler
    ///
    /// Use [`PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn patch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::patch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.patch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// partially update status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse`](./enum.PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn patch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::patch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.patch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// read the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn read_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::read_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.read_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation readAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// read status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse`](./enum.ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::read_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.read_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// replace the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn replace_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::replace_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.replace_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// replace status of the specified HorizontalPodAutoscaler
    ///
    /// Use [`ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse`](./enum.ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn replace_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::replace_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status`](./struct.HorizontalPodAutoscaler.html#method.replace_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_status)
#[derive(Debug)]
pub enum ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscaler),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespaces

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    ///
    /// Use [`WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse`](./enum.WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
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
    pub fn watch_autoscaling_v2alpha1_horizontal_pod_autoscaler_list_for_all_namespaces(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/watch/horizontalpodautoscalers?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_autoscaling_v2alpha1_horizontal_pod_autoscaler_list_for_all_namespaces`](./struct.HorizontalPodAutoscaler.html#method.watch_autoscaling_v2alpha1_horizontal_pod_autoscaler_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse {
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
                Ok((WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchAutoscalingV2alpha1HorizontalPodAutoscalerListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV2alpha1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// watch changes to an object of kind HorizontalPodAutoscaler
    ///
    /// Use [`WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse`](./enum.WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn watch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/watch/namespaces/{namespace}/horizontalpodautoscalers/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler`](./struct.HorizontalPodAutoscaler.html#method.watch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler)
#[derive(Debug)]
pub enum WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse {
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
                Ok((WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Unauthorized, 0)),
            _ => Ok((WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerList

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    ///
    /// Use [`WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse`](./enum.WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn watch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_list(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/autoscaling/v2alpha1/watch/namespaces/{namespace}/horizontalpodautoscalers?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`HorizontalPodAutoscaler::watch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_list`](./struct.HorizontalPodAutoscaler.html#method.watch_autoscaling_v2alpha1_namespaced_horizontal_pod_autoscaler_list)
#[derive(Debug)]
pub enum WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse {
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
                Ok((WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse::Unauthorized, 0)),
            _ => Ok((WatchAutoscalingV2alpha1NamespacedHorizontalPodAutoscalerListResponse::Other, 0)),
        }
    }
}

// End autoscaling/v2alpha1/HorizontalPodAutoscaler

impl ::Resource for HorizontalPodAutoscaler {
    fn api_version() -> &'static str {
        "autoscaling/v2alpha1"
    }

    fn group() -> &'static str {
        "autoscaling"
    }

    fn kind() -> &'static str {
        "HorizontalPodAutoscaler"
    }

    fn version() -> &'static str {
        "v2alpha1"
    }
}

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
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscalerSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::HorizontalPodAutoscalerStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as ::Resource>::api_version() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as ::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as ::Resource>::kind() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_kind), &<Self::Value as ::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscaler {
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
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
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
