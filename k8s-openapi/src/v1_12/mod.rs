pub mod api;

pub mod apiextensions_apiserver;

pub mod apimachinery;

pub mod kube_aggregator;

// Generated from operation getAPIVersions

/// get available API versions
///
/// Use [`GetAPIVersionsResponse`](./enum.GetAPIVersionsResponse.html) to parse the HTTP response.
pub fn get_api_versions(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_api_versions`](./fn.get_api_versions.html)
#[derive(Debug)]
pub enum GetAPIVersionsResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroupList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAPIVersionsResponse::Unauthorized, 0)),
            _ => Ok((GetAPIVersionsResponse::Other, 0)),
        }
    }
}

// Generated from operation getAdmissionregistrationAPIGroup

/// get information of a group
///
/// Use [`GetAdmissionregistrationAPIGroupResponse`](./enum.GetAdmissionregistrationAPIGroupResponse.html) to parse the HTTP response.
pub fn get_admissionregistration_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/admissionregistration.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_admissionregistration_api_group`](./fn.get_admissionregistration_api_group.html)
#[derive(Debug)]
pub enum GetAdmissionregistrationAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAdmissionregistrationAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetAdmissionregistrationAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getAdmissionregistrationV1alpha1APIResources

/// get available resources
///
/// Use [`GetAdmissionregistrationV1alpha1APIResourcesResponse`](./enum.GetAdmissionregistrationV1alpha1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_admissionregistration_v1alpha1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_admissionregistration_v1alpha1_api_resources`](./fn.get_admissionregistration_v1alpha1_api_resources.html)
#[derive(Debug)]
pub enum GetAdmissionregistrationV1alpha1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAdmissionregistrationV1alpha1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAdmissionregistrationV1alpha1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAdmissionregistrationV1beta1APIResources

/// get available resources
///
/// Use [`GetAdmissionregistrationV1beta1APIResourcesResponse`](./enum.GetAdmissionregistrationV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_admissionregistration_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_admissionregistration_v1beta1_api_resources`](./fn.get_admissionregistration_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetAdmissionregistrationV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAdmissionregistrationV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAdmissionregistrationV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getApiextensionsAPIGroup

/// get information of a group
///
/// Use [`GetApiextensionsAPIGroupResponse`](./enum.GetApiextensionsAPIGroupResponse.html) to parse the HTTP response.
pub fn get_apiextensions_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apiextensions.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apiextensions_api_group`](./fn.get_apiextensions_api_group.html)
#[derive(Debug)]
pub enum GetApiextensionsAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetApiextensionsAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetApiextensionsAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getApiextensionsV1beta1APIResources

/// get available resources
///
/// Use [`GetApiextensionsV1beta1APIResourcesResponse`](./enum.GetApiextensionsV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_apiextensions_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apiextensions.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apiextensions_v1beta1_api_resources`](./fn.get_apiextensions_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetApiextensionsV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetApiextensionsV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetApiextensionsV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getApiregistrationAPIGroup

/// get information of a group
///
/// Use [`GetApiregistrationAPIGroupResponse`](./enum.GetApiregistrationAPIGroupResponse.html) to parse the HTTP response.
pub fn get_apiregistration_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apiregistration.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apiregistration_api_group`](./fn.get_apiregistration_api_group.html)
#[derive(Debug)]
pub enum GetApiregistrationAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetApiregistrationAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetApiregistrationAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getApiregistrationV1APIResources

/// get available resources
///
/// Use [`GetApiregistrationV1APIResourcesResponse`](./enum.GetApiregistrationV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_apiregistration_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apiregistration.k8s.io/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apiregistration_v1_api_resources`](./fn.get_apiregistration_v1_api_resources.html)
#[derive(Debug)]
pub enum GetApiregistrationV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetApiregistrationV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetApiregistrationV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getApiregistrationV1beta1APIResources

/// get available resources
///
/// Use [`GetApiregistrationV1beta1APIResourcesResponse`](./enum.GetApiregistrationV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_apiregistration_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apiregistration.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apiregistration_v1beta1_api_resources`](./fn.get_apiregistration_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetApiregistrationV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetApiregistrationV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetApiregistrationV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAppsAPIGroup

