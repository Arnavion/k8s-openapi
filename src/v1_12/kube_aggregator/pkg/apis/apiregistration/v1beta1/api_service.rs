// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIService

/// APIService represents a server for a particular GroupVersion. Name must be "version.group".
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIService {
    pub metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec contains information for locating and communicating with a server
    pub spec: Option<crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec>,

    /// Status contains derived information about an API server
    pub status: Option<crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus>,
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
    #[cfg(feature = "api")]
    pub fn create_api_service(
        body: &crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: CreateAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateAPIServiceResponse>), crate::RequestError> {
        let CreateAPIServiceOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`APIService::create_api_service`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateAPIServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::create_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateAPIServiceResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Created(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Accepted(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAPIServiceResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAPIServiceResponse::Accepted(result), buf.len()))
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
                Ok((CreateAPIServiceResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn delete_api_service(
        name: &str,
        optional: crate::v1_12::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteAPIServiceResponse>), crate::RequestError> {
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<DeleteAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::delete_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteAPIServiceResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Accepted(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteAPIServiceResponse::Accepted(result), buf.len()))
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
                Ok((DeleteAPIServiceResponse::Other(result), read))
            },
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
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_api_service(
        delete_optional: crate::v1_12::DeleteOptional<'_>,
        list_optional: crate::v1_12::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionAPIServiceResponse>), crate::RequestError> {
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_owned();
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

/// Use `<DeleteCollectionAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::delete_collection_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionAPIServiceResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((DeleteCollectionAPIServiceResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn list_api_service(
        optional: crate::v1_12::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListAPIServiceResponse>), crate::RequestError> {
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_owned();
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

/// Use `<ListAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::list_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListAPIServiceResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ListAPIServiceResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn patch_api_service(
        name: &str,
        body: &crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_12::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchAPIServiceResponse>), crate::RequestError> {
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::patch_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchAPIServiceResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((PatchAPIServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchApiregistrationV1beta1APIServiceStatus

impl APIService {
    /// partially update status of the specified APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchAPIServiceStatusResponse`]`>` constructor, or [`PatchAPIServiceStatusResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn patch_api_service_status(
        name: &str,
        body: &crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_12::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchAPIServiceStatusResponse>), crate::RequestError> {
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_12::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchAPIServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::patch_api_service_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchAPIServiceStatusResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchAPIServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchAPIServiceStatusResponse::Ok(result), buf.len()))
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
                Ok((PatchAPIServiceStatusResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn read_api_service(
        name: &str,
        optional: ReadAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadAPIServiceResponse>), crate::RequestError> {
        let ReadAPIServiceOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`APIService::read_api_service`]
#[cfg(feature = "api")]
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
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadAPIServiceResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((ReadAPIServiceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readApiregistrationV1beta1APIServiceStatus

impl APIService {
    /// read status of the specified APIService
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadAPIServiceStatusResponse`]`>` constructor, or [`ReadAPIServiceStatusResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn read_api_service_status(
        name: &str,
        optional: ReadAPIServiceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadAPIServiceStatusResponse>), crate::RequestError> {
        let ReadAPIServiceStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`APIService::read_api_service_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadAPIServiceStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadAPIServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::read_api_service_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadAPIServiceStatusResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadAPIServiceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadAPIServiceStatusResponse::Ok(result), buf.len()))
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
                Ok((ReadAPIServiceStatusResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn replace_api_service(
        name: &str,
        body: &crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: ReplaceAPIServiceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceAPIServiceResponse>), crate::RequestError> {
        let ReplaceAPIServiceOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`APIService::replace_api_service`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceAPIServiceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::replace_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceAPIServiceResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Created(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAPIServiceResponse::Created(result), buf.len()))
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
                Ok((ReplaceAPIServiceResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn replace_api_service_status(
        name: &str,
        body: &crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        optional: ReplaceAPIServiceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceAPIServiceStatusResponse>), crate::RequestError> {
        let ReplaceAPIServiceStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Optional parameters of [`APIService::replace_api_service_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceAPIServiceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceAPIServiceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::replace_api_service_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceAPIServiceStatusResponse {
    Ok(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Created(crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAPIServiceStatusResponse::Created(result), buf.len()))
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
                Ok((ReplaceAPIServiceStatusResponse::Other(result), read))
            },
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
    #[cfg(feature = "api")]
    pub fn watch_api_service(
        optional: crate::v1_12::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchAPIServiceResponse>), crate::RequestError> {
        let __url = "/apis/apiregistration.k8s.io/v1beta1/apiservices?".to_owned();
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

/// Use `<WatchAPIServiceResponse as Response>::try_from_parts` to parse the HTTP response body of [`APIService::watch_api_service`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchAPIServiceResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::WatchEvent<APIService>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
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
                Ok((WatchAPIServiceResponse::Other(result), read))
            },
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
    type Ty = crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
                let mut value_metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec> = None;
                let mut value_status: Option<crate::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus> = None;

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
