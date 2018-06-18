pub mod api;

pub mod apiextensions_apiserver;

pub mod apimachinery;

pub mod kube_aggregator;

// Generated from operation getAPIVersions

#[derive(Debug)]
pub enum GetAPIVersionsResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroupList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available API versions
pub fn get_api_versions<C>(
    __client: &C,
) -> Result<GetAPIVersionsResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAPIVersionsResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAPIVersionsResponse::Unauthorized(response),
        other => GetAPIVersionsResponse::Other(other, response),
    })
}


// Generated from operation getAdmissionregistrationAPIGroup

#[derive(Debug)]
pub enum GetAdmissionregistrationAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_admissionregistration_api_group<C>(
    __client: &C,
) -> Result<GetAdmissionregistrationAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAdmissionregistrationAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAdmissionregistrationAPIGroupResponse::Unauthorized(response),
        other => GetAdmissionregistrationAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getAdmissionregistrationV1alpha1APIResources

#[derive(Debug)]
pub enum GetAdmissionregistrationV1alpha1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_admissionregistration_v1alpha1_api_resources<C>(
    __client: &C,
) -> Result<GetAdmissionregistrationV1alpha1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAdmissionregistrationV1alpha1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAdmissionregistrationV1alpha1APIResourcesResponse::Unauthorized(response),
        other => GetAdmissionregistrationV1alpha1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getApiextensionsAPIGroup

#[derive(Debug)]
pub enum GetApiextensionsAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_apiextensions_api_group<C>(
    __client: &C,
) -> Result<GetApiextensionsAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apiextensions.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetApiextensionsAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetApiextensionsAPIGroupResponse::Unauthorized(response),
        other => GetApiextensionsAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getApiextensionsV1beta1APIResources

#[derive(Debug)]
pub enum GetApiextensionsV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_apiextensions_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetApiextensionsV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apiextensions.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetApiextensionsV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetApiextensionsV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetApiextensionsV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getApiregistrationAPIGroup

#[derive(Debug)]
pub enum GetApiregistrationAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_apiregistration_api_group<C>(
    __client: &C,
) -> Result<GetApiregistrationAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetApiregistrationAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetApiregistrationAPIGroupResponse::Unauthorized(response),
        other => GetApiregistrationAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getApiregistrationV1beta1APIResources

#[derive(Debug)]
pub enum GetApiregistrationV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_apiregistration_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetApiregistrationV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetApiregistrationV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetApiregistrationV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetApiregistrationV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAppsAPIGroup

#[derive(Debug)]
pub enum GetAppsAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_apps_api_group<C>(
    __client: &C,
) -> Result<GetAppsAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apps/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAppsAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAppsAPIGroupResponse::Unauthorized(response),
        other => GetAppsAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getAppsV1beta1APIResources

#[derive(Debug)]
pub enum GetAppsV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_apps_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetAppsV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apps/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAppsV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAppsV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetAppsV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAppsV1beta2APIResources

#[derive(Debug)]
pub enum GetAppsV1beta2APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_apps_v1beta2_api_resources<C>(
    __client: &C,
) -> Result<GetAppsV1beta2APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/apps/v1beta2/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAppsV1beta2APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAppsV1beta2APIResourcesResponse::Unauthorized(response),
        other => GetAppsV1beta2APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAuthenticationAPIGroup

#[derive(Debug)]
pub enum GetAuthenticationAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_authentication_api_group<C>(
    __client: &C,
) -> Result<GetAuthenticationAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/authentication.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAuthenticationAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAuthenticationAPIGroupResponse::Unauthorized(response),
        other => GetAuthenticationAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getAuthenticationV1APIResources

#[derive(Debug)]
pub enum GetAuthenticationV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_authentication_v1_api_resources<C>(
    __client: &C,
) -> Result<GetAuthenticationV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/authentication.k8s.io/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAuthenticationV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAuthenticationV1APIResourcesResponse::Unauthorized(response),
        other => GetAuthenticationV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAuthenticationV1beta1APIResources

