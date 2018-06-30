// Generated from definition io.k8s.kubernetes.pkg.api.v1.LimitRange

/// LimitRange sets resource usage limits for each kind of resource in a Namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitRange {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the limits enforced. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_7::kubernetes::pkg::api::v1::LimitRangeSpec>,
}

// Begin /v1/LimitRange

// Generated from operation createCoreV1NamespacedLimitRange

impl LimitRange {
    /// create a LimitRange
    ///
    /// Use [`CreateCoreV1NamespacedLimitRangeResponse`](./enum.CreateCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn create_core_v1_namespaced_limit_range(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::LimitRange,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges?", namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::create_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.create_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum CreateCoreV1NamespacedLimitRangeResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::LimitRange),
    Unauthorized,
    Other,
}

impl ::Response for CreateCoreV1NamespacedLimitRangeResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateCoreV1NamespacedLimitRangeResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((CreateCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedLimitRange

impl LimitRange {
    /// delete collection of LimitRange
    ///
    /// Use [`DeleteCoreV1CollectionNamespacedLimitRangeResponse`](./enum.DeleteCoreV1CollectionNamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn delete_core_v1_collection_namespaced_limit_range(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
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
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges?", namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::delete_core_v1_collection_namespaced_limit_range`](./struct.LimitRange.html#method.delete_core_v1_collection_namespaced_limit_range)
#[derive(Debug)]
pub enum DeleteCoreV1CollectionNamespacedLimitRangeResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::api::v1::LimitRange),
    Unauthorized,
    Other,
}

impl ::Response for DeleteCoreV1CollectionNamespacedLimitRangeResponse {
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
                    Ok((DeleteCoreV1CollectionNamespacedLimitRangeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCoreV1CollectionNamespacedLimitRangeResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1CollectionNamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1CollectionNamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedLimitRange

impl LimitRange {
    /// delete a LimitRange
    ///
    /// Use [`DeleteCoreV1NamespacedLimitRangeResponse`](./enum.DeleteCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn delete_core_v1_namespaced_limit_range(
        // name of the LimitRange
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::delete_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.delete_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum DeleteCoreV1NamespacedLimitRangeResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::api::v1::LimitRange),
    Unauthorized,
    Other,
}

impl ::Response for DeleteCoreV1NamespacedLimitRangeResponse {
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
                    Ok((DeleteCoreV1NamespacedLimitRangeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteCoreV1NamespacedLimitRangeResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1LimitRangeForAllNamespaces

impl LimitRange {
    /// list or watch objects of kind LimitRange
    ///
    /// Use [`ListCoreV1LimitRangeForAllNamespacesResponse`](./enum.ListCoreV1LimitRangeForAllNamespacesResponse.html) to parse the HTTP response.
    pub fn list_core_v1_limit_range_for_all_namespaces(
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
        let __url = format!("/api/v1/limitranges?");
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

/// Parses the HTTP response of [`LimitRange::list_core_v1_limit_range_for_all_namespaces`](./struct.LimitRange.html#method.list_core_v1_limit_range_for_all_namespaces)
#[derive(Debug)]
pub enum ListCoreV1LimitRangeForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::LimitRangeList),
    Unauthorized,
    Other,
}

impl ::Response for ListCoreV1LimitRangeForAllNamespacesResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListCoreV1LimitRangeForAllNamespacesResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1LimitRangeForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1LimitRangeForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1NamespacedLimitRange

impl LimitRange {
    /// list or watch objects of kind LimitRange
    ///
    /// Use [`ListCoreV1NamespacedLimitRangeResponse`](./enum.ListCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn list_core_v1_namespaced_limit_range(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
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
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges?", namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::list_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.list_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum ListCoreV1NamespacedLimitRangeResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::LimitRangeList),
    Unauthorized,
    Other,
}

impl ::Response for ListCoreV1NamespacedLimitRangeResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListCoreV1NamespacedLimitRangeResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedLimitRange

impl LimitRange {
    /// partially update the specified LimitRange
    ///
    /// Use [`PatchCoreV1NamespacedLimitRangeResponse`](./enum.PatchCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn patch_core_v1_namespaced_limit_range(
        // name of the LimitRange
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::patch_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.patch_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedLimitRangeResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::LimitRange),
    Unauthorized,
    Other,
}

impl ::Response for PatchCoreV1NamespacedLimitRangeResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedLimitRangeResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedLimitRange

impl LimitRange {
    /// read the specified LimitRange
    ///
    /// Use [`ReadCoreV1NamespacedLimitRangeResponse`](./enum.ReadCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn read_core_v1_namespaced_limit_range(
        // name of the LimitRange
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::read_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.read_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedLimitRangeResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::LimitRange),
    Unauthorized,
    Other,
}

impl ::Response for ReadCoreV1NamespacedLimitRangeResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedLimitRangeResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedLimitRange

impl LimitRange {
    /// replace the specified LimitRange
    ///
    /// Use [`ReplaceCoreV1NamespacedLimitRangeResponse`](./enum.ReplaceCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn replace_core_v1_namespaced_limit_range(
        // name of the LimitRange
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::LimitRange,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/limitranges/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::replace_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.replace_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedLimitRangeResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::LimitRange),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceCoreV1NamespacedLimitRangeResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedLimitRangeResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1LimitRangeListForAllNamespaces

impl LimitRange {
    /// watch individual changes to a list of LimitRange
    ///
    /// Use [`WatchCoreV1LimitRangeListForAllNamespacesResponse`](./enum.WatchCoreV1LimitRangeListForAllNamespacesResponse.html) to parse the HTTP response.
    pub fn watch_core_v1_limit_range_list_for_all_namespaces(
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
        let __url = format!("/api/v1/watch/limitranges?");
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

/// Parses the HTTP response of [`LimitRange::watch_core_v1_limit_range_list_for_all_namespaces`](./struct.LimitRange.html#method.watch_core_v1_limit_range_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchCoreV1LimitRangeListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCoreV1LimitRangeListForAllNamespacesResponse {
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
                Ok((WatchCoreV1LimitRangeListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1LimitRangeListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1LimitRangeListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedLimitRange

impl LimitRange {
    /// watch changes to an object of kind LimitRange
    ///
    /// Use [`WatchCoreV1NamespacedLimitRangeResponse`](./enum.WatchCoreV1NamespacedLimitRangeResponse.html) to parse the HTTP response.
    pub fn watch_core_v1_namespaced_limit_range(
        // name of the LimitRange
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
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
        let __url = format!("/api/v1/watch/namespaces/{namespace}/limitranges/{name}?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::watch_core_v1_namespaced_limit_range`](./struct.LimitRange.html#method.watch_core_v1_namespaced_limit_range)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedLimitRangeResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCoreV1NamespacedLimitRangeResponse {
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
                Ok((WatchCoreV1NamespacedLimitRangeResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedLimitRangeResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedLimitRangeResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedLimitRangeList

impl LimitRange {
    /// watch individual changes to a list of LimitRange
    ///
    /// Use [`WatchCoreV1NamespacedLimitRangeListResponse`](./enum.WatchCoreV1NamespacedLimitRangeListResponse.html) to parse the HTTP response.
    pub fn watch_core_v1_namespaced_limit_range_list(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
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
        let __url = format!("/api/v1/watch/namespaces/{namespace}/limitranges?", namespace = namespace);
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

/// Parses the HTTP response of [`LimitRange::watch_core_v1_namespaced_limit_range_list`](./struct.LimitRange.html#method.watch_core_v1_namespaced_limit_range_list)
#[derive(Debug)]
pub enum WatchCoreV1NamespacedLimitRangeListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchCoreV1NamespacedLimitRangeListResponse {
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
                Ok((WatchCoreV1NamespacedLimitRangeListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchCoreV1NamespacedLimitRangeListResponse::Unauthorized, 0)),
            _ => Ok((WatchCoreV1NamespacedLimitRangeListResponse::Other, 0)),
        }
    }
}

// End /v1/LimitRange

impl<'de> ::serde::Deserialize<'de> for LimitRange {
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
            type Value = LimitRange;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct LimitRange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::api::v1::LimitRangeSpec> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitRange {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                })
            }
        }

        deserializer.deserialize_struct(
            "LimitRange",
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

impl ::serde::Serialize for LimitRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitRange",
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
