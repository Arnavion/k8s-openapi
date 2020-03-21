
#[cfg(feature = "api")]
mod create_optional;
#[cfg(feature = "api")]
pub use self::create_optional::CreateOptional;

#[cfg(feature = "api")]
mod create_response;
#[cfg(feature = "api")]
pub use self::create_response::CreateResponse;

#[cfg(feature = "api")]
mod delete_optional;
#[cfg(feature = "api")]
pub use self::delete_optional::DeleteOptional;

#[cfg(feature = "api")]
mod delete_response;
#[cfg(feature = "api")]
pub use self::delete_response::DeleteResponse;

mod list;
pub use self::list::List;

#[cfg(feature = "api")]
mod list_optional;
#[cfg(feature = "api")]
pub use self::list_optional::ListOptional;

#[cfg(feature = "api")]
mod list_response;
#[cfg(feature = "api")]
pub use self::list_response::ListResponse;

#[cfg(feature = "api")]
mod patch_optional;
#[cfg(feature = "api")]
pub use self::patch_optional::PatchOptional;

#[cfg(feature = "api")]
mod patch_response;
#[cfg(feature = "api")]
pub use self::patch_response::PatchResponse;

#[cfg(feature = "api")]
mod replace_optional;
#[cfg(feature = "api")]
pub use self::replace_optional::ReplaceOptional;

#[cfg(feature = "api")]
mod replace_response;
#[cfg(feature = "api")]
pub use self::replace_response::ReplaceResponse;

#[cfg(feature = "api")]
mod watch_optional;
#[cfg(feature = "api")]
pub use self::watch_optional::WatchOptional;

#[cfg(feature = "api")]
mod watch_response;
#[cfg(feature = "api")]
pub use self::watch_response::WatchResponse;

pub mod api;

pub mod apiextensions_apiserver;

pub mod apimachinery;

pub mod kube_aggregator;

// Generated from operation getAPIVersions

