// Generated from definition io.k8s.kubernetes.pkg.apis.policy.v1beta1.PodDisruptionBudget

/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDisruptionBudget {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the PodDisruptionBudget.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetSpec>,

    /// Most recently observed status of the PodDisruptionBudget.
    pub status: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetStatus>,
}

// Begin policy/v1beta1/PodDisruptionBudget

// Generated from operation createPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// create a PodDisruptionBudget
    ///
    /// Use [`CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
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
    pub fn create_policy_v1beta1_namespaced_pod_disruption_budget(
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::create_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.create_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation deletePolicyV1beta1CollectionNamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// delete collection of PodDisruptionBudget
    ///
    /// Use [`DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse`](./enum.DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
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
    pub fn delete_policy_v1beta1_collection_namespaced_pod_disruption_budget(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::delete_policy_v1beta1_collection_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.delete_policy_v1beta1_collection_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse {
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
                    Ok((DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation deletePolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// delete a PodDisruptionBudget
    ///
    /// Use [`DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn delete_policy_v1beta1_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::delete_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.delete_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse {
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
                    Ok((DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation listPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// Use [`ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
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
    pub fn list_policy_v1beta1_namespaced_pod_disruption_budget(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::list_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.list_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetList),
    Unauthorized,
    Other,
}

impl ::Response for ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation listPolicyV1beta1PodDisruptionBudgetForAllNamespaces

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// Use [`ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse`](./enum.ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse.html) to parse the HTTP response.
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
    pub fn list_policy_v1beta1_pod_disruption_budget_for_all_namespaces(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/poddisruptionbudgets?");
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

/// Parses the HTTP response of [`PodDisruptionBudget::list_policy_v1beta1_pod_disruption_budget_for_all_namespaces`](./struct.PodDisruptionBudget.html#method.list_policy_v1beta1_pod_disruption_budget_for_all_namespaces)
#[derive(Debug)]
pub enum ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetList),
    Unauthorized,
    Other,
}

impl ::Response for ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// partially update the specified PodDisruptionBudget
    ///
    /// Use [`PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn patch_policy_v1beta1_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::patch_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.patch_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// partially update status of the specified PodDisruptionBudget
    ///
    /// Use [`PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse`](./enum.PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn patch_policy_v1beta1_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::patch_policy_v1beta1_namespaced_pod_disruption_budget_status`](./struct.PodDisruptionBudget.html#method.patch_policy_v1beta1_namespaced_pod_disruption_budget_status)
#[derive(Debug)]
pub enum PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// read the specified PodDisruptionBudget
    ///
    /// Use [`ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn read_policy_v1beta1_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::read_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.read_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// read status of the specified PodDisruptionBudget
    ///
    /// Use [`ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse`](./enum.ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_policy_v1beta1_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::read_policy_v1beta1_namespaced_pod_disruption_budget_status`](./struct.PodDisruptionBudget.html#method.read_policy_v1beta1_namespaced_pod_disruption_budget_status)
#[derive(Debug)]
pub enum ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// replace the specified PodDisruptionBudget
    ///
    /// Use [`ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn replace_policy_v1beta1_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::replace_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.replace_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// replace status of the specified PodDisruptionBudget
    ///
    /// Use [`ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse`](./enum.ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn replace_policy_v1beta1_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::replace_policy_v1beta1_namespaced_pod_disruption_budget_status`](./struct.PodDisruptionBudget.html#method.replace_policy_v1beta1_namespaced_pod_disruption_budget_status)
#[derive(Debug)]
pub enum ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl ::Response for ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// watch changes to an object of kind PodDisruptionBudget
    ///
    /// Use [`WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse`](./enum.WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodDisruptionBudget
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
    pub fn watch_policy_v1beta1_namespaced_pod_disruption_budget(
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
        let __url = format!("/apis/policy/v1beta1/watch/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::watch_policy_v1beta1_namespaced_pod_disruption_budget`](./struct.PodDisruptionBudget.html#method.watch_policy_v1beta1_namespaced_pod_disruption_budget)
#[derive(Debug)]
pub enum WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse {
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
                Ok((WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudgetList

impl PodDisruptionBudget {
    /// watch individual changes to a list of PodDisruptionBudget
    ///
    /// Use [`WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse`](./enum.WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse.html) to parse the HTTP response.
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
    pub fn watch_policy_v1beta1_namespaced_pod_disruption_budget_list(
        namespace: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/watch/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Parses the HTTP response of [`PodDisruptionBudget::watch_policy_v1beta1_namespaced_pod_disruption_budget_list`](./struct.PodDisruptionBudget.html#method.watch_policy_v1beta1_namespaced_pod_disruption_budget_list)
#[derive(Debug)]
pub enum WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse {
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
                Ok((WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse::Unauthorized, 0)),
            _ => Ok((WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchPolicyV1beta1PodDisruptionBudgetListForAllNamespaces

impl PodDisruptionBudget {
    /// watch individual changes to a list of PodDisruptionBudget
    ///
    /// Use [`WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse`](./enum.WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse.html) to parse the HTTP response.
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
    pub fn watch_policy_v1beta1_pod_disruption_budget_list_for_all_namespaces(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/policy/v1beta1/watch/poddisruptionbudgets?");
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

/// Parses the HTTP response of [`PodDisruptionBudget::watch_policy_v1beta1_pod_disruption_budget_list_for_all_namespaces`](./struct.PodDisruptionBudget.html#method.watch_policy_v1beta1_pod_disruption_budget_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse {
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
                Ok((WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End policy/v1beta1/PodDisruptionBudget

impl<'de> ::serde::Deserialize<'de> for PodDisruptionBudget {
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
            type Value = PodDisruptionBudget;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodDisruptionBudget")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetStatus> = None;

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

                Ok(PodDisruptionBudget {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDisruptionBudget",
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

impl ::serde::Serialize for PodDisruptionBudget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDisruptionBudget",
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