#[derive(Debug)]
pub enum GetAuthenticationV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_authentication_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetAuthenticationV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/authentication.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAuthenticationV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAuthenticationV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetAuthenticationV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAuthorizationAPIGroup

#[derive(Debug)]
pub enum GetAuthorizationAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_authorization_api_group<C>(
    __client: &C,
) -> Result<GetAuthorizationAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/authorization.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAuthorizationAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAuthorizationAPIGroupResponse::Unauthorized(response),
        other => GetAuthorizationAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getAuthorizationV1APIResources

#[derive(Debug)]
pub enum GetAuthorizationV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_authorization_v1_api_resources<C>(
    __client: &C,
) -> Result<GetAuthorizationV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/authorization.k8s.io/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAuthorizationV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAuthorizationV1APIResourcesResponse::Unauthorized(response),
        other => GetAuthorizationV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAuthorizationV1beta1APIResources

#[derive(Debug)]
pub enum GetAuthorizationV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_authorization_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetAuthorizationV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/authorization.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAuthorizationV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAuthorizationV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetAuthorizationV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAutoscalingAPIGroup

#[derive(Debug)]
pub enum GetAutoscalingAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_autoscaling_api_group<C>(
    __client: &C,
) -> Result<GetAutoscalingAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/autoscaling/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAutoscalingAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAutoscalingAPIGroupResponse::Unauthorized(response),
        other => GetAutoscalingAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getAutoscalingV1APIResources

#[derive(Debug)]
pub enum GetAutoscalingV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_autoscaling_v1_api_resources<C>(
    __client: &C,
) -> Result<GetAutoscalingV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/autoscaling/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAutoscalingV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAutoscalingV1APIResourcesResponse::Unauthorized(response),
        other => GetAutoscalingV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getAutoscalingV2beta1APIResources

#[derive(Debug)]
pub enum GetAutoscalingV2beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_autoscaling_v2beta1_api_resources<C>(
    __client: &C,
) -> Result<GetAutoscalingV2beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/autoscaling/v2beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetAutoscalingV2beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetAutoscalingV2beta1APIResourcesResponse::Unauthorized(response),
        other => GetAutoscalingV2beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getBatchAPIGroup

#[derive(Debug)]
pub enum GetBatchAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_batch_api_group<C>(
    __client: &C,
) -> Result<GetBatchAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/batch/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetBatchAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetBatchAPIGroupResponse::Unauthorized(response),
        other => GetBatchAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getBatchV1APIResources

#[derive(Debug)]
pub enum GetBatchV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_batch_v1_api_resources<C>(
    __client: &C,
) -> Result<GetBatchV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/batch/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetBatchV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetBatchV1APIResourcesResponse::Unauthorized(response),
        other => GetBatchV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getBatchV1beta1APIResources

#[derive(Debug)]
pub enum GetBatchV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_batch_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetBatchV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/batch/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetBatchV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetBatchV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetBatchV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getBatchV2alpha1APIResources

#[derive(Debug)]
pub enum GetBatchV2alpha1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_batch_v2alpha1_api_resources<C>(
    __client: &C,
) -> Result<GetBatchV2alpha1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/batch/v2alpha1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetBatchV2alpha1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetBatchV2alpha1APIResourcesResponse::Unauthorized(response),
        other => GetBatchV2alpha1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getCertificatesAPIGroup

#[derive(Debug)]
pub enum GetCertificatesAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_certificates_api_group<C>(
    __client: &C,
) -> Result<GetCertificatesAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetCertificatesAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetCertificatesAPIGroupResponse::Unauthorized(response),
        other => GetCertificatesAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getCertificatesV1beta1APIResources

#[derive(Debug)]
pub enum GetCertificatesV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_certificates_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetCertificatesV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetCertificatesV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetCertificatesV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetCertificatesV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getCodeVersion

#[derive(Debug)]
pub enum GetCodeVersionResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::version::Info),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get the code version
pub fn get_code_version<C>(
    __client: &C,
) -> Result<GetCodeVersionResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/version/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetCodeVersionResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetCodeVersionResponse::Unauthorized(response),
        other => GetCodeVersionResponse::Other(other, response),
    })
}


