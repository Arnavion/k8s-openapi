// Generated from definition io.k8s.api.scheduling.v1alpha1.PriorityClass

/// PriorityClass defines mapping from a priority class name to the priority integer value. The value can be any valid integer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriorityClass {
    /// description is an arbitrary string that usually provides guidelines on when this priority class should be used.
    pub description: Option<String>,

    /// globalDefault specifies whether this PriorityClass should be considered as the default priority for pods that do not have any priority class. Only one PriorityClass can be marked as `globalDefault`. However, if more than one PriorityClasses exists with their `globalDefault` field set to true, the smallest value of such global default PriorityClasses will be used as the default priority.
    pub global_default: Option<bool>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The value of this priority class. This is the actual priority that pods receive when they have the name of this class in their pod spec.
    pub value: i32,
}

// Begin scheduling.k8s.io/v1alpha1/PriorityClass

// Generated from operation createSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// create a PriorityClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreatePriorityClassResponse`]`>` constructor, or [`CreatePriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_priority_class(
        body: &crate::v1_12::api::scheduling::v1alpha1::PriorityClass,
        optional: CreatePriorityClassOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreatePriorityClassResponse>), crate::RequestError> {
        let CreatePriorityClassOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_string();
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

/// Optional parameters of [`PriorityClass::create_priority_class`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreatePriorityClassOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreatePriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::create_priority_class`]
#[derive(Debug)]
pub enum CreatePriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Created(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Accepted(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Unauthorized,
    Other,
}

impl crate::Response for CreatePriorityClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreatePriorityClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreatePriorityClassResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreatePriorityClassResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreatePriorityClassResponse::Unauthorized, 0)),
            _ => Ok((CreatePriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteSchedulingV1alpha1CollectionPriorityClass

impl PriorityClass {
    /// delete collection of PriorityClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionPriorityClassResponse`]`>` constructor, or [`DeleteCollectionPriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_priority_class(
        optional: DeleteCollectionPriorityClassOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionPriorityClassResponse>), crate::RequestError> {
        let DeleteCollectionPriorityClassOptional {
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
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_string();
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

/// Optional parameters of [`PriorityClass::delete_collection_priority_class`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionPriorityClassOptional<'a> {
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

/// Use `<DeleteCollectionPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::delete_collection_priority_class`]
#[derive(Debug)]
pub enum DeleteCollectionPriorityClassResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionPriorityClassResponse {
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
                    Ok((DeleteCollectionPriorityClassResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionPriorityClassResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionPriorityClassResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionPriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// delete a PriorityClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeletePriorityClassResponse`]`>` constructor, or [`DeletePriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PriorityClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_priority_class(
        name: &str,
        optional: DeletePriorityClassOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeletePriorityClassResponse>), crate::RequestError> {
        let DeletePriorityClassOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?", name = name);
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

/// Optional parameters of [`PriorityClass::delete_priority_class`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeletePriorityClassOptional<'a> {
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

/// Use `<DeletePriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::delete_priority_class`]
#[derive(Debug)]
pub enum DeletePriorityClassResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Accepted(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeletePriorityClassResponse {
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
                    Ok((DeletePriorityClassResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeletePriorityClassResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeletePriorityClassResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeletePriorityClassResponse::Unauthorized, 0)),
            _ => Ok((DeletePriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation listSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// list or watch objects of kind PriorityClass
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListPriorityClassResponse`]`>` constructor, or [`ListPriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_priority_class(
        optional: crate::v1_12::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListPriorityClassResponse>), crate::RequestError> {
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
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_string();
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

/// Use `<ListPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::list_priority_class`]
#[derive(Debug)]
pub enum ListPriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClassList),
    Unauthorized,
    Other,
}

impl crate::Response for ListPriorityClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListPriorityClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListPriorityClassResponse::Unauthorized, 0)),
            _ => Ok((ListPriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation patchSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// partially update the specified PriorityClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchPriorityClassResponse`]`>` constructor, or [`PatchPriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PriorityClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_priority_class(
        name: &str,
        body: &crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchPriorityClassOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchPriorityClassResponse>), crate::RequestError> {
        let PatchPriorityClassOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?", name = name);
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

/// Optional parameters of [`PriorityClass::patch_priority_class`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchPriorityClassOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::patch_priority_class`]
#[derive(Debug)]
pub enum PatchPriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Unauthorized,
    Other,
}

impl crate::Response for PatchPriorityClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchPriorityClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchPriorityClassResponse::Unauthorized, 0)),
            _ => Ok((PatchPriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation readSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// read the specified PriorityClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadPriorityClassResponse`]`>` constructor, or [`ReadPriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PriorityClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_priority_class(
        name: &str,
        optional: ReadPriorityClassOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadPriorityClassResponse>), crate::RequestError> {
        let ReadPriorityClassOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?", name = name);
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

/// Optional parameters of [`PriorityClass::read_priority_class`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadPriorityClassOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::read_priority_class`]
#[derive(Debug)]
pub enum ReadPriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Unauthorized,
    Other,
}

impl crate::Response for ReadPriorityClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadPriorityClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadPriorityClassResponse::Unauthorized, 0)),
            _ => Ok((ReadPriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// replace the specified PriorityClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplacePriorityClassResponse`]`>` constructor, or [`ReplacePriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PriorityClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_priority_class(
        name: &str,
        body: &crate::v1_12::api::scheduling::v1alpha1::PriorityClass,
        optional: ReplacePriorityClassOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplacePriorityClassResponse>), crate::RequestError> {
        let ReplacePriorityClassOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?", name = name);
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

/// Optional parameters of [`PriorityClass::replace_priority_class`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplacePriorityClassOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplacePriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::replace_priority_class`]
#[derive(Debug)]
pub enum ReplacePriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Created(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Unauthorized,
    Other,
}

impl crate::Response for ReplacePriorityClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplacePriorityClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplacePriorityClassResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplacePriorityClassResponse::Unauthorized, 0)),
            _ => Ok((ReplacePriorityClassResponse::Other, 0)),
        }
    }
}

// Generated from operation watchSchedulingV1alpha1PriorityClass

impl PriorityClass {
    /// list or watch objects of kind PriorityClass
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchPriorityClassResponse`]`>` constructor, or [`WatchPriorityClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_priority_class(
        optional: crate::v1_12::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchPriorityClassResponse>), crate::RequestError> {
        let crate::v1_12::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_string();
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

/// Use `<WatchPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::watch_priority_class`]
#[derive(Debug)]
pub enum WatchPriorityClassResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::WatchEvent<PriorityClass>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchPriorityClassResponse {
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
                Ok((WatchPriorityClassResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchPriorityClassResponse::Unauthorized, 0)),
            _ => Ok((WatchPriorityClassResponse::Other, 0)),
        }
    }
}

// End scheduling.k8s.io/v1alpha1/PriorityClass

impl crate::Resource for PriorityClass {
    fn api_version() -> &'static str {
        "scheduling.k8s.io/v1alpha1"
    }

    fn group() -> &'static str {
        "scheduling.k8s.io"
    }

    fn kind() -> &'static str {
        "PriorityClass"
    }

    fn version() -> &'static str {
        "v1alpha1"
    }
}

impl crate::Metadata for PriorityClass {
    type Ty = crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for PriorityClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_description,
            Key_global_default,
            Key_metadata,
            Key_value,
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
                            "description" => Field::Key_description,
                            "globalDefault" => Field::Key_global_default,
                            "metadata" => Field::Key_metadata,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PriorityClass;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct PriorityClass")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_description: Option<String> = None;
                let mut value_global_default: Option<bool> = None;
                let mut value_metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_value: Option<i32> = None;

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
                        Field::Key_description => value_description = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_global_default => value_global_default = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityClass {
                    description: value_description,
                    global_default: value_global_default,
                    metadata: value_metadata,
                    value: value_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PriorityClass",
            &[
                "apiVersion",
                "kind",
                "description",
                "globalDefault",
                "metadata",
                "value",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PriorityClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PriorityClass",
            3 +
            self.description.as_ref().map_or(0, |_| 1) +
            self.global_default.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.description {
            serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.global_default {
            serde::ser::SerializeStruct::serialize_field(&mut state, "globalDefault", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        serde::ser::SerializeStruct::end(state)
    }
}
