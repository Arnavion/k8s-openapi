// Generated from definition io.k8s.api.apps.v1beta1.Scale

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<crate::v1_14::api::apps::v1beta1::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<crate::v1_14::api::apps::v1beta1::ScaleStatus>,
}

// Begin apps/v1beta1/Scale

// Generated from operation patchAppsV1beta1NamespacedDeploymentScale

impl Scale {
    /// partially update scale of the specified Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedDeploymentScaleResponse`]`>` constructor, or [`PatchNamespacedDeploymentScaleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
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
    pub fn patch_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_14::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedDeploymentScaleResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale?",
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

/// Use `<PatchNamespacedDeploymentScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::patch_namespaced_deployment_scale`]
#[derive(Debug)]
pub enum PatchNamespacedDeploymentScaleResponse {
    Ok(crate::v1_14::api::apps::v1beta1::Scale),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for PatchNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedDeploymentScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchAppsV1beta1NamespacedStatefulSetScale

impl Scale {
    /// partially update scale of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedStatefulSetScaleResponse`]`>` constructor, or [`PatchNamespacedStatefulSetScaleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
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
    pub fn patch_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_14::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedStatefulSetScaleResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/scale?",
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

/// Use `<PatchNamespacedStatefulSetScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::patch_namespaced_stateful_set_scale`]
#[derive(Debug)]
pub enum PatchNamespacedStatefulSetScaleResponse {
    Ok(crate::v1_14::api::apps::v1beta1::Scale),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for PatchNamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedStatefulSetScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAppsV1beta1NamespacedDeploymentScale

impl Scale {
    /// read scale of the specified Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedDeploymentScaleResponse`]`>` constructor, or [`ReadNamespacedDeploymentScaleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedDeploymentScaleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedDeploymentScaleResponse>), crate::RequestError> {
        let ReadNamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale?",
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

/// Optional parameters of [`Scale::read_namespaced_deployment_scale`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedDeploymentScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::read_namespaced_deployment_scale`]
#[derive(Debug)]
pub enum ReadNamespacedDeploymentScaleResponse {
    Ok(crate::v1_14::api::apps::v1beta1::Scale),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReadNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedDeploymentScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAppsV1beta1NamespacedStatefulSetScale

impl Scale {
    /// read scale of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedStatefulSetScaleResponse`]`>` constructor, or [`ReadNamespacedStatefulSetScaleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedStatefulSetScaleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedStatefulSetScaleResponse>), crate::RequestError> {
        let ReadNamespacedStatefulSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/scale?",
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

/// Optional parameters of [`Scale::read_namespaced_stateful_set_scale`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedStatefulSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedStatefulSetScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::read_namespaced_stateful_set_scale`]
#[derive(Debug)]
pub enum ReadNamespacedStatefulSetScaleResponse {
    Ok(crate::v1_14::api::apps::v1beta1::Scale),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReadNamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedStatefulSetScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAppsV1beta1NamespacedDeploymentScale

impl Scale {
    /// replace scale of the specified Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedDeploymentScaleResponse`]`>` constructor, or [`ReplaceNamespacedDeploymentScaleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
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
    pub fn replace_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_14::api::apps::v1beta1::Scale,
        optional: ReplaceNamespacedDeploymentScaleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedDeploymentScaleResponse>), crate::RequestError> {
        let ReplaceNamespacedDeploymentScaleOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale?",
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

/// Optional parameters of [`Scale::replace_namespaced_deployment_scale`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedDeploymentScaleOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedDeploymentScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::replace_namespaced_deployment_scale`]
#[derive(Debug)]
pub enum ReplaceNamespacedDeploymentScaleResponse {
    Ok(crate::v1_14::api::apps::v1beta1::Scale),
    Created(crate::v1_14::api::apps::v1beta1::Scale),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReplaceNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedDeploymentScaleResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedDeploymentScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAppsV1beta1NamespacedStatefulSetScale

impl Scale {
    /// replace scale of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedStatefulSetScaleResponse`]`>` constructor, or [`ReplaceNamespacedStatefulSetScaleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
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
    pub fn replace_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_14::api::apps::v1beta1::Scale,
        optional: ReplaceNamespacedStatefulSetScaleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedStatefulSetScaleResponse>), crate::RequestError> {
        let ReplaceNamespacedStatefulSetScaleOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/scale?",
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

/// Optional parameters of [`Scale::replace_namespaced_stateful_set_scale`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedStatefulSetScaleOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedStatefulSetScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::replace_namespaced_stateful_set_scale`]
#[derive(Debug)]
pub enum ReplaceNamespacedStatefulSetScaleResponse {
    Ok(crate::v1_14::api::apps::v1beta1::Scale),
    Created(crate::v1_14::api::apps::v1beta1::Scale),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReplaceNamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedStatefulSetScaleResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedStatefulSetScaleResponse::Other(result), read))
            },
        }
    }
}

// End apps/v1beta1/Scale

impl crate::Resource for Scale {
    fn api_version() -> &'static str {
        "apps/v1beta1"
    }

    fn group() -> &'static str {
        "apps"
    }

    fn kind() -> &'static str {
        "Scale"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for Scale {
    type Ty = crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Scale {
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
            type Value = Scale;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Scale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_14::api::apps::v1beta1::ScaleSpec> = None;
                let mut value_status: Option<crate::v1_14::api::apps::v1beta1::ScaleStatus> = None;

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

                Ok(Scale {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Scale",
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

impl serde::Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scale",
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