// Generated from operation getCoreAPIVersions

#[derive(Debug)]
pub enum GetCoreAPIVersionsResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIVersions),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available API versions
pub fn get_core_api_versions<C>(
    __client: &C,
) -> Result<GetCoreAPIVersionsResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/api/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetCoreAPIVersionsResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetCoreAPIVersionsResponse::Unauthorized(response),
        other => GetCoreAPIVersionsResponse::Other(other, response),
    })
}


// Generated from operation getCoreV1APIResources

#[derive(Debug)]
pub enum GetCoreV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_core_v1_api_resources<C>(
    __client: &C,
) -> Result<GetCoreV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/api/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetCoreV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetCoreV1APIResourcesResponse::Unauthorized(response),
        other => GetCoreV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getExtensionsAPIGroup

#[derive(Debug)]
pub enum GetExtensionsAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_extensions_api_group<C>(
    __client: &C,
) -> Result<GetExtensionsAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/extensions/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetExtensionsAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetExtensionsAPIGroupResponse::Unauthorized(response),
        other => GetExtensionsAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getExtensionsV1beta1APIResources

#[derive(Debug)]
pub enum GetExtensionsV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_extensions_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetExtensionsV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetExtensionsV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetExtensionsV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetExtensionsV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getNetworkingAPIGroup

#[derive(Debug)]
pub enum GetNetworkingAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_networking_api_group<C>(
    __client: &C,
) -> Result<GetNetworkingAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/networking.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetNetworkingAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetNetworkingAPIGroupResponse::Unauthorized(response),
        other => GetNetworkingAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getNetworkingV1APIResources

#[derive(Debug)]
pub enum GetNetworkingV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_networking_v1_api_resources<C>(
    __client: &C,
) -> Result<GetNetworkingV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/networking.k8s.io/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetNetworkingV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetNetworkingV1APIResourcesResponse::Unauthorized(response),
        other => GetNetworkingV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getPolicyAPIGroup

#[derive(Debug)]
pub enum GetPolicyAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_policy_api_group<C>(
    __client: &C,
) -> Result<GetPolicyAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/policy/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetPolicyAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetPolicyAPIGroupResponse::Unauthorized(response),
        other => GetPolicyAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getPolicyV1beta1APIResources

#[derive(Debug)]
pub enum GetPolicyV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_policy_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetPolicyV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/policy/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetPolicyV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetPolicyV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetPolicyV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getRbacAuthorizationAPIGroup

#[derive(Debug)]
pub enum GetRbacAuthorizationAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_rbac_authorization_api_group<C>(
    __client: &C,
) -> Result<GetRbacAuthorizationAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetRbacAuthorizationAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationAPIGroupResponse::Unauthorized(response),
        other => GetRbacAuthorizationAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getRbacAuthorizationV1APIResources

#[derive(Debug)]
pub enum GetRbacAuthorizationV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_rbac_authorization_v1_api_resources<C>(
    __client: &C,
) -> Result<GetRbacAuthorizationV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetRbacAuthorizationV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationV1APIResourcesResponse::Unauthorized(response),
        other => GetRbacAuthorizationV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getRbacAuthorizationV1alpha1APIResources

#[derive(Debug)]
pub enum GetRbacAuthorizationV1alpha1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_rbac_authorization_v1alpha1_api_resources<C>(
    __client: &C,
) -> Result<GetRbacAuthorizationV1alpha1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetRbacAuthorizationV1alpha1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationV1alpha1APIResourcesResponse::Unauthorized(response),
        other => GetRbacAuthorizationV1alpha1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getRbacAuthorizationV1beta1APIResources

#[derive(Debug)]
pub enum GetRbacAuthorizationV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_rbac_authorization_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetRbacAuthorizationV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetRbacAuthorizationV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetRbacAuthorizationV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getSchedulingAPIGroup

