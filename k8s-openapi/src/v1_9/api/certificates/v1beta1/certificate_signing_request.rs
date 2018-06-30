// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequest

/// Describes a certificate signing request
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequest {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The certificate request itself and any additional information.
    pub spec: Option<::v1_9::api::certificates::v1beta1::CertificateSigningRequestSpec>,

    /// Derived information about the request.
    pub status: Option<::v1_9::api::certificates::v1beta1::CertificateSigningRequestStatus>,
}

// Begin certificates.k8s.io/v1beta1/CertificateSigningRequest

// Generated from operation createCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// create a CertificateSigningRequest
    ///
    /// Use [`CreateCertificatesV1beta1CertificateSigningRequestResponse`](./enum.CreateCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn create_certificates_v1beta1_certificate_signing_request(
        body: &::v1_9::api::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::create_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.create_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum CreateCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Created(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Accepted(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for CreateCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::ACCEPTED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Accepted(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// delete a CertificateSigningRequest
    ///
    /// Use [`DeleteCertificatesV1beta1CertificateSigningRequestResponse`](./enum.DeleteCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn delete_certificates_v1beta1_certificate_signing_request(
        // name of the CertificateSigningRequest
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::delete_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.delete_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum DeleteCertificatesV1beta1CertificateSigningRequestResponse {
    OkStatus(::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for DeleteCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result: ::serde_json::Map<String, ::serde_json::Value> = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(::serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCertificatesV1beta1CollectionCertificateSigningRequest

impl CertificateSigningRequest {
    /// delete collection of CertificateSigningRequest
    ///
    /// Use [`DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse`](./enum.DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn delete_certificates_v1beta1_collection_certificate_signing_request(
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::delete_certificates_v1beta1_collection_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.delete_certificates_v1beta1_collection_certificate_signing_request)
#[derive(Debug)]
pub enum DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse {
    OkStatus(::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result: ::serde_json::Map<String, ::serde_json::Value> = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(::serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation listCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// list or watch objects of kind CertificateSigningRequest
    ///
    /// Use [`ListCertificatesV1beta1CertificateSigningRequestResponse`](./enum.ListCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn list_certificates_v1beta1_certificate_signing_request(
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::list_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.list_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum ListCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequestList),
    Unauthorized,
    Other,
}

impl ::Response for ListCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((ListCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// partially update the specified CertificateSigningRequest
    ///
    /// Use [`PatchCertificatesV1beta1CertificateSigningRequestResponse`](./enum.PatchCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn patch_certificates_v1beta1_certificate_signing_request(
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::patch_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.patch_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum PatchCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for PatchCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((PatchCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation readCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// read the specified CertificateSigningRequest
    ///
    /// Use [`ReadCertificatesV1beta1CertificateSigningRequestResponse`](./enum.ReadCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn read_certificates_v1beta1_certificate_signing_request(
        // name of the CertificateSigningRequest
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::read_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.read_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum ReadCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for ReadCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((ReadCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// replace the specified CertificateSigningRequest
    ///
    /// Use [`ReplaceCertificatesV1beta1CertificateSigningRequestResponse`](./enum.ReplaceCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn replace_certificates_v1beta1_certificate_signing_request(
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_9::api::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::replace_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.replace_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Created(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequestApproval

impl CertificateSigningRequest {
    /// replace approval of the specified CertificateSigningRequest
    ///
    /// Use [`ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse`](./enum.ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse.html) to parse the HTTP response.
    pub fn replace_certificates_v1beta1_certificate_signing_request_approval(
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_9::api::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/approval?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::replace_certificates_v1beta1_certificate_signing_request_approval`](./struct.CertificateSigningRequest.html#method.replace_certificates_v1beta1_certificate_signing_request_approval)
#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Created(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequestStatus

impl CertificateSigningRequest {
    /// replace status of the specified CertificateSigningRequest
    ///
    /// Use [`ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse`](./enum.ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse.html) to parse the HTTP response.
    pub fn replace_certificates_v1beta1_certificate_signing_request_status(
        // name of the CertificateSigningRequest
        name: &str,
        body: &::v1_9::api::certificates::v1beta1::CertificateSigningRequest,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/status?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::replace_certificates_v1beta1_certificate_signing_request_status`](./struct.CertificateSigningRequest.html#method.replace_certificates_v1beta1_certificate_signing_request_status)
#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse {
    Ok(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Created(::v1_9::api::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// watch changes to an object of kind CertificateSigningRequest
    ///
    /// Use [`WatchCertificatesV1beta1CertificateSigningRequestResponse`](./enum.WatchCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    pub fn watch_certificates_v1beta1_certificate_signing_request(
        // name of the CertificateSigningRequest
        name: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::watch_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.watch_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum WatchCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                Ok((WatchCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((WatchCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCertificatesV1beta1CertificateSigningRequestList

impl CertificateSigningRequest {
    /// watch individual changes to a list of CertificateSigningRequest
    ///
    /// Use [`WatchCertificatesV1beta1CertificateSigningRequestListResponse`](./enum.WatchCertificatesV1beta1CertificateSigningRequestListResponse.html) to parse the HTTP response.
    pub fn watch_certificates_v1beta1_certificate_signing_request_list(
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::watch_certificates_v1beta1_certificate_signing_request_list`](./struct.CertificateSigningRequest.html#method.watch_certificates_v1beta1_certificate_signing_request_list)
#[derive(Debug)]
pub enum WatchCertificatesV1beta1CertificateSigningRequestListResponse {
    Ok(::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCertificatesV1beta1CertificateSigningRequestListResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                Ok((WatchCertificatesV1beta1CertificateSigningRequestListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCertificatesV1beta1CertificateSigningRequestListResponse::Unauthorized, 0)),
            _ => Ok((WatchCertificatesV1beta1CertificateSigningRequestListResponse::Other, 0)),
        }
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
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_9::api::certificates::v1beta1::CertificateSigningRequestSpec> = None;
                let mut value_status: Option<::v1_9::api::certificates::v1beta1::CertificateSigningRequestStatus> = None;

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
