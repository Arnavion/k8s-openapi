// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIService

/// APIService represents a server for a particular GroupVersion. Name must be "version.group".
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIService {
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec contains information for locating and communicating with a server
    pub spec: Option<crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec>,

    /// Status contains derived information about an API server
    pub status: Option<crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus>,
}

// Begin apiregistration.k8s.io/v1beta1/APIService

// Generated from operation createApiregistrationV1beta1APIService

impl APIService {
    /// create an APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateAPIServiceResponse`]`>` constructor, or [`CreateAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_api_service(
        body: &crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: CreateAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateAPIServiceResponse>), crate::RequestError> {
        let CreateAPIServiceOptional {
            pretty,
        } = optional;
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`APIService::create_api_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateAPIServiceOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::create_api_service`]
#[derive(Debug)]
pub enum CreateAPIServiceResponse {
    Ok(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for CreateAPIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAPIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((CreateAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiregistrationV1beta1APIService

impl APIService {
    /// delete an APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteAPIServiceResponse`]`>` constructor, or [`DeleteAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the APIService
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_api_service(
        name: &str,
        optional: DeleteAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteAPIServiceResponse>), crate::RequestError> {
        let DeleteAPIServiceOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?", name = name);
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`APIService::delete_api_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteAPIServiceOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::delete_api_service`]
#[derive(Debug)]
pub enum DeleteAPIServiceResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteAPIServiceResponse {
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
                    Ok((DeleteAPIServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteAPIServiceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((DeleteAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiregistrationV1beta1CollectionAPIService

impl APIService {
    /// delete collection of APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionAPIServiceResponse`]`>` constructor, or [`DeleteCollectionAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_api_service(
        optional: DeleteCollectionAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionAPIServiceResponse>), crate::RequestError> {
        let DeleteCollectionAPIServiceOptional {
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
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_string();
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

/// Optional parameters of [`APIService::delete_collection_api_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionAPIServiceOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<DeleteCollectionAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::delete_collection_api_service`]
#[derive(Debug)]
pub enum DeleteCollectionAPIServiceResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionAPIServiceResponse {
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
                    Ok((DeleteCollectionAPIServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionAPIServiceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation listApiregistrationV1beta1APIService

impl APIService {
    /// list or watch objects of kind APIService
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListAPIServiceResponse`]`>` constructor, or [`ListAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_api_service(
        optional: crate::v1_8::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListAPIServiceResponse>), crate::RequestError> {
        let crate::v1_8::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_string();
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

/// Use `<ListAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::list_api_service`]
#[derive(Debug)]
pub enum ListAPIServiceResponse {
    Ok(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceList),
    Unauthorized,
    Other,
}

impl crate::Response for ListAPIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListAPIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((ListAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation patchApiregistrationV1beta1APIService

impl APIService {
    /// partially update the specified APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchAPIServiceResponse`]`>` constructor, or [`PatchAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the APIService
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_api_service(
        name: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchAPIServiceResponse>), crate::RequestError> {
        let PatchAPIServiceOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`APIService::patch_api_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchAPIServiceOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::patch_api_service`]
#[derive(Debug)]
pub enum PatchAPIServiceResponse {
    Ok(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for PatchAPIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchAPIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((PatchAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation readApiregistrationV1beta1APIService

impl APIService {
    /// read the specified APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadAPIServiceResponse`]`>` constructor, or [`ReadAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the APIService
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_api_service(
        name: &str,
        optional: ReadAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadAPIServiceResponse>), crate::RequestError> {
        let ReadAPIServiceOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?", name = name);
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

/// Optional parameters of [`APIService::read_api_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadAPIServiceOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::read_api_service`]
#[derive(Debug)]
pub enum ReadAPIServiceResponse {
    Ok(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for ReadAPIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadAPIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((ReadAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiregistrationV1beta1APIService

impl APIService {
    /// replace the specified APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceAPIServiceResponse`]`>` constructor, or [`ReplaceAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the APIService
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_api_service(
        name: &str,
        body: &crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: ReplaceAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceAPIServiceResponse>), crate::RequestError> {
        let ReplaceAPIServiceOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`APIService::replace_api_service`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceAPIServiceOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::replace_api_service`]
#[derive(Debug)]
pub enum ReplaceAPIServiceResponse {
    Ok(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceAPIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAPIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiregistrationV1beta1APIServiceStatus

impl APIService {
    /// replace status of the specified APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceAPIServiceStatusResponse`]`>` constructor, or [`ReplaceAPIServiceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the APIService
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_api_service_status(
        name: &str,
        body: &crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: ReplaceAPIServiceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceAPIServiceStatusResponse>), crate::RequestError> {
        let ReplaceAPIServiceStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`APIService::replace_api_service_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceAPIServiceStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceAPIServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::replace_api_service_status`]
#[derive(Debug)]
pub enum ReplaceAPIServiceStatusResponse {
    Ok(crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceAPIServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAPIServiceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceAPIServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAPIServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchApiregistrationV1beta1APIService

impl APIService {
    /// list or watch objects of kind APIService
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchAPIServiceResponse`]`>` constructor, or [`WatchAPIServiceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_api_service(
        optional: crate::v1_8::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchAPIServiceResponse>), crate::RequestError> {
        let crate::v1_8::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_string();
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

/// Use `<WatchAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::watch_api_service`]
#[derive(Debug)]
pub enum WatchAPIServiceResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent<APIService>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchAPIServiceResponse {
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
                Ok((WatchAPIServiceResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((WatchAPIServiceResponse::Other, 0)),
        }
    }
}

// End apiregistration.k8s.io/v1beta1/APIService

impl crate::Resource for APIService {
    fn api_version() -> &'static str {
        "apiregistration.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "apiregistration.k8s.io"
    }

    fn kind() -> &'static str {
        "APIService"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for APIService {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for APIService {
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
            type Value = APIService;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct APIService")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec> = None;
                let mut value_status: Option<crate::v1_8::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus> = None;

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

                Ok(APIService {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIService",
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

impl serde::Serialize for APIService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIService",
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
