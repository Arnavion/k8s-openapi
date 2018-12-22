// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIService

/// APIService represents a server for a particular GroupVersion. Name must be "version.group".
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIService {
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec contains information for locating and communicating with a server
    pub spec: Option<crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec>,

    /// Status contains derived information about an API server
    pub status: Option<crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus>,
}

// Begin apiregistration.k8s.io/v1beta1/APIService

// Generated from operation createApiregistrationV1beta1APIService

impl APIService {
    /// create an APIService
    ///
    /// Use [`CreateApiregistrationV1beta1APIServiceResponse`](./enum.CreateApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_apiregistration_v1beta1_api_service(
        body: &crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: CreateApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateApiregistrationV1beta1APIServiceOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::create_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.create_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct CreateApiregistrationV1beta1APIServiceOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`APIService::create_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.create_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum CreateApiregistrationV1beta1APIServiceResponse {
    Ok(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for CreateApiregistrationV1beta1APIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateApiregistrationV1beta1APIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((CreateApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiregistrationV1beta1APIService

impl APIService {
    /// delete an APIService
    ///
    /// Use [`DeleteApiregistrationV1beta1APIServiceResponse`](./enum.DeleteApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
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
    pub fn delete_apiregistration_v1beta1_api_service(
        name: &str,
        optional: DeleteApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteApiregistrationV1beta1APIServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::delete_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.delete_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct DeleteApiregistrationV1beta1APIServiceOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`APIService::delete_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.delete_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum DeleteApiregistrationV1beta1APIServiceResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteApiregistrationV1beta1APIServiceResponse {
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
                    Ok((DeleteApiregistrationV1beta1APIServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteApiregistrationV1beta1APIServiceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((DeleteApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiregistrationV1beta1CollectionAPIService

impl APIService {
    /// delete collection of APIService
    ///
    /// Use [`DeleteApiregistrationV1beta1CollectionAPIServiceResponse`](./enum.DeleteApiregistrationV1beta1CollectionAPIServiceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_apiregistration_v1beta1_collection_api_service(
        optional: DeleteApiregistrationV1beta1CollectionAPIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteApiregistrationV1beta1CollectionAPIServiceOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices?");
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::delete_apiregistration_v1beta1_collection_api_service`](./struct.APIService.html#method.delete_apiregistration_v1beta1_collection_api_service)
#[derive(Debug, Default)]
pub struct DeleteApiregistrationV1beta1CollectionAPIServiceOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`APIService::delete_apiregistration_v1beta1_collection_api_service`](./struct.APIService.html#method.delete_apiregistration_v1beta1_collection_api_service)
#[derive(Debug)]
pub enum DeleteApiregistrationV1beta1CollectionAPIServiceResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteApiregistrationV1beta1CollectionAPIServiceResponse {
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
                    Ok((DeleteApiregistrationV1beta1CollectionAPIServiceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteApiregistrationV1beta1CollectionAPIServiceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteApiregistrationV1beta1CollectionAPIServiceResponse::Unauthorized, 0)),
            _ => Ok((DeleteApiregistrationV1beta1CollectionAPIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation listApiregistrationV1beta1APIService

impl APIService {
    /// list or watch objects of kind APIService
    ///
    /// Use [`ListApiregistrationV1beta1APIServiceResponse`](./enum.ListApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_apiregistration_v1beta1_api_service(
        optional: ListApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListApiregistrationV1beta1APIServiceOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices?");
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::list_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.list_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct ListApiregistrationV1beta1APIServiceOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`APIService::list_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.list_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum ListApiregistrationV1beta1APIServiceResponse {
    Ok(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceList),
    Unauthorized,
    Other,
}

impl crate::Response for ListApiregistrationV1beta1APIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListApiregistrationV1beta1APIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((ListApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation patchApiregistrationV1beta1APIService

impl APIService {
    /// partially update the specified APIService
    ///
    /// Use [`PatchApiregistrationV1beta1APIServiceResponse`](./enum.PatchApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
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
    pub fn patch_apiregistration_v1beta1_api_service(
        name: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchApiregistrationV1beta1APIServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::patch_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.patch_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct PatchApiregistrationV1beta1APIServiceOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`APIService::patch_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.patch_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum PatchApiregistrationV1beta1APIServiceResponse {
    Ok(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for PatchApiregistrationV1beta1APIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchApiregistrationV1beta1APIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((PatchApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation readApiregistrationV1beta1APIService

impl APIService {
    /// read the specified APIService
    ///
    /// Use [`ReadApiregistrationV1beta1APIServiceResponse`](./enum.ReadApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
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
    pub fn read_apiregistration_v1beta1_api_service(
        name: &str,
        optional: ReadApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadApiregistrationV1beta1APIServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::read_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.read_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct ReadApiregistrationV1beta1APIServiceOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`APIService::read_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.read_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum ReadApiregistrationV1beta1APIServiceResponse {
    Ok(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for ReadApiregistrationV1beta1APIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadApiregistrationV1beta1APIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((ReadApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiregistrationV1beta1APIService

impl APIService {
    /// replace the specified APIService
    ///
    /// Use [`ReplaceApiregistrationV1beta1APIServiceResponse`](./enum.ReplaceApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
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
    pub fn replace_apiregistration_v1beta1_api_service(
        name: &str,
        body: &crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: ReplaceApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceApiregistrationV1beta1APIServiceOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::replace_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.replace_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct ReplaceApiregistrationV1beta1APIServiceOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`APIService::replace_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.replace_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum ReplaceApiregistrationV1beta1APIServiceResponse {
    Ok(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceApiregistrationV1beta1APIServiceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceApiregistrationV1beta1APIServiceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((ReplaceApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiregistrationV1beta1APIServiceStatus

impl APIService {
    /// replace status of the specified APIService
    ///
    /// Use [`ReplaceApiregistrationV1beta1APIServiceStatusResponse`](./enum.ReplaceApiregistrationV1beta1APIServiceStatusResponse.html) to parse the HTTP response.
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
    pub fn replace_apiregistration_v1beta1_api_service_status(
        name: &str,
        body: &crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: ReplaceApiregistrationV1beta1APIServiceStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceApiregistrationV1beta1APIServiceStatusOptional {
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::replace_apiregistration_v1beta1_api_service_status`](./struct.APIService.html#method.replace_apiregistration_v1beta1_api_service_status)
#[derive(Debug, Default)]
pub struct ReplaceApiregistrationV1beta1APIServiceStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`APIService::replace_apiregistration_v1beta1_api_service_status`](./struct.APIService.html#method.replace_apiregistration_v1beta1_api_service_status)
#[derive(Debug)]
pub enum ReplaceApiregistrationV1beta1APIServiceStatusResponse {
    Ok(crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceApiregistrationV1beta1APIServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceApiregistrationV1beta1APIServiceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceApiregistrationV1beta1APIServiceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceApiregistrationV1beta1APIServiceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchApiregistrationV1beta1APIService

impl APIService {
    /// watch changes to an object of kind APIService
    ///
    /// Use [`WatchApiregistrationV1beta1APIServiceResponse`](./enum.WatchApiregistrationV1beta1APIServiceResponse.html) to parse the HTTP response.
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
    pub fn watch_apiregistration_v1beta1_api_service(
        name: &str,
        optional: WatchApiregistrationV1beta1APIServiceOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchApiregistrationV1beta1APIServiceOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/watch/apiservices/{name}?", name = name);
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::watch_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.watch_apiregistration_v1beta1_api_service)
#[derive(Debug, Default)]
pub struct WatchApiregistrationV1beta1APIServiceOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`APIService::watch_apiregistration_v1beta1_api_service`](./struct.APIService.html#method.watch_apiregistration_v1beta1_api_service)
#[derive(Debug)]
pub enum WatchApiregistrationV1beta1APIServiceResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchApiregistrationV1beta1APIServiceResponse {
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
                Ok((WatchApiregistrationV1beta1APIServiceResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchApiregistrationV1beta1APIServiceResponse::Unauthorized, 0)),
            _ => Ok((WatchApiregistrationV1beta1APIServiceResponse::Other, 0)),
        }
    }
}

// Generated from operation watchApiregistrationV1beta1APIServiceList

impl APIService {
    /// watch individual changes to a list of APIService
    ///
    /// Use [`WatchApiregistrationV1beta1APIServiceListResponse`](./enum.WatchApiregistrationV1beta1APIServiceListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_apiregistration_v1beta1_api_service_list(
        optional: WatchApiregistrationV1beta1APIServiceListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchApiregistrationV1beta1APIServiceListOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/watch/apiservices?");
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
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`APIService::watch_apiregistration_v1beta1_api_service_list`](./struct.APIService.html#method.watch_apiregistration_v1beta1_api_service_list)
#[derive(Debug, Default)]
pub struct WatchApiregistrationV1beta1APIServiceListOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`APIService::watch_apiregistration_v1beta1_api_service_list`](./struct.APIService.html#method.watch_apiregistration_v1beta1_api_service_list)
#[derive(Debug)]
pub enum WatchApiregistrationV1beta1APIServiceListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchApiregistrationV1beta1APIServiceListResponse {
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
                Ok((WatchApiregistrationV1beta1APIServiceListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchApiregistrationV1beta1APIServiceListResponse::Unauthorized, 0)),
            _ => Ok((WatchApiregistrationV1beta1APIServiceListResponse::Other, 0)),
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
    type Ty = crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec> = None;
                let mut value_status: Option<crate::v1_7::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus> = None;

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
            0 +
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