/// get available API versions
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAPIVersionsResponse`]`>` constructor, or [`GetAPIVersionsResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_api_versions(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAPIVersionsResponse>), crate::RequestError> {
    let __url = "/apis/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAPIVersionsResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_api_versions`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAPIVersionsResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroupList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAPIVersionsResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAPIVersionsResponse::Ok(result), buf.len()))
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
                Ok((GetAPIVersionsResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAdmissionregistrationAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAdmissionregistrationAPIGroupResponse`]`>` constructor, or [`GetAdmissionregistrationAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_admissionregistration_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAdmissionregistrationAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/admissionregistration.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAdmissionregistrationAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_admissionregistration_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAdmissionregistrationAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAdmissionregistrationAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAdmissionregistrationAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAdmissionregistrationAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAdmissionregistrationV1alpha1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAdmissionregistrationV1alpha1APIResourcesResponse`]`>` constructor, or [`GetAdmissionregistrationV1alpha1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_admissionregistration_v1alpha1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAdmissionregistrationV1alpha1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/admissionregistration.k8s.io/v1alpha1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAdmissionregistrationV1alpha1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_admissionregistration_v1alpha1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAdmissionregistrationV1alpha1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAdmissionregistrationV1alpha1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAdmissionregistrationV1alpha1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAdmissionregistrationV1alpha1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAdmissionregistrationV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAdmissionregistrationV1beta1APIResourcesResponse`]`>` constructor, or [`GetAdmissionregistrationV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_admissionregistration_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAdmissionregistrationV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/admissionregistration.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAdmissionregistrationV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_admissionregistration_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAdmissionregistrationV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAdmissionregistrationV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAdmissionregistrationV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAdmissionregistrationV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getApiextensionsAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetApiextensionsAPIGroupResponse`]`>` constructor, or [`GetApiextensionsAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apiextensions_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetApiextensionsAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/apiextensions.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetApiextensionsAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apiextensions_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetApiextensionsAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetApiextensionsAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetApiextensionsAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetApiextensionsAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getApiextensionsV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetApiextensionsV1beta1APIResourcesResponse`]`>` constructor, or [`GetApiextensionsV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apiextensions_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetApiextensionsV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/apiextensions.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetApiextensionsV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apiextensions_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetApiextensionsV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetApiextensionsV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetApiextensionsV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetApiextensionsV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getApiregistrationAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetApiregistrationAPIGroupResponse`]`>` constructor, or [`GetApiregistrationAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apiregistration_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetApiregistrationAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/apiregistration.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetApiregistrationAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apiregistration_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetApiregistrationAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetApiregistrationAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetApiregistrationAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetApiregistrationAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getApiregistrationV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetApiregistrationV1APIResourcesResponse`]`>` constructor, or [`GetApiregistrationV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apiregistration_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetApiregistrationV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/apiregistration.k8s.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetApiregistrationV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apiregistration_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetApiregistrationV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetApiregistrationV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetApiregistrationV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetApiregistrationV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getApiregistrationV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetApiregistrationV1beta1APIResourcesResponse`]`>` constructor, or [`GetApiregistrationV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apiregistration_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetApiregistrationV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/apiregistration.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetApiregistrationV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apiregistration_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetApiregistrationV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetApiregistrationV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetApiregistrationV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetApiregistrationV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAppsAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAppsAPIGroupResponse`]`>` constructor, or [`GetAppsAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apps_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAppsAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/apps/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAppsAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apps_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAppsAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAppsAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAppsAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAppsAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAppsV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAppsV1APIResourcesResponse`]`>` constructor, or [`GetAppsV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apps_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAppsV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/apps/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAppsV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apps_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAppsV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAppsV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAppsV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAppsV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAppsV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAppsV1beta1APIResourcesResponse`]`>` constructor, or [`GetAppsV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apps_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAppsV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/apps/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAppsV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apps_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAppsV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAppsV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAppsV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAppsV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAppsV1beta2APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAppsV1beta2APIResourcesResponse`]`>` constructor, or [`GetAppsV1beta2APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apps_v1beta2_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAppsV1beta2APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/apps/v1beta2/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAppsV1beta2APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apps_v1beta2_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAppsV1beta2APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAppsV1beta2APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAppsV1beta2APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAppsV1beta2APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthenticationAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAuthenticationAPIGroupResponse`]`>` constructor, or [`GetAuthenticationAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authentication_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAuthenticationAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/authentication.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAuthenticationAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authentication_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthenticationAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAuthenticationAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAuthenticationAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAuthenticationAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthenticationV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAuthenticationV1APIResourcesResponse`]`>` constructor, or [`GetAuthenticationV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authentication_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAuthenticationV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/authentication.k8s.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAuthenticationV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authentication_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthenticationV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAuthenticationV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAuthenticationV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAuthenticationV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthenticationV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAuthenticationV1beta1APIResourcesResponse`]`>` constructor, or [`GetAuthenticationV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authentication_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAuthenticationV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/authentication.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAuthenticationV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authentication_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthenticationV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAuthenticationV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAuthenticationV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAuthenticationV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthorizationAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAuthorizationAPIGroupResponse`]`>` constructor, or [`GetAuthorizationAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authorization_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAuthorizationAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/authorization.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAuthorizationAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authorization_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthorizationAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAuthorizationAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAuthorizationAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAuthorizationAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthorizationV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAuthorizationV1APIResourcesResponse`]`>` constructor, or [`GetAuthorizationV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authorization_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAuthorizationV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/authorization.k8s.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAuthorizationV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authorization_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthorizationV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAuthorizationV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAuthorizationV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAuthorizationV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthorizationV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAuthorizationV1beta1APIResourcesResponse`]`>` constructor, or [`GetAuthorizationV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authorization_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAuthorizationV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/authorization.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAuthorizationV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authorization_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthorizationV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAuthorizationV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAuthorizationV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAuthorizationV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAutoscalingAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAutoscalingAPIGroupResponse`]`>` constructor, or [`GetAutoscalingAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_autoscaling_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAutoscalingAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/autoscaling/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAutoscalingAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_autoscaling_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAutoscalingAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAutoscalingAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAutoscalingAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAutoscalingAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAutoscalingV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAutoscalingV1APIResourcesResponse`]`>` constructor, or [`GetAutoscalingV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_autoscaling_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAutoscalingV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/autoscaling/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAutoscalingV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_autoscaling_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAutoscalingV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAutoscalingV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAutoscalingV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAutoscalingV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAutoscalingV2beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetAutoscalingV2beta1APIResourcesResponse`]`>` constructor, or [`GetAutoscalingV2beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_autoscaling_v2beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetAutoscalingV2beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/autoscaling/v2beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetAutoscalingV2beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_autoscaling_v2beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAutoscalingV2beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetAutoscalingV2beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAutoscalingV2beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAutoscalingV2beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getBatchAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetBatchAPIGroupResponse`]`>` constructor, or [`GetBatchAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_batch_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetBatchAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/batch/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetBatchAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_batch_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetBatchAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetBatchAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetBatchAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetBatchAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getBatchV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetBatchV1APIResourcesResponse`]`>` constructor, or [`GetBatchV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_batch_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetBatchV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/batch/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetBatchV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_batch_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetBatchV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetBatchV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetBatchV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetBatchV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getBatchV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetBatchV1beta1APIResourcesResponse`]`>` constructor, or [`GetBatchV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_batch_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetBatchV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/batch/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetBatchV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_batch_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetBatchV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetBatchV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetBatchV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetBatchV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getBatchV2alpha1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetBatchV2alpha1APIResourcesResponse`]`>` constructor, or [`GetBatchV2alpha1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_batch_v2alpha1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetBatchV2alpha1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/batch/v2alpha1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetBatchV2alpha1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_batch_v2alpha1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetBatchV2alpha1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetBatchV2alpha1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetBatchV2alpha1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetBatchV2alpha1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getCertificatesAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetCertificatesAPIGroupResponse`]`>` constructor, or [`GetCertificatesAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_certificates_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetCertificatesAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/certificates.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetCertificatesAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_certificates_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetCertificatesAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetCertificatesAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCertificatesAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetCertificatesAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getCertificatesV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetCertificatesV1beta1APIResourcesResponse`]`>` constructor, or [`GetCertificatesV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_certificates_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetCertificatesV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/certificates.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetCertificatesV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_certificates_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetCertificatesV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetCertificatesV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCertificatesV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetCertificatesV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getCodeVersion

/// get the code version
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetCodeVersionResponse`]`>` constructor, or [`GetCodeVersionResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_code_version(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetCodeVersionResponse>), crate::RequestError> {
    let __url = "/version/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetCodeVersionResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_code_version`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetCodeVersionResponse {
    Ok(crate::apimachinery::pkg::version::Info),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetCodeVersionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCodeVersionResponse::Ok(result), buf.len()))
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
                Ok((GetCodeVersionResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getCoreAPIVersions

/// get available API versions
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetCoreAPIVersionsResponse`]`>` constructor, or [`GetCoreAPIVersionsResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_core_api_versions(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetCoreAPIVersionsResponse>), crate::RequestError> {
    let __url = "/api/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetCoreAPIVersionsResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_core_api_versions`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetCoreAPIVersionsResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIVersions),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetCoreAPIVersionsResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCoreAPIVersionsResponse::Ok(result), buf.len()))
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
                Ok((GetCoreAPIVersionsResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getCoreV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetCoreV1APIResourcesResponse`]`>` constructor, or [`GetCoreV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_core_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetCoreV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/api/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetCoreV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_core_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetCoreV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetCoreV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCoreV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetCoreV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getEventsAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetEventsAPIGroupResponse`]`>` constructor, or [`GetEventsAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_events_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetEventsAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/events.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetEventsAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_events_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetEventsAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetEventsAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetEventsAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetEventsAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getEventsV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetEventsV1beta1APIResourcesResponse`]`>` constructor, or [`GetEventsV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_events_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetEventsV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/events.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetEventsV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_events_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetEventsV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetEventsV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetEventsV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetEventsV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getExtensionsAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetExtensionsAPIGroupResponse`]`>` constructor, or [`GetExtensionsAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_extensions_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetExtensionsAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/extensions/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetExtensionsAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_extensions_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetExtensionsAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetExtensionsAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetExtensionsAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetExtensionsAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getExtensionsV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetExtensionsV1beta1APIResourcesResponse`]`>` constructor, or [`GetExtensionsV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_extensions_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetExtensionsV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/extensions/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetExtensionsV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_extensions_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetExtensionsV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetExtensionsV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetExtensionsV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetExtensionsV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getNetworkingAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetNetworkingAPIGroupResponse`]`>` constructor, or [`GetNetworkingAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_networking_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetNetworkingAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/networking.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetNetworkingAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_networking_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetNetworkingAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetNetworkingAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetNetworkingAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetNetworkingAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getNetworkingV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetNetworkingV1APIResourcesResponse`]`>` constructor, or [`GetNetworkingV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_networking_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetNetworkingV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/networking.k8s.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetNetworkingV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_networking_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetNetworkingV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetNetworkingV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetNetworkingV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetNetworkingV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getPolicyAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetPolicyAPIGroupResponse`]`>` constructor, or [`GetPolicyAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_policy_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetPolicyAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/policy/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetPolicyAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_policy_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetPolicyAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetPolicyAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetPolicyAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetPolicyAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getPolicyV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetPolicyV1beta1APIResourcesResponse`]`>` constructor, or [`GetPolicyV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_policy_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetPolicyV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/policy/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetPolicyV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_policy_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetPolicyV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetPolicyV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetPolicyV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetPolicyV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getRbacAuthorizationAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetRbacAuthorizationAPIGroupResponse`]`>` constructor, or [`GetRbacAuthorizationAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_rbac_authorization_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetRbacAuthorizationAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/rbac.authorization.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetRbacAuthorizationAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_rbac_authorization_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetRbacAuthorizationAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetRbacAuthorizationAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetRbacAuthorizationAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetRbacAuthorizationAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getRbacAuthorizationV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetRbacAuthorizationV1APIResourcesResponse`]`>` constructor, or [`GetRbacAuthorizationV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_rbac_authorization_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetRbacAuthorizationV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/rbac.authorization.k8s.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetRbacAuthorizationV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_rbac_authorization_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetRbacAuthorizationV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetRbacAuthorizationV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetRbacAuthorizationV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetRbacAuthorizationV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getRbacAuthorizationV1alpha1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetRbacAuthorizationV1alpha1APIResourcesResponse`]`>` constructor, or [`GetRbacAuthorizationV1alpha1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_rbac_authorization_v1alpha1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetRbacAuthorizationV1alpha1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/rbac.authorization.k8s.io/v1alpha1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetRbacAuthorizationV1alpha1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_rbac_authorization_v1alpha1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetRbacAuthorizationV1alpha1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetRbacAuthorizationV1alpha1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetRbacAuthorizationV1alpha1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetRbacAuthorizationV1alpha1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getRbacAuthorizationV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetRbacAuthorizationV1beta1APIResourcesResponse`]`>` constructor, or [`GetRbacAuthorizationV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_rbac_authorization_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetRbacAuthorizationV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/rbac.authorization.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetRbacAuthorizationV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_rbac_authorization_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetRbacAuthorizationV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetRbacAuthorizationV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetRbacAuthorizationV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetRbacAuthorizationV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSchedulingAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetSchedulingAPIGroupResponse`]`>` constructor, or [`GetSchedulingAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_scheduling_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetSchedulingAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/scheduling.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetSchedulingAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_scheduling_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSchedulingAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetSchedulingAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetSchedulingAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetSchedulingAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSchedulingV1alpha1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetSchedulingV1alpha1APIResourcesResponse`]`>` constructor, or [`GetSchedulingV1alpha1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_scheduling_v1alpha1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetSchedulingV1alpha1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/scheduling.k8s.io/v1alpha1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetSchedulingV1alpha1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_scheduling_v1alpha1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSchedulingV1alpha1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetSchedulingV1alpha1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetSchedulingV1alpha1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetSchedulingV1alpha1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSchedulingV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetSchedulingV1beta1APIResourcesResponse`]`>` constructor, or [`GetSchedulingV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_scheduling_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetSchedulingV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/scheduling.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetSchedulingV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_scheduling_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSchedulingV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetSchedulingV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetSchedulingV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetSchedulingV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSettingsAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetSettingsAPIGroupResponse`]`>` constructor, or [`GetSettingsAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_settings_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetSettingsAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/settings.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetSettingsAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_settings_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSettingsAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetSettingsAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetSettingsAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetSettingsAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSettingsV1alpha1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetSettingsV1alpha1APIResourcesResponse`]`>` constructor, or [`GetSettingsV1alpha1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_settings_v1alpha1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetSettingsV1alpha1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/settings.k8s.io/v1alpha1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetSettingsV1alpha1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_settings_v1alpha1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSettingsV1alpha1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetSettingsV1alpha1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetSettingsV1alpha1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetSettingsV1alpha1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getStorageAPIGroup

/// get information of a group
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetStorageAPIGroupResponse`]`>` constructor, or [`GetStorageAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_storage_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetStorageAPIGroupResponse>), crate::RequestError> {
    let __url = "/apis/storage.k8s.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetStorageAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_storage_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetStorageAPIGroupResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetStorageAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetStorageAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetStorageAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getStorageV1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetStorageV1APIResourcesResponse`]`>` constructor, or [`GetStorageV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_storage_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetStorageV1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/storage.k8s.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetStorageV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_storage_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetStorageV1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetStorageV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetStorageV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetStorageV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getStorageV1alpha1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetStorageV1alpha1APIResourcesResponse`]`>` constructor, or [`GetStorageV1alpha1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_storage_v1alpha1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetStorageV1alpha1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/storage.k8s.io/v1alpha1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetStorageV1alpha1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_storage_v1alpha1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetStorageV1alpha1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetStorageV1alpha1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetStorageV1alpha1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetStorageV1alpha1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getStorageV1beta1APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetStorageV1beta1APIResourcesResponse`]`>` constructor, or [`GetStorageV1beta1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_storage_v1beta1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<GetStorageV1beta1APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/storage.k8s.io/v1beta1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetStorageV1beta1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_storage_v1beta1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetStorageV1beta1APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetStorageV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetStorageV1beta1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetStorageV1beta1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation logFileHandler

/// Use the returned [`crate::ResponseBody`]`<`[`LogFileHandlerResponse`]`>` constructor, or [`LogFileHandlerResponse`] directly, to parse the HTTP response.
///
/// # Arguments
///
/// * `logpath`
///
///     path to the log
#[cfg(feature = "api")]
pub fn log_file_handler(
    logpath: &str,
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<LogFileHandlerResponse>), crate::RequestError> {
    let __url = format!("/logs/{logpath}",
        logpath = crate::percent_encoding::percent_encode(logpath.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
    );

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<LogFileHandlerResponse as Response>::try_from_parts` to parse the HTTP response body of [`log_file_handler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum LogFileHandlerResponse {
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for LogFileHandlerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
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
                Ok((LogFileHandlerResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation logFileListHandler

/// Use the returned [`crate::ResponseBody`]`<`[`LogFileListHandlerResponse`]`>` constructor, or [`LogFileListHandlerResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn log_file_list_handler(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<LogFileListHandlerResponse>), crate::RequestError> {
    let __url = "/logs/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<LogFileListHandlerResponse as Response>::try_from_parts` to parse the HTTP response body of [`log_file_list_handler`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum LogFileListHandlerResponse {
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for LogFileListHandlerResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
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
                Ok((LogFileListHandlerResponse::Other(result), read))
            },
        }
    }
}
