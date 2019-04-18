// Generated from definition io.k8s.api.policy.v1beta1.PodDisruptionBudget

/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDisruptionBudget {
    pub metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the PodDisruptionBudget.
    pub spec: Option<crate::v1_12::api::policy::v1beta1::PodDisruptionBudgetSpec>,

    /// Most recently observed status of the PodDisruptionBudget.
    pub status: Option<crate::v1_12::api::policy::v1beta1::PodDisruptionBudgetStatus>,
}

// Begin policy/v1beta1/PodDisruptionBudget

// Generated from operation createPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// create a PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`CreateNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_pod_disruption_budget(
        namespace: &str,
        body: &crate::v1_12::api::policy::v1beta1::PodDisruptionBudget,
        optional: CreateNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let CreateNamespacedPodDisruptionBudgetOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::create_namespaced_pod_disruption_budget`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedPodDisruptionBudgetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::create_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum CreateNamespacedPodDisruptionBudgetResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Created(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Accepted(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodDisruptionBudgetResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodDisruptionBudgetResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation deletePolicyV1beta1CollectionNamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// delete collection of PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`DeleteCollectionNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_namespaced_pod_disruption_budget(
        namespace: &str,
        optional: DeleteCollectionNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let DeleteCollectionNamespacedPodDisruptionBudgetOptional {
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
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::delete_collection_namespaced_pod_disruption_budget`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionNamespacedPodDisruptionBudgetOptional<'a> {
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

/// Use `<DeleteCollectionNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::delete_collection_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedPodDisruptionBudgetResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionNamespacedPodDisruptionBudgetResponse {
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
                    Ok((DeleteCollectionNamespacedPodDisruptionBudgetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedPodDisruptionBudgetResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation deletePolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// delete a PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`DeleteNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let DeleteNamespacedPodDisruptionBudgetOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::delete_namespaced_pod_disruption_budget`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespacedPodDisruptionBudgetOptional<'a> {
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

/// Use `<DeleteNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::delete_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum DeleteNamespacedPodDisruptionBudgetResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Accepted(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespacedPodDisruptionBudgetResponse {
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
                    Ok((DeleteNamespacedPodDisruptionBudgetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedPodDisruptionBudgetResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespacedPodDisruptionBudgetResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation listPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`ListNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_pod_disruption_budget(
        namespace: &str,
        optional: crate::v1_12::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let crate::v1_12::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Use `<ListNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::list_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum ListNamespacedPodDisruptionBudgetResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudgetList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((ListNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation listPolicyV1beta1PodDisruptionBudgetForAllNamespaces

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListPodDisruptionBudgetForAllNamespacesResponse`]`>` constructor, or [`ListPodDisruptionBudgetForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_pod_disruption_budget_for_all_namespaces(
        optional: crate::v1_12::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListPodDisruptionBudgetForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_12::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/policy/v1beta1/poddisruptionbudgets?".to_string();
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

/// Use `<ListPodDisruptionBudgetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::list_pod_disruption_budget_for_all_namespaces`]
#[derive(Debug)]
pub enum ListPodDisruptionBudgetForAllNamespacesResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudgetList),
    Unauthorized,
    Other,
}

impl crate::Response for ListPodDisruptionBudgetForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListPodDisruptionBudgetForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListPodDisruptionBudgetForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListPodDisruptionBudgetForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// partially update the specified PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`PatchNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        body: &crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let PatchNamespacedPodDisruptionBudgetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::patch_namespaced_pod_disruption_budget`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedPodDisruptionBudgetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::patch_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum PatchNamespacedPodDisruptionBudgetResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// partially update status of the specified PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedPodDisruptionBudgetStatusResponse`]`>` constructor, or [`PatchNamespacedPodDisruptionBudgetStatusResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPodDisruptionBudgetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedPodDisruptionBudgetStatusResponse>), crate::RequestError> {
        let PatchNamespacedPodDisruptionBudgetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::patch_namespaced_pod_disruption_budget_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedPodDisruptionBudgetStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespacedPodDisruptionBudgetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::patch_namespaced_pod_disruption_budget_status`]
#[derive(Debug)]
pub enum PatchNamespacedPodDisruptionBudgetStatusResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedPodDisruptionBudgetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPodDisruptionBudgetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedPodDisruptionBudgetStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedPodDisruptionBudgetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// read the specified PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`ReadNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let ReadNamespacedPodDisruptionBudgetOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedPodDisruptionBudgetOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum ReadNamespacedPodDisruptionBudgetResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// read status of the specified PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedPodDisruptionBudgetStatusResponse`]`>` constructor, or [`ReadNamespacedPodDisruptionBudgetStatusResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodDisruptionBudgetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedPodDisruptionBudgetStatusResponse>), crate::RequestError> {
        let ReadNamespacedPodDisruptionBudgetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedPodDisruptionBudgetStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedPodDisruptionBudgetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget_status`]
#[derive(Debug)]
pub enum ReadNamespacedPodDisruptionBudgetStatusResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedPodDisruptionBudgetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPodDisruptionBudgetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedPodDisruptionBudgetStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedPodDisruptionBudgetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// replace the specified PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`ReplaceNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        body: &crate::v1_12::api::policy::v1beta1::PodDisruptionBudget,
        optional: ReplaceNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let ReplaceNamespacedPodDisruptionBudgetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedPodDisruptionBudgetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum ReplaceNamespacedPodDisruptionBudgetResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Created(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedPodDisruptionBudgetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodDisruptionBudgetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodDisruptionBudgetResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// replace status of the specified PodDisruptionBudget
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedPodDisruptionBudgetStatusResponse`]`>` constructor, or [`ReplaceNamespacedPodDisruptionBudgetStatusResponse`] directly, to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_12::api::policy::v1beta1::PodDisruptionBudget,
        optional: ReplaceNamespacedPodDisruptionBudgetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedPodDisruptionBudgetStatusResponse>), crate::RequestError> {
        let ReplaceNamespacedPodDisruptionBudgetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedPodDisruptionBudgetStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedPodDisruptionBudgetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget_status`]
#[derive(Debug)]
pub enum ReplaceNamespacedPodDisruptionBudgetStatusResponse {
    Ok(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Created(crate::v1_12::api::policy::v1beta1::PodDisruptionBudget),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedPodDisruptionBudgetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodDisruptionBudgetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodDisruptionBudgetStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedPodDisruptionBudgetStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedPodDisruptionBudgetStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedPodDisruptionBudgetResponse`]`>` constructor, or [`WatchNamespacedPodDisruptionBudgetResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_pod_disruption_budget(
        namespace: &str,
        optional: crate::v1_12::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedPodDisruptionBudgetResponse>), crate::RequestError> {
        let crate::v1_12::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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

/// Use `<WatchNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::watch_namespaced_pod_disruption_budget`]
#[derive(Debug)]
pub enum WatchNamespacedPodDisruptionBudgetResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::WatchEvent<PodDisruptionBudget>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespacedPodDisruptionBudgetResponse {
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
                Ok((WatchNamespacedPodDisruptionBudgetResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespacedPodDisruptionBudgetResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespacedPodDisruptionBudgetResponse::Other, 0)),
        }
    }
}

// Generated from operation watchPolicyV1beta1PodDisruptionBudgetForAllNamespaces

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchPodDisruptionBudgetForAllNamespacesResponse`]`>` constructor, or [`WatchPodDisruptionBudgetForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_pod_disruption_budget_for_all_namespaces(
        optional: crate::v1_12::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchPodDisruptionBudgetForAllNamespacesResponse>), crate::RequestError> {
        let crate::v1_12::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/policy/v1beta1/poddisruptionbudgets?".to_string();
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

/// Use `<WatchPodDisruptionBudgetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::watch_pod_disruption_budget_for_all_namespaces`]
#[derive(Debug)]
pub enum WatchPodDisruptionBudgetForAllNamespacesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::WatchEvent<PodDisruptionBudget>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchPodDisruptionBudgetForAllNamespacesResponse {
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
                Ok((WatchPodDisruptionBudgetForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchPodDisruptionBudgetForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchPodDisruptionBudgetForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End policy/v1beta1/PodDisruptionBudget

impl crate::Resource for PodDisruptionBudget {
    fn api_version() -> &'static str {
        "policy/v1beta1"
    }

    fn group() -> &'static str {
        "policy"
    }

    fn kind() -> &'static str {
        "PodDisruptionBudget"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for PodDisruptionBudget {
    type Ty = crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for PodDisruptionBudget {
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
            type Value = PodDisruptionBudget;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct PodDisruptionBudget")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_12::api::policy::v1beta1::PodDisruptionBudgetSpec> = None;
                let mut value_status: Option<crate::v1_12::api::policy::v1beta1::PodDisruptionBudgetStatus> = None;

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

                Ok(PodDisruptionBudget {
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

impl serde::Serialize for PodDisruptionBudget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDisruptionBudget",
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
