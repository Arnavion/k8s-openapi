// Generated from definition io.k8s.api.rbac.v1alpha1.ClusterRoleBinding

/// ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRoleBinding {
    /// Standard object's metadata.
    pub metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// RoleRef can only reference a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
    pub role_ref: crate::api::rbac::v1alpha1::RoleRef,

    /// Subjects holds references to the objects the role applies to.
    pub subjects: Vec<crate::api::rbac::v1alpha1::Subject>,
}

// Begin rbac.authorization.k8s.io/v1alpha1/ClusterRoleBinding

// Generated from operation createRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// create a ClusterRoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateClusterRoleBindingResponse`]`>` constructor, or [`CreateClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_cluster_role_binding(
        body: &crate::api::rbac::v1alpha1::ClusterRoleBinding,
        optional: CreateClusterRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateClusterRoleBindingResponse>), crate::RequestError> {
        let CreateClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = "/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?".to_owned();
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

/// Optional parameters of [`ClusterRoleBinding::create_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::create_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateClusterRoleBindingResponse {
    Ok(crate::api::rbac::v1alpha1::ClusterRoleBinding),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateClusterRoleBindingResponse::Ok(result), buf.len()))
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
                Ok((CreateClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// delete a ClusterRoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteClusterRoleBindingResponse`]`>` constructor, or [`DeleteClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_cluster_role_binding(
        name: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteClusterRoleBindingResponse>), crate::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}",
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

/// Use `<DeleteClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::delete_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteClusterRoleBindingResponse {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::api::rbac::v1alpha1::ClusterRoleBinding),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteClusterRoleBindingResponse {
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
                    Ok((DeleteClusterRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteClusterRoleBindingResponse::OkValue(result), buf.len()))
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
                Ok((DeleteClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1CollectionClusterRoleBinding

impl ClusterRoleBinding {
    /// delete collection of ClusterRoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionClusterRoleBindingResponse`]`>` constructor, or [`DeleteCollectionClusterRoleBindingResponse`] directly, to parse the HTTP response.
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
    pub fn delete_collection_cluster_role_binding(
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionClusterRoleBindingResponse>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?".to_owned();
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

/// Use `<DeleteCollectionClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::delete_collection_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionClusterRoleBindingResponse {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::api::rbac::v1alpha1::ClusterRoleBindingList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionClusterRoleBindingResponse {
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
                    Ok((DeleteCollectionClusterRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionClusterRoleBindingResponse::OkValue(result), buf.len()))
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
                Ok((DeleteCollectionClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// list or watch objects of kind ClusterRoleBinding
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListClusterRoleBindingResponse`]`>` constructor, or [`ListClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_cluster_role_binding(
        optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListClusterRoleBindingResponse>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?".to_owned();
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

/// Use `<ListClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::list_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListClusterRoleBindingResponse {
    Ok(crate::api::rbac::v1alpha1::ClusterRoleBindingList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListClusterRoleBindingResponse::Ok(result), buf.len()))
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
                Ok((ListClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// partially update the specified ClusterRoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchClusterRoleBindingResponse`]`>` constructor, or [`PatchClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_cluster_role_binding(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchClusterRoleBindingResponse>), crate::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?",
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

/// Use `<PatchClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::patch_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchClusterRoleBindingResponse {
    Ok(crate::api::rbac::v1alpha1::ClusterRoleBinding),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchClusterRoleBindingResponse::Ok(result), buf.len()))
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
                Ok((PatchClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// read the specified ClusterRoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadClusterRoleBindingResponse`]`>` constructor, or [`ReadClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_cluster_role_binding(
        name: &str,
        optional: ReadClusterRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadClusterRoleBindingResponse>), crate::RequestError> {
        let ReadClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`ClusterRoleBinding::read_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::read_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadClusterRoleBindingResponse {
    Ok(crate::api::rbac::v1alpha1::ClusterRoleBinding),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadClusterRoleBindingResponse::Ok(result), buf.len()))
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
                Ok((ReadClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// replace the specified ClusterRoleBinding
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceClusterRoleBindingResponse`]`>` constructor, or [`ReplaceClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_cluster_role_binding(
        name: &str,
        body: &crate::api::rbac::v1alpha1::ClusterRoleBinding,
        optional: ReplaceClusterRoleBindingOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceClusterRoleBindingResponse>), crate::RequestError> {
        let ReplaceClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?",
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

/// Optional parameters of [`ClusterRoleBinding::replace_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::replace_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceClusterRoleBindingResponse {
    Ok(crate::api::rbac::v1alpha1::ClusterRoleBinding),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceClusterRoleBindingResponse::Ok(result), buf.len()))
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
                Ok((ReplaceClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// list or watch objects of kind ClusterRoleBinding
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchClusterRoleBindingResponse`]`>` constructor, or [`WatchClusterRoleBindingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_cluster_role_binding(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchClusterRoleBindingResponse>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?".to_owned();
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

/// Use `<WatchClusterRoleBindingResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRoleBinding::watch_cluster_role_binding`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchClusterRoleBindingResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::WatchEvent<ClusterRoleBinding>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchClusterRoleBindingResponse {
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
                Ok((WatchClusterRoleBindingResponse::Ok(result), byte_offset))
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
                Ok((WatchClusterRoleBindingResponse::Other(result), read))
            },
        }
    }
}

// End rbac.authorization.k8s.io/v1alpha1/ClusterRoleBinding

impl crate::Resource for ClusterRoleBinding {
    const API_VERSION: &'static str = "rbac.authorization.k8s.io/v1alpha1";
    const GROUP: &'static str = "rbac.authorization.k8s.io";
    const KIND: &'static str = "ClusterRoleBinding";
    const VERSION: &'static str = "v1alpha1";
}

impl crate::Metadata for ClusterRoleBinding {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ClusterRoleBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_role_ref,
            Key_subjects,
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
                            "roleRef" => Field::Key_role_ref,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRoleBinding;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ClusterRoleBinding")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_role_ref: Option<crate::api::rbac::v1alpha1::RoleRef> = None;
                let mut value_subjects: Option<Vec<crate::api::rbac::v1alpha1::Subject>> = None;

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
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_role_ref => value_role_ref = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subjects => value_subjects = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRoleBinding {
                    metadata: value_metadata,
                    role_ref: value_role_ref.ok_or_else(|| serde::de::Error::missing_field("roleRef"))?,
                    subjects: value_subjects.ok_or_else(|| serde::de::Error::missing_field("subjects"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterRoleBinding",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "roleRef",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterRoleBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterRoleBinding",
            4 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "roleRef", &self.role_ref)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        serde::ser::SerializeStruct::end(state)
    }
}
