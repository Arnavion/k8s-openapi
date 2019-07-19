// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinition

/// CustomResourceDefinition represents a resource that should be exposed on the API server.  Its name MUST be in the format \<.spec.name\>.\<.spec.group\>.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinition {
    pub metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec describes how the user wants the resources to appear
    pub spec: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec>,

    /// Status indicates the actual state of the CustomResourceDefinition
    pub status: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus>,
}

// Begin apiextensions.k8s.io/v1beta1/CustomResourceDefinition

// Generated from operation createApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// create a CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateCustomResourceDefinitionResponse`]`>` constructor, or [`CreateCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_custom_resource_definition(
        body: &crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        optional: CreateCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateCustomResourceDefinitionResponse>), crate::RequestError> {
        let CreateCustomResourceDefinitionOptional {
            pretty,
        } = optional;
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`CustomResourceDefinition::create_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateCustomResourceDefinitionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::create_custom_resource_definition`]
#[derive(Debug)]
pub enum CreateCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Accepted(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for CreateCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCustomResourceDefinitionResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCustomResourceDefinitionResponse::Accepted(result), buf.len()))
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
                Ok((CreateCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteApiextensionsV1beta1CollectionCustomResourceDefinition

impl CustomResourceDefinition {
    /// delete collection of CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionCustomResourceDefinitionResponse`]`>` constructor, or [`DeleteCollectionCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_custom_resource_definition(
        delete_optional: crate::v1_10::DeleteOptional<'_>,
        list_optional: crate::v1_10::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionCustomResourceDefinitionResponse>), crate::RequestError> {
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&delete_optional).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<DeleteCollectionCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::delete_collection_custom_resource_definition`]
#[derive(Debug)]
pub enum DeleteCollectionCustomResourceDefinitionResponse {
    OkStatus(crate::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for DeleteCollectionCustomResourceDefinitionResponse {
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
                    Ok((DeleteCollectionCustomResourceDefinitionResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionCustomResourceDefinitionResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCollectionCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// delete a CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCustomResourceDefinitionResponse`]`>` constructor, or [`DeleteCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_custom_resource_definition(
        name: &str,
        optional: crate::v1_10::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCustomResourceDefinitionResponse>), crate::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );

        let mut __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<DeleteCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::delete_custom_resource_definition`]
#[derive(Debug)]
pub enum DeleteCustomResourceDefinitionResponse {
    OkStatus(crate::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for DeleteCustomResourceDefinitionResponse {
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
                    Ok((DeleteCustomResourceDefinitionResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCustomResourceDefinitionResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// list or watch objects of kind CustomResourceDefinition
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListCustomResourceDefinitionResponse`]`>` constructor, or [`ListCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_custom_resource_definition(
        optional: crate::v1_10::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListCustomResourceDefinitionResponse>), crate::RequestError> {
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ListCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::list_custom_resource_definition`]
#[derive(Debug)]
pub enum ListCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ListCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCustomResourceDefinitionResponse::Ok(result), buf.len()))
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
                Ok((ListCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// partially update the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchCustomResourceDefinitionResponse`]`>` constructor, or [`PatchCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_custom_resource_definition(
        name: &str,
        body: &crate::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchCustomResourceDefinitionResponse>), crate::RequestError> {
        let PatchCustomResourceDefinitionOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`CustomResourceDefinition::patch_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchCustomResourceDefinitionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::patch_custom_resource_definition`]
#[derive(Debug)]
pub enum PatchCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for PatchCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCustomResourceDefinitionResponse::Ok(result), buf.len()))
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
                Ok((PatchCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// read the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadCustomResourceDefinitionResponse`]`>` constructor, or [`ReadCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_custom_resource_definition(
        name: &str,
        optional: ReadCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadCustomResourceDefinitionResponse>), crate::RequestError> {
        let ReadCustomResourceDefinitionOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?",
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
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CustomResourceDefinition::read_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadCustomResourceDefinitionOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::read_custom_resource_definition`]
#[derive(Debug)]
pub enum ReadCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReadCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCustomResourceDefinitionResponse::Ok(result), buf.len()))
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
                Ok((ReadCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// replace the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceCustomResourceDefinitionResponse`]`>` constructor, or [`ReplaceCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_custom_resource_definition(
        name: &str,
        body: &crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        optional: ReplaceCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceCustomResourceDefinitionResponse>), crate::RequestError> {
        let ReplaceCustomResourceDefinitionOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`CustomResourceDefinition::replace_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceCustomResourceDefinitionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::replace_custom_resource_definition`]
#[derive(Debug)]
pub enum ReplaceCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReplaceCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionResponse::Created(result), buf.len()))
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
                Ok((ReplaceCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceApiextensionsV1beta1CustomResourceDefinitionStatus

impl CustomResourceDefinition {
    /// replace status of the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceCustomResourceDefinitionStatusResponse`]`>` constructor, or [`ReplaceCustomResourceDefinitionStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_custom_resource_definition_status(
        name: &str,
        body: &crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        optional: ReplaceCustomResourceDefinitionStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceCustomResourceDefinitionStatusResponse>), crate::RequestError> {
        let ReplaceCustomResourceDefinitionStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}/status?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`CustomResourceDefinition::replace_custom_resource_definition_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceCustomResourceDefinitionStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceCustomResourceDefinitionStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::replace_custom_resource_definition_status`]
#[derive(Debug)]
pub enum ReplaceCustomResourceDefinitionStatusResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for ReplaceCustomResourceDefinitionStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionStatusResponse::Created(result), buf.len()))
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
                Ok((ReplaceCustomResourceDefinitionStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// list or watch objects of kind CustomResourceDefinition
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchCustomResourceDefinitionResponse`]`>` constructor, or [`WatchCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_custom_resource_definition(
        optional: crate::v1_10::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchCustomResourceDefinitionResponse>), crate::RequestError> {
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<WatchCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::watch_custom_resource_definition`]
#[derive(Debug)]
pub enum WatchCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent<CustomResourceDefinition>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl crate::Response for WatchCustomResourceDefinitionResponse {
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
                Ok((WatchCustomResourceDefinitionResponse::Ok(result), byte_offset))
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
                Ok((WatchCustomResourceDefinitionResponse::Other(result), read))
            },
        }
    }
}

// End apiextensions.k8s.io/v1beta1/CustomResourceDefinition

impl crate::Resource for CustomResourceDefinition {
    fn api_version() -> &'static str {
        "apiextensions.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "apiextensions.k8s.io"
    }

    fn kind() -> &'static str {
        "CustomResourceDefinition"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for CustomResourceDefinition {
    type Ty = crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for CustomResourceDefinition {
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
            type Value = CustomResourceDefinition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct CustomResourceDefinition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec> = None;
                let mut value_status: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus> = None;

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

                Ok(CustomResourceDefinition {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinition",
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

impl serde::Serialize for CustomResourceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinition",
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
