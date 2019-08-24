// Generated from definition io.k8s.api.core.v1.PodTemplate

/// PodTemplate describes a template for creating copies of a predefined pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodTemplate {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Template defines the pods that will be created from this pod template. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub template: Option<crate::v1_14::api::core::v1::PodTemplateSpec>,
}

// Begin /v1/PodTemplate

// Generated from operation createCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// create a PodTemplate
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedPodTemplateResponse`]`>` constructor, or [`CreateNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_pod_template(
        namespace: &str,
        body: &crate::v1_14::api::core::v1::PodTemplate,
        optional: CreateNamespacedPodTemplateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedPodTemplateResponse>), crate::RequestError> {
        let CreateNamespacedPodTemplateOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
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

/// Optional parameters of [`PodTemplate::create_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedPodTemplateOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::create_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedPodTemplateResponse {
    Ok(crate::v1_14::api::core::v1::PodTemplate),
    Created(crate::v1_14::api::core::v1::PodTemplate),
    Accepted(crate::v1_14::api::core::v1::PodTemplate),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateNamespacedPodTemplateResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodTemplateResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodTemplateResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodTemplateResponse::Accepted(result), buf.len()))
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
                Ok((CreateNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPodTemplate

impl PodTemplate {
    /// delete collection of PodTemplate
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedPodTemplateResponse`]`>` constructor, or [`DeleteCollectionNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_namespaced_pod_template(
        namespace: &str,
        delete_optional: crate::v1_14::DeleteOptional<'_>,
        list_optional: crate::v1_14::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedPodTemplateResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates?",
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

/// Use `<DeleteCollectionNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::delete_collection_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedPodTemplateResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::core::v1::PodTemplateList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionNamespacedPodTemplateResponse {
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
                    Ok((DeleteCollectionNamespacedPodTemplateResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedPodTemplateResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCollectionNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// delete a PodTemplate
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedPodTemplateResponse`]`>` constructor, or [`DeleteNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodTemplate
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_namespaced_pod_template(
        name: &str,
        namespace: &str,
        optional: crate::v1_14::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedPodTemplateResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates/{name}",
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

/// Use `<DeleteNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::delete_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteNamespacedPodTemplateResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::core::v1::PodTemplate),
    Accepted(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteNamespacedPodTemplateResponse {
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
                    Ok((DeleteNamespacedPodTemplateResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedPodTemplateResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespacedPodTemplateResponse::Accepted(result), buf.len()))
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
                Ok((DeleteNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// list or watch objects of kind PodTemplate
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedPodTemplateResponse`]`>` constructor, or [`ListNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_pod_template(
        namespace: &str,
        optional: crate::v1_14::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedPodTemplateResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates?",
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

/// Use `<ListNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::list_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListNamespacedPodTemplateResponse {
    Ok(crate::v1_14::api::core::v1::PodTemplateList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListNamespacedPodTemplateResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedPodTemplateResponse::Ok(result), buf.len()))
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
                Ok((ListNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreV1PodTemplateForAllNamespaces

impl PodTemplate {
    /// list or watch objects of kind PodTemplate
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListPodTemplateForAllNamespacesResponse`]`>` constructor, or [`ListPodTemplateForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_pod_template_for_all_namespaces(
        optional: crate::v1_14::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListPodTemplateForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/api/v1/podtemplates?".to_owned();
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

/// Use `<ListPodTemplateForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::list_pod_template_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListPodTemplateForAllNamespacesResponse {
    Ok(crate::v1_14::api::core::v1::PodTemplateList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListPodTemplateForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListPodTemplateForAllNamespacesResponse::Ok(result), buf.len()))
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
                Ok((ListPodTemplateForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// partially update the specified PodTemplate
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedPodTemplateResponse`]`>` constructor, or [`PatchNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodTemplate
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
    pub fn patch_namespaced_pod_template(
        name: &str,
        namespace: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_14::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedPodTemplateResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::patch_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNamespacedPodTemplateResponse {
    Ok(crate::v1_14::api::core::v1::PodTemplate),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchNamespacedPodTemplateResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedPodTemplateResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// read the specified PodTemplate
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedPodTemplateResponse`]`>` constructor, or [`ReadNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodTemplate
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_pod_template(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodTemplateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedPodTemplateResponse>), crate::RequestError> {
        let ReadNamespacedPodTemplateOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates/{name}?",
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

/// Optional parameters of [`PodTemplate::read_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedPodTemplateOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::read_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedPodTemplateResponse {
    Ok(crate::v1_14::api::core::v1::PodTemplate),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedPodTemplateResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedPodTemplateResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// replace the specified PodTemplate
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedPodTemplateResponse`]`>` constructor, or [`ReplaceNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodTemplate
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
    pub fn replace_namespaced_pod_template(
        name: &str,
        namespace: &str,
        body: &crate::v1_14::api::core::v1::PodTemplate,
        optional: ReplaceNamespacedPodTemplateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedPodTemplateResponse>), crate::RequestError> {
        let ReplaceNamespacedPodTemplateOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
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

/// Optional parameters of [`PodTemplate::replace_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedPodTemplateOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::replace_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNamespacedPodTemplateResponse {
    Ok(crate::v1_14::api::core::v1::PodTemplate),
    Created(crate::v1_14::api::core::v1::PodTemplate),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceNamespacedPodTemplateResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodTemplateResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedPodTemplateResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreV1NamespacedPodTemplate

impl PodTemplate {
    /// list or watch objects of kind PodTemplate
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedPodTemplateResponse`]`>` constructor, or [`WatchNamespacedPodTemplateResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_pod_template(
        namespace: &str,
        optional: crate::v1_14::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedPodTemplateResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/podtemplates?",
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

/// Use `<WatchNamespacedPodTemplateResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::watch_namespaced_pod_template`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchNamespacedPodTemplateResponse {
    Ok(crate::v1_14::apimachinery::pkg::apis::meta::v1::WatchEvent<PodTemplate>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchNamespacedPodTemplateResponse {
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
                Ok((WatchNamespacedPodTemplateResponse::Ok(result), byte_offset))
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
                Ok((WatchNamespacedPodTemplateResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreV1PodTemplateForAllNamespaces

impl PodTemplate {
    /// list or watch objects of kind PodTemplate
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchPodTemplateForAllNamespacesResponse`]`>` constructor, or [`WatchPodTemplateForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_pod_template_for_all_namespaces(
        optional: crate::v1_14::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchPodTemplateForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/api/v1/podtemplates?".to_owned();
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

/// Use `<WatchPodTemplateForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodTemplate::watch_pod_template_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchPodTemplateForAllNamespacesResponse {
    Ok(crate::v1_14::apimachinery::pkg::apis::meta::v1::WatchEvent<PodTemplate>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchPodTemplateForAllNamespacesResponse {
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
                Ok((WatchPodTemplateForAllNamespacesResponse::Ok(result), byte_offset))
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
                Ok((WatchPodTemplateForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// End /v1/PodTemplate

impl crate::Resource for PodTemplate {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "PodTemplate"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for PodTemplate {
    type Ty = crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for PodTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_template,
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
                            "template" => Field::Key_template,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodTemplate;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct PodTemplate")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_template: Option<crate::v1_14::api::core::v1::PodTemplateSpec> = None;

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
                        Field::Key_template => value_template = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodTemplate {
                    metadata: value_metadata,
                    template: value_template,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodTemplate",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "template",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodTemplate",
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.template.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.template {
            serde::ser::SerializeStruct::serialize_field(&mut state, "template", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