/// get information of a group
///
/// Use [`GetAppsAPIGroupResponse`](./enum.GetAppsAPIGroupResponse.html) to parse the HTTP response.
pub fn get_apps_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apps/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apps_api_group`](./fn.get_apps_api_group.html)
#[derive(Debug)]
pub enum GetAppsAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAppsAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetAppsAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getAppsV1APIResources

/// get available resources
///
/// Use [`GetAppsV1APIResourcesResponse`](./enum.GetAppsV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_apps_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apps/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apps_v1_api_resources`](./fn.get_apps_v1_api_resources.html)
#[derive(Debug)]
pub enum GetAppsV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAppsV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAppsV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAppsV1beta1APIResources

/// get available resources
///
/// Use [`GetAppsV1beta1APIResourcesResponse`](./enum.GetAppsV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_apps_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apps/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apps_v1beta1_api_resources`](./fn.get_apps_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetAppsV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAppsV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAppsV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAppsV1beta2APIResources

/// get available resources
///
/// Use [`GetAppsV1beta2APIResourcesResponse`](./enum.GetAppsV1beta2APIResourcesResponse.html) to parse the HTTP response.
pub fn get_apps_v1beta2_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/apps/v1beta2/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_apps_v1beta2_api_resources`](./fn.get_apps_v1beta2_api_resources.html)
#[derive(Debug)]
pub enum GetAppsV1beta2APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAppsV1beta2APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAppsV1beta2APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAuthenticationAPIGroup

/// get information of a group
///
/// Use [`GetAuthenticationAPIGroupResponse`](./enum.GetAuthenticationAPIGroupResponse.html) to parse the HTTP response.
pub fn get_authentication_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/authentication.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_authentication_api_group`](./fn.get_authentication_api_group.html)
#[derive(Debug)]
pub enum GetAuthenticationAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAuthenticationAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetAuthenticationAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getAuthenticationV1APIResources

/// get available resources
///
/// Use [`GetAuthenticationV1APIResourcesResponse`](./enum.GetAuthenticationV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_authentication_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/authentication.k8s.io/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_authentication_v1_api_resources`](./fn.get_authentication_v1_api_resources.html)
#[derive(Debug)]
pub enum GetAuthenticationV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAuthenticationV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAuthenticationV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAuthenticationV1beta1APIResources

/// get available resources
///
/// Use [`GetAuthenticationV1beta1APIResourcesResponse`](./enum.GetAuthenticationV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_authentication_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/authentication.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_authentication_v1beta1_api_resources`](./fn.get_authentication_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetAuthenticationV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAuthenticationV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAuthenticationV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAuthorizationAPIGroup

/// get information of a group
///
/// Use [`GetAuthorizationAPIGroupResponse`](./enum.GetAuthorizationAPIGroupResponse.html) to parse the HTTP response.
pub fn get_authorization_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/authorization.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_authorization_api_group`](./fn.get_authorization_api_group.html)
#[derive(Debug)]
pub enum GetAuthorizationAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAuthorizationAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetAuthorizationAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getAuthorizationV1APIResources

/// get available resources
///
/// Use [`GetAuthorizationV1APIResourcesResponse`](./enum.GetAuthorizationV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_authorization_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/authorization.k8s.io/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_authorization_v1_api_resources`](./fn.get_authorization_v1_api_resources.html)
#[derive(Debug)]
pub enum GetAuthorizationV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAuthorizationV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAuthorizationV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAuthorizationV1beta1APIResources

/// get available resources
///
/// Use [`GetAuthorizationV1beta1APIResourcesResponse`](./enum.GetAuthorizationV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_authorization_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/authorization.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_authorization_v1beta1_api_resources`](./fn.get_authorization_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetAuthorizationV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAuthorizationV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAuthorizationV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAutoscalingAPIGroup

/// get information of a group
///
/// Use [`GetAutoscalingAPIGroupResponse`](./enum.GetAutoscalingAPIGroupResponse.html) to parse the HTTP response.
pub fn get_autoscaling_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/autoscaling/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_autoscaling_api_group`](./fn.get_autoscaling_api_group.html)
#[derive(Debug)]
pub enum GetAutoscalingAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAutoscalingAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetAutoscalingAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getAutoscalingV1APIResources

/// get available resources
///
/// Use [`GetAutoscalingV1APIResourcesResponse`](./enum.GetAutoscalingV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_autoscaling_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/autoscaling/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_autoscaling_v1_api_resources`](./fn.get_autoscaling_v1_api_resources.html)
#[derive(Debug)]
pub enum GetAutoscalingV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAutoscalingV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAutoscalingV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAutoscalingV2beta1APIResources

