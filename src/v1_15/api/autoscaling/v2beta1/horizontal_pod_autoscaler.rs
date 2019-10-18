// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscaler

/// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource implementing the scale subresource based on the metrics specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscaler {
    /// metadata is the standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_15::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// spec is the specification for the behaviour of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec>,

    /// status is the current information about the autoscaler.
    pub status: Option<crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus>,
}

// Begin autoscaling/v2beta1/HorizontalPodAutoscaler

// Generated from operation createAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// create a HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`CreateNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
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
    pub fn create_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        body: &crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        optional: CreateNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let CreateNamespacedHorizontalPodAutoscalerOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`HorizontalPodAutoscaler::create_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::create_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Created(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Accepted(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Accepted(result), buf.len()))
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
                Ok((CreateNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteAutoscalingV2beta1CollectionNamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete collection of HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`DeleteCollectionNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        delete_optional: crate::v1_15::DeleteOptional<'_>,
        list_optional: crate::v1_15::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<DeleteCollectionNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::delete_collection_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedHorizontalPodAutoscalerResponse {
    OkStatus(crate::v1_15::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionNamespacedHorizontalPodAutoscalerResponse {
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
                    Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCollectionNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// delete a HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`DeleteNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        optional: crate::v1_15::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<DeleteNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::delete_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteNamespacedHorizontalPodAutoscalerResponse {
    OkStatus(crate::v1_15::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Accepted(crate::v1_15::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteNamespacedHorizontalPodAutoscalerResponse {
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
                    Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::Accepted(result), buf.len()))
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
                Ok((DeleteNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespaces

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListHorizontalPodAutoscalerForAllNamespacesResponse`]`>` constructor, or [`ListHorizontalPodAutoscalerForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_horizontal_pod_autoscaler_for_all_namespaces(
        optional: crate::v1_15::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListHorizontalPodAutoscalerForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/apis/autoscaling/v2beta1/horizontalpodautoscalers?".to_owned();
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

/// Use `<ListHorizontalPodAutoscalerForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::list_horizontal_pod_autoscaler_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListHorizontalPodAutoscalerForAllNamespacesResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListHorizontalPodAutoscalerForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListHorizontalPodAutoscalerForAllNamespacesResponse::Ok(result), buf.len()))
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
                Ok((ListHorizontalPodAutoscalerForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`ListNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
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
    pub fn list_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        optional: crate::v1_15::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<ListNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::list_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
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
                Ok((ListNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// partially update the specified HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`PatchNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn patch_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        body: &crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_15::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::patch_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// partially update status of the specified HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedHorizontalPodAutoscalerStatusResponse`]`>` constructor, or [`PatchNamespacedHorizontalPodAutoscalerStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn patch_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_15::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedHorizontalPodAutoscalerStatusResponse>), crate::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::v1_15::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchNamespacedHorizontalPodAutoscalerStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::patch_namespaced_horizontal_pod_autoscaler_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchNamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
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
                Ok((PatchNamespacedHorizontalPodAutoscalerStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// read the specified HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`ReadNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let ReadNamespacedHorizontalPodAutoscalerOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// read status of the specified HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedHorizontalPodAutoscalerStatusResponse`]`>` constructor, or [`ReadNamespacedHorizontalPodAutoscalerStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedHorizontalPodAutoscalerStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedHorizontalPodAutoscalerStatusResponse>), crate::RequestError> {
        let ReadNamespacedHorizontalPodAutoscalerStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedHorizontalPodAutoscalerStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedHorizontalPodAutoscalerStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::read_namespaced_horizontal_pod_autoscaler_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedHorizontalPodAutoscalerStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// replace the specified HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`ReplaceNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn replace_namespaced_horizontal_pod_autoscaler(
        name: &str,
        namespace: &str,
        body: &crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        optional: ReplaceNamespacedHorizontalPodAutoscalerOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let ReplaceNamespacedHorizontalPodAutoscalerOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedHorizontalPodAutoscalerOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Created(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceNamespacedHorizontalPodAutoscalerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedHorizontalPodAutoscalerResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedHorizontalPodAutoscalerResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAutoscalingV2beta1NamespacedHorizontalPodAutoscalerStatus

impl HorizontalPodAutoscaler {
    /// replace status of the specified HorizontalPodAutoscaler
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedHorizontalPodAutoscalerStatusResponse`]`>` constructor, or [`ReplaceNamespacedHorizontalPodAutoscalerStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the HorizontalPodAutoscaler
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
    pub fn replace_namespaced_horizontal_pod_autoscaler_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler,
        optional: ReplaceNamespacedHorizontalPodAutoscalerStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedHorizontalPodAutoscalerStatusResponse>), crate::RequestError> {
        let ReplaceNamespacedHorizontalPodAutoscalerStatusOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedHorizontalPodAutoscalerStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedHorizontalPodAutoscalerStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::replace_namespaced_horizontal_pod_autoscaler_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNamespacedHorizontalPodAutoscalerStatusResponse {
    Ok(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Created(crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscaler),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceNamespacedHorizontalPodAutoscalerStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedHorizontalPodAutoscalerStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedHorizontalPodAutoscalerStatusResponse::Created(result), buf.len()))
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
                Ok((ReplaceNamespacedHorizontalPodAutoscalerStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchAutoscalingV2beta1HorizontalPodAutoscalerForAllNamespaces

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchHorizontalPodAutoscalerForAllNamespacesResponse`]`>` constructor, or [`WatchHorizontalPodAutoscalerForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_horizontal_pod_autoscaler_for_all_namespaces(
        optional: crate::v1_15::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchHorizontalPodAutoscalerForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/apis/autoscaling/v2beta1/horizontalpodautoscalers?".to_owned();
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

/// Use `<WatchHorizontalPodAutoscalerForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::watch_horizontal_pod_autoscaler_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchHorizontalPodAutoscalerForAllNamespacesResponse {
    Ok(crate::v1_15::apimachinery::pkg::apis::meta::v1::WatchEvent<HorizontalPodAutoscaler>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchHorizontalPodAutoscalerForAllNamespacesResponse {
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
                Ok((WatchHorizontalPodAutoscalerForAllNamespacesResponse::Ok(result), byte_offset))
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
                Ok((WatchHorizontalPodAutoscalerForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchAutoscalingV2beta1NamespacedHorizontalPodAutoscaler

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedHorizontalPodAutoscalerResponse`]`>` constructor, or [`WatchNamespacedHorizontalPodAutoscalerResponse`] directly, to parse the HTTP response.
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
    pub fn watch_namespaced_horizontal_pod_autoscaler(
        namespace: &str,
        optional: crate::v1_15::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedHorizontalPodAutoscalerResponse>), crate::RequestError> {
        let __url = format!("/apis/autoscaling/v2beta1/namespaces/{namespace}/horizontalpodautoscalers?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<WatchNamespacedHorizontalPodAutoscalerResponse as Response>::try_from_parts` to parse the HTTP response body of [`HorizontalPodAutoscaler::watch_namespaced_horizontal_pod_autoscaler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchNamespacedHorizontalPodAutoscalerResponse {
    Ok(crate::v1_15::apimachinery::pkg::apis::meta::v1::WatchEvent<HorizontalPodAutoscaler>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchNamespacedHorizontalPodAutoscalerResponse {
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
                Ok((WatchNamespacedHorizontalPodAutoscalerResponse::Ok(result), byte_offset))
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
                Ok((WatchNamespacedHorizontalPodAutoscalerResponse::Other(result), read))
            },
        }
    }
}

// End autoscaling/v2beta1/HorizontalPodAutoscaler

impl crate::Resource for HorizontalPodAutoscaler {
    fn api_version() -> &'static str {
        "autoscaling/v2beta1"
    }

    fn group() -> &'static str {
        "autoscaling"
    }

    fn kind() -> &'static str {
        "HorizontalPodAutoscaler"
    }

    fn version() -> &'static str {
        "v2beta1"
    }
}

impl crate::Metadata for HorizontalPodAutoscaler {
    type Ty = crate::v1_15::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for HorizontalPodAutoscaler {
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
            type Value = HorizontalPodAutoscaler;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct HorizontalPodAutoscaler")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_15::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec> = None;
                let mut value_status: Option<crate::v1_15::api::autoscaling::v2beta1::HorizontalPodAutoscalerStatus> = None;

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

                Ok(HorizontalPodAutoscaler {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscaler",
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

impl serde::Serialize for HorizontalPodAutoscaler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscaler",
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