#[derive(Debug)]
pub enum GetSchedulingAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_scheduling_api_group<C>(
    __client: &C,
) -> Result<GetSchedulingAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/scheduling.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetSchedulingAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetSchedulingAPIGroupResponse::Unauthorized(response),
        other => GetSchedulingAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getSchedulingV1alpha1APIResources

#[derive(Debug)]
pub enum GetSchedulingV1alpha1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_scheduling_v1alpha1_api_resources<C>(
    __client: &C,
) -> Result<GetSchedulingV1alpha1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/scheduling.k8s.io/v1alpha1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetSchedulingV1alpha1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetSchedulingV1alpha1APIResourcesResponse::Unauthorized(response),
        other => GetSchedulingV1alpha1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getSettingsAPIGroup

#[derive(Debug)]
pub enum GetSettingsAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_settings_api_group<C>(
    __client: &C,
) -> Result<GetSettingsAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/settings.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetSettingsAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetSettingsAPIGroupResponse::Unauthorized(response),
        other => GetSettingsAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getSettingsV1alpha1APIResources

#[derive(Debug)]
pub enum GetSettingsV1alpha1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_settings_v1alpha1_api_resources<C>(
    __client: &C,
) -> Result<GetSettingsV1alpha1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/settings.k8s.io/v1alpha1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetSettingsV1alpha1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetSettingsV1alpha1APIResourcesResponse::Unauthorized(response),
        other => GetSettingsV1alpha1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getStorageAPIGroup

#[derive(Debug)]
pub enum GetStorageAPIGroupResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get information of a group
pub fn get_storage_api_group<C>(
    __client: &C,
) -> Result<GetStorageAPIGroupResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/storage.k8s.io/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetStorageAPIGroupResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetStorageAPIGroupResponse::Unauthorized(response),
        other => GetStorageAPIGroupResponse::Other(other, response),
    })
}


// Generated from operation getStorageV1APIResources

#[derive(Debug)]
pub enum GetStorageV1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_storage_v1_api_resources<C>(
    __client: &C,
) -> Result<GetStorageV1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/storage.k8s.io/v1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetStorageV1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetStorageV1APIResourcesResponse::Unauthorized(response),
        other => GetStorageV1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation getStorageV1beta1APIResources

#[derive(Debug)]
pub enum GetStorageV1beta1APIResourcesResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

/// get available resources
pub fn get_storage_v1beta1_api_resources<C>(
    __client: &C,
) -> Result<GetStorageV1beta1APIResourcesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/apis/storage.k8s.io/v1beta1/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::OK => {
            let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                GetStorageV1beta1APIResourcesResponse::Ok(result)
            },
        ::http::StatusCode::UNAUTHORIZED => GetStorageV1beta1APIResourcesResponse::Unauthorized(response),
        other => GetStorageV1beta1APIResourcesResponse::Other(other, response),
    })
}


// Generated from operation logFileHandler

#[derive(Debug)]
pub enum LogFileHandlerResponse<R> where R: ::std::io::Read {
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

pub fn log_file_handler<C>(
    __client: &C,
    // path to the log
    logpath: &str,
) -> Result<LogFileHandlerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/logs/{logpath}", logpath = logpath)).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::UNAUTHORIZED => LogFileHandlerResponse::Unauthorized(response),
        other => LogFileHandlerResponse::Other(other, response),
    })
}


// Generated from operation logFileListHandler

#[derive(Debug)]
pub enum LogFileListHandlerResponse<R> where R: ::std::io::Read {
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

pub fn log_file_list_handler<C>(
    __client: &C,
) -> Result<LogFileListHandlerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
    let __url = __client.base_url().join(&format!("/logs/")).map_err(::Error::URL)?;

    let response = __client.get(__url).map_err(::Error::Client)?;

    Ok(match ::Response::status_code(&response) {
        ::http::StatusCode::UNAUTHORIZED => LogFileListHandlerResponse::Unauthorized(response),
        other => LogFileListHandlerResponse::Other(other, response),
    })
}