/// get available resources
///
/// Use [`GetAutoscalingV2beta1APIResourcesResponse`](./enum.GetAutoscalingV2beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_autoscaling_v2beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/autoscaling/v2beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_autoscaling_v2beta1_api_resources`](./fn.get_autoscaling_v2beta1_api_resources.html)
#[derive(Debug)]
pub enum GetAutoscalingV2beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetAutoscalingV2beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAutoscalingV2beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getAutoscalingV2beta2APIResources

/// get available resources
///
/// Use [`GetAutoscalingV2beta2APIResourcesResponse`](./enum.GetAutoscalingV2beta2APIResourcesResponse.html) to parse the HTTP response.
pub fn get_autoscaling_v2beta2_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/autoscaling/v2beta2/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_autoscaling_v2beta2_api_resources`](./fn.get_autoscaling_v2beta2_api_resources.html)
#[derive(Debug)]
pub enum GetAutoscalingV2beta2APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl crate::Response for GetAutoscalingV2beta2APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetAutoscalingV2beta2APIResourcesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((GetAutoscalingV2beta2APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetAutoscalingV2beta2APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getBatchAPIGroup

/// get information of a group
///
/// Use [`GetBatchAPIGroupResponse`](./enum.GetBatchAPIGroupResponse.html) to parse the HTTP response.
pub fn get_batch_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/batch/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_batch_api_group`](./fn.get_batch_api_group.html)
#[derive(Debug)]
pub enum GetBatchAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetBatchAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetBatchAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getBatchV1APIResources

/// get available resources
///
/// Use [`GetBatchV1APIResourcesResponse`](./enum.GetBatchV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_batch_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/batch/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_batch_v1_api_resources`](./fn.get_batch_v1_api_resources.html)
#[derive(Debug)]
pub enum GetBatchV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetBatchV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetBatchV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getBatchV1beta1APIResources

/// get available resources
///
/// Use [`GetBatchV1beta1APIResourcesResponse`](./enum.GetBatchV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_batch_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/batch/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_batch_v1beta1_api_resources`](./fn.get_batch_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetBatchV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetBatchV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetBatchV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getBatchV2alpha1APIResources

/// get available resources
///
/// Use [`GetBatchV2alpha1APIResourcesResponse`](./enum.GetBatchV2alpha1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_batch_v2alpha1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/batch/v2alpha1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_batch_v2alpha1_api_resources`](./fn.get_batch_v2alpha1_api_resources.html)
#[derive(Debug)]
pub enum GetBatchV2alpha1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetBatchV2alpha1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetBatchV2alpha1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getCertificatesAPIGroup

