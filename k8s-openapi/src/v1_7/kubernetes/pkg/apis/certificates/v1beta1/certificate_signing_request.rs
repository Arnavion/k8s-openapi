// Generated from definition io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest

/// Describes a certificate signing request
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequest {
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The certificate request itself and any additional information.
    pub spec: Option<crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestSpec>,

    /// Derived information about the request.
    pub status: Option<crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestStatus>,
}

// Begin certificates.k8s.io/v1beta1/CertificateSigningRequest

// Generated from operation createCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// create a CertificateSigningRequest
    ///
    /// Use [`CreateCertificatesV1beta1CertificateSigningRequestResponse`](./enum.CreateCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_certificates_v1beta1_certificate_signing_request(
        body: &crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::create_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.create_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum CreateCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((CreateCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// delete a CertificateSigningRequest
    ///
    /// Use [`DeleteCertificatesV1beta1CertificateSigningRequestResponse`](./enum.DeleteCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `body`
    ///
    /// * `grace_period_seconds`
    ///
    ///     The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    ///
    /// * `orphan_dependents`
    ///
    ///     Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `propagation_policy`
    ///
    ///     Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub fn delete_certificates_v1beta1_certificate_signing_request(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::delete_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.delete_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum DeleteCertificatesV1beta1CertificateSigningRequestResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCertificatesV1beta1CertificateSigningRequestResponse {
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
                    Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((DeleteCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCertificatesV1beta1CollectionCertificateSigningRequest

impl CertificateSigningRequest {
    /// delete collection of CertificateSigningRequest
    ///
    /// Use [`DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse`](./enum.DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn delete_certificates_v1beta1_collection_certificate_signing_request(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::delete_certificates_v1beta1_collection_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.delete_certificates_v1beta1_collection_certificate_signing_request)
#[derive(Debug)]
pub enum DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse {
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
                    Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((DeleteCertificatesV1beta1CollectionCertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation listCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// list or watch objects of kind CertificateSigningRequest
    ///
    /// Use [`ListCertificatesV1beta1CertificateSigningRequestResponse`](./enum.ListCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_certificates_v1beta1_certificate_signing_request(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::list_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.list_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum ListCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((ListCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// partially update the specified CertificateSigningRequest
    ///
    /// Use [`PatchCertificatesV1beta1CertificateSigningRequestResponse`](./enum.PatchCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_certificates_v1beta1_certificate_signing_request(
        name: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::patch_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.patch_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum PatchCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((PatchCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation readCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// read the specified CertificateSigningRequest
    ///
    /// Use [`ReadCertificatesV1beta1CertificateSigningRequestResponse`](./enum.ReadCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `exact`
    ///
    ///     Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    ///
    /// * `export`
    ///
    ///     Should this value be exported.  Export strips fields that a user can not specify.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_certificates_v1beta1_certificate_signing_request(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::read_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.read_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum ReadCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((ReadCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// replace the specified CertificateSigningRequest
    ///
    /// Use [`ReplaceCertificatesV1beta1CertificateSigningRequestResponse`](./enum.ReplaceCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_certificates_v1beta1_certificate_signing_request(
        name: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::replace_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.replace_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCertificatesV1beta1CertificateSigningRequestResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequestApproval

impl CertificateSigningRequest {
    /// replace approval of the specified CertificateSigningRequest
    ///
    /// Use [`ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse`](./enum.ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_certificates_v1beta1_certificate_signing_request_approval(
        name: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/approval?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::replace_certificates_v1beta1_certificate_signing_request_approval`](./struct.CertificateSigningRequest.html#method.replace_certificates_v1beta1_certificate_signing_request_approval)
#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestApprovalResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCertificatesV1beta1CertificateSigningRequestStatus

impl CertificateSigningRequest {
    /// replace status of the specified CertificateSigningRequest
    ///
    /// Use [`ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse`](./enum.ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_certificates_v1beta1_certificate_signing_request_status(
        name: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::replace_certificates_v1beta1_certificate_signing_request_status`](./struct.CertificateSigningRequest.html#method.replace_certificates_v1beta1_certificate_signing_request_status)
#[derive(Debug)]
pub enum ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequest),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCertificatesV1beta1CertificateSigningRequestStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCertificatesV1beta1CertificateSigningRequest

impl CertificateSigningRequest {
    /// watch changes to an object of kind CertificateSigningRequest
    ///
    /// Use [`WatchCertificatesV1beta1CertificateSigningRequestResponse`](./enum.WatchCertificatesV1beta1CertificateSigningRequestResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CertificateSigningRequest
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_certificates_v1beta1_certificate_signing_request(
        name: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::watch_certificates_v1beta1_certificate_signing_request`](./struct.CertificateSigningRequest.html#method.watch_certificates_v1beta1_certificate_signing_request)
#[derive(Debug)]
pub enum WatchCertificatesV1beta1CertificateSigningRequestResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCertificatesV1beta1CertificateSigningRequestResponse {
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
                Ok((WatchCertificatesV1beta1CertificateSigningRequestResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCertificatesV1beta1CertificateSigningRequestResponse::Unauthorized, 0)),
            _ => Ok((WatchCertificatesV1beta1CertificateSigningRequestResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCertificatesV1beta1CertificateSigningRequestList

impl CertificateSigningRequest {
    /// watch individual changes to a list of CertificateSigningRequest
    ///
    /// Use [`WatchCertificatesV1beta1CertificateSigningRequestListResponse`](./enum.WatchCertificatesV1beta1CertificateSigningRequestListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_certificates_v1beta1_certificate_signing_request_list(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CertificateSigningRequest::watch_certificates_v1beta1_certificate_signing_request_list`](./struct.CertificateSigningRequest.html#method.watch_certificates_v1beta1_certificate_signing_request_list)
#[derive(Debug)]
pub enum WatchCertificatesV1beta1CertificateSigningRequestListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCertificatesV1beta1CertificateSigningRequestListResponse {
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
                Ok((WatchCertificatesV1beta1CertificateSigningRequestListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCertificatesV1beta1CertificateSigningRequestListResponse::Unauthorized, 0)),
            _ => Ok((WatchCertificatesV1beta1CertificateSigningRequestListResponse::Other, 0)),
        }
    }
}

// End certificates.k8s.io/v1beta1/CertificateSigningRequest

impl crate::Resource for CertificateSigningRequest {
    fn api_version() -> &'static str {
        "certificates.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "certificates.k8s.io"
    }

    fn kind() -> &'static str {
        "CertificateSigningRequest"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl<'de> serde::Deserialize<'de> for CertificateSigningRequest {
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

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            type Value = CertificateSigningRequest;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct CertificateSigningRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestSpec> = None;
                let mut value_status: Option<crate::v1_7::kubernetes::pkg::apis::certificates::v1beta1::CertificateSigningRequestStatus> = None;

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

                Ok(CertificateSigningRequest {
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

impl serde::Serialize for CertificateSigningRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequest",
            0 +
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
