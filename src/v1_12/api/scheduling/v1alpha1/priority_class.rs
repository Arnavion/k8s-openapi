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
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_owned();
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
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((CreatePriorityClassResponse::Other(result), read))
            },
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
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    pub fn delete_collection_priority_class(
        delete_optional: crate::v1_12::DeleteOptional<'_>,
        list_optional: crate::v1_12::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionPriorityClassResponse>), crate::RequestError> {
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_owned();
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

/// Use `<DeleteCollectionPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::delete_collection_priority_class`]
#[derive(Debug)]
pub enum DeleteCollectionPriorityClassResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::api::scheduling::v1alpha1::PriorityClassList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((DeleteCollectionPriorityClassResponse::Other(result), read))
            },
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
        optional: crate::v1_12::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeletePriorityClassResponse>), crate::RequestError> {
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}",
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

/// Use `<DeletePriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::delete_priority_class`]
#[derive(Debug)]
pub enum DeletePriorityClassResponse {
    OkStatus(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Accepted(crate::v1_12::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((DeletePriorityClassResponse::Other(result), read))
            },
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
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_owned();
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

/// Use `<ListPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::list_priority_class`]
#[derive(Debug)]
pub enum ListPriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClassList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((ListPriorityClassResponse::Other(result), read))
            },
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
        optional: crate::v1_12::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchPriorityClassResponse>), crate::RequestError> {
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?",
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

/// Use `<PatchPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::patch_priority_class`]
#[derive(Debug)]
pub enum PatchPriorityClassResponse {
    Ok(crate::v1_12::api::scheduling::v1alpha1::PriorityClass),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((PatchPriorityClassResponse::Other(result), read))
            },
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
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?",
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
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((ReadPriorityClassResponse::Other(result), read))
            },
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
        let __url = format!("/apis/scheduling.k8s.io/v1alpha1/priorityclasses/{name}?",
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
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((ReplacePriorityClassResponse::Other(result), read))
            },
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
        let __url = "/apis/scheduling.k8s.io/v1alpha1/priorityclasses?".to_owned();
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

/// Use `<WatchPriorityClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`PriorityClass::watch_priority_class`]
#[derive(Debug)]
pub enum WatchPriorityClassResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::WatchEvent<PriorityClass>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
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
                Ok((WatchPriorityClassResponse::Other(result), read))
            },
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