/// get information of a group
///
/// Use [`GetCertificatesAPIGroupResponse`](./enum.GetCertificatesAPIGroupResponse.html) to parse the HTTP response.
pub fn get_certificates_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/certificates.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_certificates_api_group`](./fn.get_certificates_api_group.html)
#[derive(Debug)]
pub enum GetCertificatesAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetCertificatesAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetCertificatesAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getCertificatesV1beta1APIResources

/// get available resources
///
/// Use [`GetCertificatesV1beta1APIResourcesResponse`](./enum.GetCertificatesV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_certificates_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/certificates.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_certificates_v1beta1_api_resources`](./fn.get_certificates_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetCertificatesV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetCertificatesV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetCertificatesV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getCodeVersion

/// get the code version
///
/// Use [`GetCodeVersionResponse`](./enum.GetCodeVersionResponse.html) to parse the HTTP response.
pub fn get_code_version(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/version/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_code_version`](./fn.get_code_version.html)
#[derive(Debug)]
pub enum GetCodeVersionResponse {
    Ok(crate::v1_12::apimachinery::pkg::version::Info),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetCodeVersionResponse::Unauthorized, 0)),
            _ => Ok((GetCodeVersionResponse::Other, 0)),
        }
    }
}

// Generated from operation getCoordinationAPIGroup

/// get information of a group
///
/// Use [`GetCoordinationAPIGroupResponse`](./enum.GetCoordinationAPIGroupResponse.html) to parse the HTTP response.
pub fn get_coordination_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/coordination.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_coordination_api_group`](./fn.get_coordination_api_group.html)
#[derive(Debug)]
pub enum GetCoordinationAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl crate::Response for GetCoordinationAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCoordinationAPIGroupResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((GetCoordinationAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetCoordinationAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getCoordinationV1beta1APIResources

/// get available resources
///
/// Use [`GetCoordinationV1beta1APIResourcesResponse`](./enum.GetCoordinationV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_coordination_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/coordination.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_coordination_v1beta1_api_resources`](./fn.get_coordination_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetCoordinationV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl crate::Response for GetCoordinationV1beta1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCoordinationV1beta1APIResourcesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((GetCoordinationV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetCoordinationV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getCoreAPIVersions

/// get available API versions
///
/// Use [`GetCoreAPIVersionsResponse`](./enum.GetCoreAPIVersionsResponse.html) to parse the HTTP response.
pub fn get_core_api_versions(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/api/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_core_api_versions`](./fn.get_core_api_versions.html)
#[derive(Debug)]
pub enum GetCoreAPIVersionsResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIVersions),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetCoreAPIVersionsResponse::Unauthorized, 0)),
            _ => Ok((GetCoreAPIVersionsResponse::Other, 0)),
        }
    }
}

// Generated from operation getCoreV1APIResources

/// get available resources
///
/// Use [`GetCoreV1APIResourcesResponse`](./enum.GetCoreV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_core_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/api/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_core_v1_api_resources`](./fn.get_core_v1_api_resources.html)
#[derive(Debug)]
pub enum GetCoreV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetCoreV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetCoreV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getEventsAPIGroup

/// get information of a group
///
/// Use [`GetEventsAPIGroupResponse`](./enum.GetEventsAPIGroupResponse.html) to parse the HTTP response.
pub fn get_events_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/events.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_events_api_group`](./fn.get_events_api_group.html)
#[derive(Debug)]
pub enum GetEventsAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetEventsAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetEventsAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getEventsV1beta1APIResources

/// get available resources
///
/// Use [`GetEventsV1beta1APIResourcesResponse`](./enum.GetEventsV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_events_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/events.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_events_v1beta1_api_resources`](./fn.get_events_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetEventsV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetEventsV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetEventsV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getExtensionsAPIGroup

/// get information of a group
///
/// Use [`GetExtensionsAPIGroupResponse`](./enum.GetExtensionsAPIGroupResponse.html) to parse the HTTP response.
pub fn get_extensions_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/extensions/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_extensions_api_group`](./fn.get_extensions_api_group.html)
#[derive(Debug)]
pub enum GetExtensionsAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetExtensionsAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetExtensionsAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getExtensionsV1beta1APIResources

/// get available resources
///
/// Use [`GetExtensionsV1beta1APIResourcesResponse`](./enum.GetExtensionsV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_extensions_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/extensions/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_extensions_v1beta1_api_resources`](./fn.get_extensions_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetExtensionsV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetExtensionsV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetExtensionsV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getNetworkingAPIGroup

/// get information of a group
///
/// Use [`GetNetworkingAPIGroupResponse`](./enum.GetNetworkingAPIGroupResponse.html) to parse the HTTP response.
pub fn get_networking_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/networking.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_networking_api_group`](./fn.get_networking_api_group.html)
#[derive(Debug)]
pub enum GetNetworkingAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetNetworkingAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetNetworkingAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getNetworkingV1APIResources

/// get available resources
///
/// Use [`GetNetworkingV1APIResourcesResponse`](./enum.GetNetworkingV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_networking_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/networking.k8s.io/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_networking_v1_api_resources`](./fn.get_networking_v1_api_resources.html)
#[derive(Debug)]
pub enum GetNetworkingV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetNetworkingV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetNetworkingV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getPolicyAPIGroup

/// get information of a group
///
/// Use [`GetPolicyAPIGroupResponse`](./enum.GetPolicyAPIGroupResponse.html) to parse the HTTP response.
pub fn get_policy_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/policy/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_policy_api_group`](./fn.get_policy_api_group.html)
#[derive(Debug)]
pub enum GetPolicyAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetPolicyAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetPolicyAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getPolicyV1beta1APIResources

/// get available resources
///
/// Use [`GetPolicyV1beta1APIResourcesResponse`](./enum.GetPolicyV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_policy_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/policy/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_policy_v1beta1_api_resources`](./fn.get_policy_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetPolicyV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetPolicyV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetPolicyV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getRbacAuthorizationAPIGroup

/// get information of a group
///
/// Use [`GetRbacAuthorizationAPIGroupResponse`](./enum.GetRbacAuthorizationAPIGroupResponse.html) to parse the HTTP response.
pub fn get_rbac_authorization_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_rbac_authorization_api_group`](./fn.get_rbac_authorization_api_group.html)
#[derive(Debug)]
pub enum GetRbacAuthorizationAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetRbacAuthorizationAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetRbacAuthorizationAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getRbacAuthorizationV1APIResources

/// get available resources
///
/// Use [`GetRbacAuthorizationV1APIResourcesResponse`](./enum.GetRbacAuthorizationV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_rbac_authorization_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_rbac_authorization_v1_api_resources`](./fn.get_rbac_authorization_v1_api_resources.html)
#[derive(Debug)]
pub enum GetRbacAuthorizationV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetRbacAuthorizationV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetRbacAuthorizationV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getRbacAuthorizationV1alpha1APIResources

/// get available resources
///
/// Use [`GetRbacAuthorizationV1alpha1APIResourcesResponse`](./enum.GetRbacAuthorizationV1alpha1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_rbac_authorization_v1alpha1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_rbac_authorization_v1alpha1_api_resources`](./fn.get_rbac_authorization_v1alpha1_api_resources.html)
#[derive(Debug)]
pub enum GetRbacAuthorizationV1alpha1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetRbacAuthorizationV1alpha1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetRbacAuthorizationV1alpha1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getRbacAuthorizationV1beta1APIResources

/// get available resources
///
/// Use [`GetRbacAuthorizationV1beta1APIResourcesResponse`](./enum.GetRbacAuthorizationV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_rbac_authorization_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_rbac_authorization_v1beta1_api_resources`](./fn.get_rbac_authorization_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetRbacAuthorizationV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetRbacAuthorizationV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetRbacAuthorizationV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getSchedulingAPIGroup

/// get information of a group
///
/// Use [`GetSchedulingAPIGroupResponse`](./enum.GetSchedulingAPIGroupResponse.html) to parse the HTTP response.
pub fn get_scheduling_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/scheduling.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_scheduling_api_group`](./fn.get_scheduling_api_group.html)
#[derive(Debug)]
pub enum GetSchedulingAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetSchedulingAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetSchedulingAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getSchedulingV1alpha1APIResources

/// get available resources
///
/// Use [`GetSchedulingV1alpha1APIResourcesResponse`](./enum.GetSchedulingV1alpha1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_scheduling_v1alpha1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/scheduling.k8s.io/v1alpha1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_scheduling_v1alpha1_api_resources`](./fn.get_scheduling_v1alpha1_api_resources.html)
#[derive(Debug)]
pub enum GetSchedulingV1alpha1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetSchedulingV1alpha1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetSchedulingV1alpha1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getSchedulingV1beta1APIResources

