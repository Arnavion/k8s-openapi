// Generated from definition io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest

/// Describes a certificate signing request
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequest {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The certificate request itself and any additional information.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestSpec>,

    /// Derived information about the request.
    pub status: Option<::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestStatus>,
}

// Begin certificates.k8s.io/v1beta1/CertificateSigningRequest

// Generated from operation createCertificatesV1beta1CertificateSigningRequest

#[derive(Debug)]
pub enum CreateCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// create a CertificateSigningRequest
    pub fn create_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        body: &::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.post(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => CreateCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCertificatesV1beta1CertificateSigningRequest

#[derive(Debug)]
pub enum DeleteCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// delete a CertificateSigningRequest
    pub fn delete_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(grace_period_seconds) = grace_period_seconds {
                __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
            }
            if let Some(orphan_dependents) = orphan_dependents {
                __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(propagation_policy) = propagation_policy {
                __query_pairs.append_pair("propagationPolicy", &propagation_policy);
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => DeleteCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCertificatesV1beta1CollectionCertificateSigningRequest

#[derive(Debug)]
pub enum DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// delete collection of CertificateSigningRequest
    pub fn delete_certificates_v1beta1_collection_certificate_signing_request<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Unauthorized(response),
            other => DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation listCertificatesV1beta1CertificateSigningRequest

#[derive(Debug)]
pub enum ListCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// list or watch objects of kind CertificateSigningRequest
    pub fn list_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => ListCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCertificatesV1beta1CertificateSigningRequest

#[derive(Debug)]
pub enum PatchCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// partially update the specified CertificateSigningRequest
    pub fn patch_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => PatchCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation readCertificatesV1beta1CertificateSigningRequest

#[derive(Debug)]
pub enum ReadCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// read the specified CertificateSigningRequest
    pub fn read_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(exact) = exact {
                __query_pairs.append_pair("exact", &exact.to_string());
            }
            if let Some(export) = export {
                __query_pairs.append_pair("export", &export.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => ReadCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequest

#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// replace the specified CertificateSigningRequest
    pub fn replace_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequestApproval

#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// replace approval of the specified CertificateSigningRequest
    pub fn replace_certificates_v1beta1_certificate_signing_request_approval<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/approval", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Unauthorized(response),
            other => ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequestStatus

#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// replace status of the specified CertificateSigningRequest
    pub fn replace_certificates_v1beta1_certificate_signing_request_status<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/status", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Unauthorized(response),
            other => ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCertificatesV1beta1CertificateSigningRequest

pub enum WatchCertificatesV1beta1CertificateSigningRequestResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// watch changes to an object of kind CertificateSigningRequest
    pub fn watch_certificates_v1beta1_certificate_signing_request<C>(
        __client: &C,
        // name of the CertificateSigningRequest
        name: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCertificatesV1beta1CertificateSigningRequestResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests/{name}", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCertificatesV1beta1CertificateSigningRequestResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized(response),
            other => WatchCertificatesV1beta1CertificateSigningRequestResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCertificatesV1beta1CertificateSigningRequestList

pub enum WatchCertificatesV1beta1CertificateSigningRequestListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl CertificateSigningRequest {
    /// watch individual changes to a list of CertificateSigningRequest
    pub fn watch_certificates_v1beta1_certificate_signing_request_list<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCertificatesV1beta1CertificateSigningRequestListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCertificatesV1beta1CertificateSigningRequestListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCertificatesV1beta1CertificateSigningRequestListResponse::Unauthorized(response),
            other => WatchCertificatesV1beta1CertificateSigningRequestListResponse::Other(other, response),
        })
    }
}

// End certificates.k8s.io/v1beta1/CertificateSigningRequest

impl<'de> ::serde::Deserialize<'de> for CertificateSigningRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequest;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CertificateSigningRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequest {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "CertificateSigningRequest",
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

impl ::serde::Serialize for CertificateSigningRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequest",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
