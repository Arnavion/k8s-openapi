// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.PodSecurityPolicy

/// Pod Security Policy governs the ability to make requests that affect the Security Context that will be applied to a pod and container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicy {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// spec defines the policy enforced.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicySpec>,
}

// Begin extensions/v1beta1/PodSecurityPolicy

// Generated from operation createExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// create a PodSecurityPolicy
    ///
    /// Use [`CreateExtensionsV1beta1PodSecurityPolicyResponse`](./enum.CreateExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn create_extensions_v1beta1_pod_security_policy(
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies?");
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

/// Parses the HTTP response of [`PodSecurityPolicy::create_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.create_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum CreateExtensionsV1beta1PodSecurityPolicyResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy),
    Unauthorized,
    Other,
}

impl ::Response for CreateExtensionsV1beta1PodSecurityPolicyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateExtensionsV1beta1PodSecurityPolicyResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((CreateExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteExtensionsV1beta1CollectionPodSecurityPolicy

impl PodSecurityPolicy {
    /// delete collection of PodSecurityPolicy
    ///
    /// Use [`DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse`](./enum.DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn delete_extensions_v1beta1_collection_pod_security_policy(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PodSecurityPolicy::delete_extensions_v1beta1_collection_pod_security_policy`](./struct.PodSecurityPolicy.html#method.delete_extensions_v1beta1_collection_pod_security_policy)
#[derive(Debug)]
pub enum DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy),
    Unauthorized,
    Other,
}

impl ::Response for DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse {
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
                    Ok((DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((DeleteExtensionsV1beta1CollectionPodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// delete a PodSecurityPolicy
    ///
    /// Use [`DeleteExtensionsV1beta1PodSecurityPolicyResponse`](./enum.DeleteExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn delete_extensions_v1beta1_pod_security_policy(
        // name of the PodSecurityPolicy
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies/{name}?", name = name);
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

/// Parses the HTTP response of [`PodSecurityPolicy::delete_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.delete_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum DeleteExtensionsV1beta1PodSecurityPolicyResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy),
    Unauthorized,
    Other,
}

impl ::Response for DeleteExtensionsV1beta1PodSecurityPolicyResponse {
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
                    Ok((DeleteExtensionsV1beta1PodSecurityPolicyResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1PodSecurityPolicyResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((DeleteExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation listExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// list or watch objects of kind PodSecurityPolicy
    ///
    /// Use [`ListExtensionsV1beta1PodSecurityPolicyResponse`](./enum.ListExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn list_extensions_v1beta1_pod_security_policy(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PodSecurityPolicy::list_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.list_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum ListExtensionsV1beta1PodSecurityPolicyResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicyList),
    Unauthorized,
    Other,
}

impl ::Response for ListExtensionsV1beta1PodSecurityPolicyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListExtensionsV1beta1PodSecurityPolicyResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((ListExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// partially update the specified PodSecurityPolicy
    ///
    /// Use [`PatchExtensionsV1beta1PodSecurityPolicyResponse`](./enum.PatchExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn patch_extensions_v1beta1_pod_security_policy(
        // name of the PodSecurityPolicy
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies/{name}?", name = name);
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

/// Parses the HTTP response of [`PodSecurityPolicy::patch_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.patch_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1PodSecurityPolicyResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy),
    Unauthorized,
    Other,
}

impl ::Response for PatchExtensionsV1beta1PodSecurityPolicyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1PodSecurityPolicyResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// read the specified PodSecurityPolicy
    ///
    /// Use [`ReadExtensionsV1beta1PodSecurityPolicyResponse`](./enum.ReadExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn read_extensions_v1beta1_pod_security_policy(
        // name of the PodSecurityPolicy
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies/{name}?", name = name);
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

/// Parses the HTTP response of [`PodSecurityPolicy::read_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.read_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1PodSecurityPolicyResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy),
    Unauthorized,
    Other,
}

impl ::Response for ReadExtensionsV1beta1PodSecurityPolicyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1PodSecurityPolicyResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// replace the specified PodSecurityPolicy
    ///
    /// Use [`ReplaceExtensionsV1beta1PodSecurityPolicyResponse`](./enum.ReplaceExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn replace_extensions_v1beta1_pod_security_policy(
        // name of the PodSecurityPolicy
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/podsecuritypolicies/{name}?", name = name);
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

/// Parses the HTTP response of [`PodSecurityPolicy::replace_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.replace_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1PodSecurityPolicyResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicy),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceExtensionsV1beta1PodSecurityPolicyResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1PodSecurityPolicyResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1PodSecurityPolicy

impl PodSecurityPolicy {
    /// watch changes to an object of kind PodSecurityPolicy
    ///
    /// Use [`WatchExtensionsV1beta1PodSecurityPolicyResponse`](./enum.WatchExtensionsV1beta1PodSecurityPolicyResponse.html) to parse the HTTP response.
    pub fn watch_extensions_v1beta1_pod_security_policy(
        // name of the PodSecurityPolicy
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/watch/podsecuritypolicies/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PodSecurityPolicy::watch_extensions_v1beta1_pod_security_policy`](./struct.PodSecurityPolicy.html#method.watch_extensions_v1beta1_pod_security_policy)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1PodSecurityPolicyResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchExtensionsV1beta1PodSecurityPolicyResponse {
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
                Ok((WatchExtensionsV1beta1PodSecurityPolicyResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1PodSecurityPolicyResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1PodSecurityPolicyResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1PodSecurityPolicyList

impl PodSecurityPolicy {
    /// watch individual changes to a list of PodSecurityPolicy
    ///
    /// Use [`WatchExtensionsV1beta1PodSecurityPolicyListResponse`](./enum.WatchExtensionsV1beta1PodSecurityPolicyListResponse.html) to parse the HTTP response.
    pub fn watch_extensions_v1beta1_pod_security_policy_list(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/watch/podsecuritypolicies?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`PodSecurityPolicy::watch_extensions_v1beta1_pod_security_policy_list`](./struct.PodSecurityPolicy.html#method.watch_extensions_v1beta1_pod_security_policy_list)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1PodSecurityPolicyListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchExtensionsV1beta1PodSecurityPolicyListResponse {
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
                Ok((WatchExtensionsV1beta1PodSecurityPolicyListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1PodSecurityPolicyListResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1PodSecurityPolicyListResponse::Other, 0)),
        }
    }
}

// End extensions/v1beta1/PodSecurityPolicy

impl<'de> ::serde::Deserialize<'de> for PodSecurityPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodSecurityPolicy;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodSecurityPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::PodSecurityPolicySpec> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicy {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicy",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodSecurityPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicy",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1),
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
        ::serde::ser::SerializeStruct::end(state)
    }
}