/// get available resources
///
/// Use [`GetSchedulingV1beta1APIResourcesResponse`](./enum.GetSchedulingV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_scheduling_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/scheduling.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_scheduling_v1beta1_api_resources`](./fn.get_scheduling_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetSchedulingV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetSchedulingV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetSchedulingV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getSettingsAPIGroup

/// get information of a group
///
/// Use [`GetSettingsAPIGroupResponse`](./enum.GetSettingsAPIGroupResponse.html) to parse the HTTP response.
pub fn get_settings_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/settings.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_settings_api_group`](./fn.get_settings_api_group.html)
#[derive(Debug)]
pub enum GetSettingsAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetSettingsAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetSettingsAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getSettingsV1alpha1APIResources

/// get available resources
///
/// Use [`GetSettingsV1alpha1APIResourcesResponse`](./enum.GetSettingsV1alpha1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_settings_v1alpha1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/settings.k8s.io/v1alpha1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_settings_v1alpha1_api_resources`](./fn.get_settings_v1alpha1_api_resources.html)
#[derive(Debug)]
pub enum GetSettingsV1alpha1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetSettingsV1alpha1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetSettingsV1alpha1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getStorageAPIGroup

/// get information of a group
///
/// Use [`GetStorageAPIGroupResponse`](./enum.GetStorageAPIGroupResponse.html) to parse the HTTP response.
pub fn get_storage_api_group(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/storage.k8s.io/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_storage_api_group`](./fn.get_storage_api_group.html)
#[derive(Debug)]
pub enum GetStorageAPIGroupResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetStorageAPIGroupResponse::Unauthorized, 0)),
            _ => Ok((GetStorageAPIGroupResponse::Other, 0)),
        }
    }
}

// Generated from operation getStorageV1APIResources

/// get available resources
///
/// Use [`GetStorageV1APIResourcesResponse`](./enum.GetStorageV1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_storage_v1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/storage.k8s.io/v1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_storage_v1_api_resources`](./fn.get_storage_v1_api_resources.html)
#[derive(Debug)]
pub enum GetStorageV1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetStorageV1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetStorageV1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getStorageV1alpha1APIResources

