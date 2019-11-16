// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration

/// ExternalAdmissionHookConfiguration describes the configuration of initializers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalAdmissionHookConfiguration {
    /// ExternalAdmissionHooks is a list of external admission webhooks and the affected resources and operations.
    pub external_admission_hooks: Option<Vec<crate::api::admissionregistration::v1alpha1::ExternalAdmissionHook>>,

    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}

// Begin admissionregistration.k8s.io/v1alpha1/ExternalAdmissionHookConfiguration

// Generated from operation createAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// create an ExternalAdmissionHookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`CreateExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_external_admission_hook_configuration(
        body: &crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration,
        optional: CreateExternalAdmissionHookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let CreateExternalAdmissionHookConfigurationOptional {
            pretty,
        } = optional;
        let __url = "/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`ExternalAdmissionHookConfiguration::create_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateExternalAdmissionHookConfigurationOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::create_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateExternalAdmissionHookConfigurationResponse {
    Ok(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
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
                Ok((CreateExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// delete collection of ExternalAdmissionHookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`DeleteCollectionExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_external_admission_hook_configuration(
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let __url = "/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?".to_owned();
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

/// Use `<DeleteCollectionExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::delete_collection_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionExternalAdmissionHookConfigurationResponse {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfigurationList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionExternalAdmissionHookConfigurationResponse {
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
                    Ok((DeleteCollectionExternalAdmissionHookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionExternalAdmissionHookConfigurationResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCollectionExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// delete an ExternalAdmissionHookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`DeleteExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_external_admission_hook_configuration(
        name: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<DeleteExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::delete_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteExternalAdmissionHookConfigurationResponse {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteExternalAdmissionHookConfigurationResponse {
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
                    Ok((DeleteExternalAdmissionHookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteExternalAdmissionHookConfigurationResponse::OkValue(result), buf.len()))
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
                Ok((DeleteExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// list or watch objects of kind ExternalAdmissionHookConfiguration
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`ListExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_external_admission_hook_configuration(
        optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let __url = "/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?".to_owned();
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

/// Use `<ListExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::list_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListExternalAdmissionHookConfigurationResponse {
    Ok(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfigurationList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
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
                Ok((ListExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// partially update the specified ExternalAdmissionHookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`PatchExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_external_admission_hook_configuration(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<PatchExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::patch_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchExternalAdmissionHookConfigurationResponse {
    Ok(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
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
                Ok((PatchExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// read the specified ExternalAdmissionHookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`ReadExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_external_admission_hook_configuration(
        name: &str,
        optional: ReadExternalAdmissionHookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let ReadExternalAdmissionHookConfigurationOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`ExternalAdmissionHookConfiguration::read_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadExternalAdmissionHookConfigurationOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::read_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadExternalAdmissionHookConfigurationResponse {
    Ok(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
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
                Ok((ReadExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// replace the specified ExternalAdmissionHookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`ReplaceExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_external_admission_hook_configuration(
        name: &str,
        body: &crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration,
        optional: ReplaceExternalAdmissionHookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let ReplaceExternalAdmissionHookConfigurationOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`ExternalAdmissionHookConfiguration::replace_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceExternalAdmissionHookConfigurationOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::replace_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceExternalAdmissionHookConfigurationResponse {
    Ok(crate::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
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
                Ok((ReplaceExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// list or watch objects of kind ExternalAdmissionHookConfiguration
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchExternalAdmissionHookConfigurationResponse`]`>` constructor, or [`WatchExternalAdmissionHookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_external_admission_hook_configuration(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchExternalAdmissionHookConfigurationResponse>), crate::RequestError> {
        let __url = "/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?".to_owned();
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

/// Use `<WatchExternalAdmissionHookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ExternalAdmissionHookConfiguration::watch_external_admission_hook_configuration`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchExternalAdmissionHookConfigurationResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::WatchEvent<ExternalAdmissionHookConfiguration>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchExternalAdmissionHookConfigurationResponse {
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
                Ok((WatchExternalAdmissionHookConfigurationResponse::Ok(result), byte_offset))
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
                Ok((WatchExternalAdmissionHookConfigurationResponse::Other(result), read))
            },
        }
    }
}

// End admissionregistration.k8s.io/v1alpha1/ExternalAdmissionHookConfiguration

impl crate::Resource for ExternalAdmissionHookConfiguration {
    const API_VERSION: &'static str = "admissionregistration.k8s.io/v1alpha1";
    const GROUP: &'static str = "admissionregistration.k8s.io";
    const KIND: &'static str = "ExternalAdmissionHookConfiguration";
    const VERSION: &'static str = "v1alpha1";
}

impl crate::Metadata for ExternalAdmissionHookConfiguration {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ExternalAdmissionHookConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_external_admission_hooks,
            Key_metadata,
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
                            "externalAdmissionHooks" => Field::Key_external_admission_hooks,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExternalAdmissionHookConfiguration;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ExternalAdmissionHookConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_external_admission_hooks: Option<Vec<crate::api::admissionregistration::v1alpha1::ExternalAdmissionHook>> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_external_admission_hooks => value_external_admission_hooks = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalAdmissionHookConfiguration {
                    external_admission_hooks: value_external_admission_hooks,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalAdmissionHookConfiguration",
            &[
                "apiVersion",
                "kind",
                "externalAdmissionHooks",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ExternalAdmissionHookConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalAdmissionHookConfiguration",
            2 +
            self.external_admission_hooks.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.external_admission_hooks {
            serde::ser::SerializeStruct::serialize_field(&mut state, "externalAdmissionHooks", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
