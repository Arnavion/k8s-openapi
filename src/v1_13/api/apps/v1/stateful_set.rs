// Generated from definition io.k8s.api.apps.v1.StatefulSet

/// StatefulSet represents a set of pods with consistent identities. Identities are defined as:
///  - Network: A single stable DNS and hostname.
///  - Storage: As many VolumeClaims as requested.
/// The StatefulSet guarantees that a given network identity will always map to the same storage identity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSet {
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the desired identities of pods in this set.
    pub spec: Option<crate::v1_13::api::apps::v1::StatefulSetSpec>,

    /// Status is the current status of Pods in this StatefulSet. This data may be out of date by some window of time.
    pub status: Option<crate::v1_13::api::apps::v1::StatefulSetStatus>,
}

// Begin apps/v1/StatefulSet

// Generated from operation createAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// create a StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedStatefulSetResponse`]`>` constructor, or [`CreateNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn create_namespaced_stateful_set(
        namespace: &str,
        body: &crate::v1_13::api::apps::v1::StatefulSet,
        optional: CreateNamespacedStatefulSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedStatefulSetResponse>), crate::RequestError> {
        let CreateNamespacedStatefulSetOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
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

/// Optional parameters of [`StatefulSet::create_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedStatefulSetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::create_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedStatefulSetResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Created(crate::v1_13::api::apps::v1::StatefulSet),
    Accepted(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateNamespacedStatefulSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedStatefulSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedStatefulSetResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedStatefulSetResponse::Accepted(result), buf.len()))
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
                Ok((CreateNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteAppsV1CollectionNamespacedStatefulSet

impl StatefulSet {
    /// delete collection of StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedStatefulSetResponse`]`>` constructor, or [`DeleteCollectionNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_namespaced_stateful_set(
        namespace: &str,
        delete_optional: crate::v1_13::DeleteOptional<'_>,
        list_optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedStatefulSetResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
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

/// Use `<DeleteCollectionNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::delete_collection_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedStatefulSetResponse {
    OkStatus(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_13::api::apps::v1::StatefulSetList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionNamespacedStatefulSetResponse {
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
                    Ok((DeleteCollectionNamespacedStatefulSetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedStatefulSetResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCollectionNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// delete a StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedStatefulSetResponse`]`>` constructor, or [`DeleteNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_namespaced_stateful_set(
        name: &str,
        namespace: &str,
        optional: crate::v1_13::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedStatefulSetResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<DeleteNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::delete_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteNamespacedStatefulSetResponse {
    OkStatus(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_13::api::apps::v1::StatefulSet),
    Accepted(crate::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteNamespacedStatefulSetResponse {
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
                    Ok((DeleteNamespacedStatefulSetResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedStatefulSetResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespacedStatefulSetResponse::Accepted(result), buf.len()))
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
                Ok((DeleteNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// list or watch objects of kind StatefulSet
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedStatefulSetResponse`]`>` constructor, or [`ListNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn list_namespaced_stateful_set(
        namespace: &str,
        optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedStatefulSetResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
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

/// Use `<ListNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::list_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListNamespacedStatefulSetResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSetList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListNamespacedStatefulSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedStatefulSetResponse::Ok(result), buf.len()))
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
                Ok((ListNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listAppsV1StatefulSetForAllNamespaces

impl StatefulSet {
    /// list or watch objects of kind StatefulSet
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListStatefulSetForAllNamespacesResponse`]`>` constructor, or [`ListStatefulSetForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_stateful_set_for_all_namespaces(
        optional: crate::v1_13::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListStatefulSetForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/apis/apps/v1/statefulsets?".to_owned();
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

/// Use `<ListStatefulSetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::list_stateful_set_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListStatefulSetForAllNamespacesResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSetList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListStatefulSetForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListStatefulSetForAllNamespacesResponse::Ok(result), buf.len()))
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
                Ok((ListStatefulSetForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// partially update the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedStatefulSetResponse`]`>` constructor, or [`PatchNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
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
    #[cfg(feature = "api")]
    pub fn patch_namespaced_stateful_set(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_13::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedStatefulSetResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::patch_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNamespacedStatefulSetResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchNamespacedStatefulSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedStatefulSetResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchAppsV1NamespacedStatefulSetStatus

impl StatefulSet {
    /// partially update status of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedStatefulSetStatusResponse`]`>` constructor, or [`PatchNamespacedStatefulSetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
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
    #[cfg(feature = "api")]
    pub fn patch_namespaced_stateful_set_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_13::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedStatefulSetStatusResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNamespacedStatefulSetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::patch_namespaced_stateful_set_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNamespacedStatefulSetStatusResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchNamespacedStatefulSetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedStatefulSetStatusResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedStatefulSetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// read the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedStatefulSetResponse`]`>` constructor, or [`ReadNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_stateful_set(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedStatefulSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedStatefulSetResponse>), crate::RequestError> {
        let ReadNamespacedStatefulSetOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`StatefulSet::read_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedStatefulSetOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::read_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedStatefulSetResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedStatefulSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedStatefulSetResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAppsV1NamespacedStatefulSetStatus

impl StatefulSet {
    /// read status of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedStatefulSetStatusResponse`]`>` constructor, or [`ReadNamespacedStatefulSetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_stateful_set_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedStatefulSetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedStatefulSetStatusResponse>), crate::RequestError> {
        let ReadNamespacedStatefulSetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`StatefulSet::read_namespaced_stateful_set_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedStatefulSetStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedStatefulSetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::read_namespaced_stateful_set_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedStatefulSetStatusResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedStatefulSetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedStatefulSetStatusResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedStatefulSetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// replace the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedStatefulSetResponse`]`>` constructor, or [`ReplaceNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
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
    #[cfg(feature = "api")]
    pub fn replace_namespaced_stateful_set(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::apps::v1::StatefulSet,
        optional: ReplaceNamespacedStatefulSetOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedStatefulSetResponse>), crate::RequestError> {
        let ReplaceNamespacedStatefulSetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`StatefulSet::replace_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedStatefulSetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::replace_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNamespacedStatefulSetResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Created(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceNamespacedStatefulSetResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedStatefulSetResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedStatefulSetResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAppsV1NamespacedStatefulSetStatus

impl StatefulSet {
    /// replace status of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedStatefulSetStatusResponse`]`>` constructor, or [`ReplaceNamespacedStatefulSetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StatefulSet
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
    #[cfg(feature = "api")]
    pub fn replace_namespaced_stateful_set_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::apps::v1::StatefulSet,
        optional: ReplaceNamespacedStatefulSetStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedStatefulSetStatusResponse>), crate::RequestError> {
        let ReplaceNamespacedStatefulSetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`StatefulSet::replace_namespaced_stateful_set_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedStatefulSetStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedStatefulSetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::replace_namespaced_stateful_set_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNamespacedStatefulSetStatusResponse {
    Ok(crate::v1_13::api::apps::v1::StatefulSet),
    Created(crate::v1_13::api::apps::v1::StatefulSet),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceNamespacedStatefulSetStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedStatefulSetStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedStatefulSetStatusResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedStatefulSetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchAppsV1NamespacedStatefulSet

impl StatefulSet {
    /// list or watch objects of kind StatefulSet
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedStatefulSetResponse`]`>` constructor, or [`WatchNamespacedStatefulSetResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn watch_namespaced_stateful_set(
        namespace: &str,
        optional: crate::v1_13::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedStatefulSetResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
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

/// Use `<WatchNamespacedStatefulSetResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::watch_namespaced_stateful_set`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchNamespacedStatefulSetResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent<StatefulSet>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchNamespacedStatefulSetResponse {
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
                Ok((WatchNamespacedStatefulSetResponse::Ok(result), byte_offset))
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
                Ok((WatchNamespacedStatefulSetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchAppsV1StatefulSetForAllNamespaces

impl StatefulSet {
    /// list or watch objects of kind StatefulSet
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchStatefulSetForAllNamespacesResponse`]`>` constructor, or [`WatchStatefulSetForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_stateful_set_for_all_namespaces(
        optional: crate::v1_13::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchStatefulSetForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/apis/apps/v1/statefulsets?".to_owned();
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

/// Use `<WatchStatefulSetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`StatefulSet::watch_stateful_set_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchStatefulSetForAllNamespacesResponse {
    Ok(crate::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent<StatefulSet>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchStatefulSetForAllNamespacesResponse {
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
                Ok((WatchStatefulSetForAllNamespacesResponse::Ok(result), byte_offset))
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
                Ok((WatchStatefulSetForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// End apps/v1/StatefulSet

impl crate::Resource for StatefulSet {
    fn api_version() -> &'static str {
        "apps/v1"
    }

    fn group() -> &'static str {
        "apps"
    }

    fn kind() -> &'static str {
        "StatefulSet"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for StatefulSet {
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for StatefulSet {
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
            type Value = StatefulSet;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct StatefulSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_13::api::apps::v1::StatefulSetSpec> = None;
                let mut value_status: Option<crate::v1_13::api::apps::v1::StatefulSetStatus> = None;

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

                Ok(StatefulSet {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSet",
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

impl serde::Serialize for StatefulSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSet",
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