/// get available resources
///
/// Use [`GetStorageV1alpha1APIResourcesResponse`](./enum.GetStorageV1alpha1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_storage_v1alpha1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/storage.k8s.io/v1alpha1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_storage_v1alpha1_api_resources`](./fn.get_storage_v1alpha1_api_resources.html)
#[derive(Debug)]
pub enum GetStorageV1alpha1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetStorageV1alpha1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetStorageV1alpha1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation getStorageV1beta1APIResources

/// get available resources
///
/// Use [`GetStorageV1beta1APIResourcesResponse`](./enum.GetStorageV1beta1APIResourcesResponse.html) to parse the HTTP response.
pub fn get_storage_v1beta1_api_resources(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/apis/storage.k8s.io/v1beta1/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`get_storage_v1beta1_api_resources`](./fn.get_storage_v1beta1_api_resources.html)
#[derive(Debug)]
pub enum GetStorageV1beta1APIResourcesResponse {
    Ok(crate::v1_12::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

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
            http::StatusCode::UNAUTHORIZED => Ok((GetStorageV1beta1APIResourcesResponse::Unauthorized, 0)),
            _ => Ok((GetStorageV1beta1APIResourcesResponse::Other, 0)),
        }
    }
}

// Generated from operation logFileHandler

/// Use [`LogFileHandlerResponse`](./enum.LogFileHandlerResponse.html) to parse the HTTP response.
///
/// # Arguments
///
/// * `logpath`
///
///     path to the log
pub fn log_file_handler(
    logpath: &str,
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/logs/{logpath}", logpath = logpath);

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`log_file_handler`](./fn.log_file_handler.html)
#[derive(Debug)]
pub enum LogFileHandlerResponse {
    Unauthorized,
    Other,
}

impl crate::Response for LogFileHandlerResponse {
    fn try_from_parts(status_code: http::StatusCode, _: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::UNAUTHORIZED => Ok((LogFileHandlerResponse::Unauthorized, 0)),
            _ => Ok((LogFileHandlerResponse::Other, 0)),
        }
    }
}

// Generated from operation logFileListHandler

/// Use [`LogFileListHandlerResponse`](./enum.LogFileListHandlerResponse.html) to parse the HTTP response.
pub fn log_file_list_handler(
) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
    let __url = format!("/logs/");

    let mut __request = http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(crate::RequestError::Http)
}

/// Parses the HTTP response of [`log_file_list_handler`](./fn.log_file_list_handler.html)
#[derive(Debug)]
pub enum LogFileListHandlerResponse {
    Unauthorized,
    Other,
}

impl crate::Response for LogFileListHandlerResponse {
    fn try_from_parts(status_code: http::StatusCode, _: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::UNAUTHORIZED => Ok((LogFileListHandlerResponse::Unauthorized, 0)),
            _ => Ok((LogFileListHandlerResponse::Other, 0)),
        }
    }
}
