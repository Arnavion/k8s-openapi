// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.ThirdPartyResource

/// A ThirdPartyResource is a generic representation of a resource, it is used by add-ons and plugins to add new resource types to the API.  It consists of one or more Versions of the api.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThirdPartyResource {
    /// Description is the description of this object.
    pub description: Option<String>,

    /// Standard object metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Versions are versions for this third party object
    pub versions: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::APIVersion>>,
}

// Begin extensions/v1beta1/ThirdPartyResource

// Generated from operation createExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// create a ThirdPartyResource
    ///
    /// Use [`CreateExtensionsV1beta1ThirdPartyResourceResponse`](./enum.CreateExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_extensions_v1beta1_third_party_resource(
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::create_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.create_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum CreateExtensionsV1beta1ThirdPartyResourceResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized,
    Other,
}

impl ::Response for CreateExtensionsV1beta1ThirdPartyResourceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateExtensionsV1beta1ThirdPartyResourceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((CreateExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteExtensionsV1beta1CollectionThirdPartyResource

impl ThirdPartyResource {
    /// delete collection of ThirdPartyResource
    ///
    /// Use [`DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse`](./enum.DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse.html) to parse the HTTP response.
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
    pub fn delete_extensions_v1beta1_collection_third_party_resource(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::delete_extensions_v1beta1_collection_third_party_resource`](./struct.ThirdPartyResource.html#method.delete_extensions_v1beta1_collection_third_party_resource)
#[derive(Debug)]
pub enum DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized,
    Other,
}

impl ::Response for DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse {
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
                    Ok((DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// delete a ThirdPartyResource
    ///
    /// Use [`DeleteExtensionsV1beta1ThirdPartyResourceResponse`](./enum.DeleteExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ThirdPartyResource
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
    pub fn delete_extensions_v1beta1_third_party_resource(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::delete_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.delete_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum DeleteExtensionsV1beta1ThirdPartyResourceResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized,
    Other,
}

impl ::Response for DeleteExtensionsV1beta1ThirdPartyResourceResponse {
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
                    Ok((DeleteExtensionsV1beta1ThirdPartyResourceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1ThirdPartyResourceResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((DeleteExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation listExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// list or watch objects of kind ThirdPartyResource
    ///
    /// Use [`ListExtensionsV1beta1ThirdPartyResourceResponse`](./enum.ListExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
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
    pub fn list_extensions_v1beta1_third_party_resource(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::list_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.list_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum ListExtensionsV1beta1ThirdPartyResourceResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResourceList),
    Unauthorized,
    Other,
}

impl ::Response for ListExtensionsV1beta1ThirdPartyResourceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListExtensionsV1beta1ThirdPartyResourceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((ListExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// partially update the specified ThirdPartyResource
    ///
    /// Use [`PatchExtensionsV1beta1ThirdPartyResourceResponse`](./enum.PatchExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ThirdPartyResource
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_extensions_v1beta1_third_party_resource(
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::patch_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.patch_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1ThirdPartyResourceResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized,
    Other,
}

impl ::Response for PatchExtensionsV1beta1ThirdPartyResourceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1ThirdPartyResourceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// read the specified ThirdPartyResource
    ///
    /// Use [`ReadExtensionsV1beta1ThirdPartyResourceResponse`](./enum.ReadExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ThirdPartyResource
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
    pub fn read_extensions_v1beta1_third_party_resource(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::read_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.read_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1ThirdPartyResourceResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized,
    Other,
}

impl ::Response for ReadExtensionsV1beta1ThirdPartyResourceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1ThirdPartyResourceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// replace the specified ThirdPartyResource
    ///
    /// Use [`ReplaceExtensionsV1beta1ThirdPartyResourceResponse`](./enum.ReplaceExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ThirdPartyResource
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_extensions_v1beta1_third_party_resource(
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/thirdpartyresources/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::replace_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.replace_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1ThirdPartyResourceResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceExtensionsV1beta1ThirdPartyResourceResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1ThirdPartyResourceResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1ThirdPartyResource

impl ThirdPartyResource {
    /// watch changes to an object of kind ThirdPartyResource
    ///
    /// Use [`WatchExtensionsV1beta1ThirdPartyResourceResponse`](./enum.WatchExtensionsV1beta1ThirdPartyResourceResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ThirdPartyResource
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
    pub fn watch_extensions_v1beta1_third_party_resource(
        name: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/watch/thirdpartyresources/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::watch_extensions_v1beta1_third_party_resource`](./struct.ThirdPartyResource.html#method.watch_extensions_v1beta1_third_party_resource)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1ThirdPartyResourceResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchExtensionsV1beta1ThirdPartyResourceResponse {
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
                Ok((WatchExtensionsV1beta1ThirdPartyResourceResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1ThirdPartyResourceResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1ThirdPartyResourceList

impl ThirdPartyResource {
    /// watch individual changes to a list of ThirdPartyResource
    ///
    /// Use [`WatchExtensionsV1beta1ThirdPartyResourceListResponse`](./enum.WatchExtensionsV1beta1ThirdPartyResourceListResponse.html) to parse the HTTP response.
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
    pub fn watch_extensions_v1beta1_third_party_resource_list(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/watch/thirdpartyresources?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ThirdPartyResource::watch_extensions_v1beta1_third_party_resource_list`](./struct.ThirdPartyResource.html#method.watch_extensions_v1beta1_third_party_resource_list)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1ThirdPartyResourceListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchExtensionsV1beta1ThirdPartyResourceListResponse {
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
                Ok((WatchExtensionsV1beta1ThirdPartyResourceListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1ThirdPartyResourceListResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1ThirdPartyResourceListResponse::Other, 0)),
        }
    }
}

// End extensions/v1beta1/ThirdPartyResource

impl ::Resource for ThirdPartyResource {
    fn api_version() -> &'static str {
        "extensions/v1beta1"
    }

    fn group() -> &'static str {
        "extensions"
    }

    fn kind() -> &'static str {
        "ThirdPartyResource"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl<'de> ::serde::Deserialize<'de> for ThirdPartyResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_description,
            Key_metadata,
            Key_versions,
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
                            "description" => Field::Key_description,
                            "metadata" => Field::Key_metadata,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThirdPartyResource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ThirdPartyResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_description: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_versions: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::APIVersion>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as ::Resource>::api_version() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as ::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as ::Resource>::kind() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_kind), &<Self::Value as ::Resource>::kind()));
                            }
                        },
                        Field::Key_description => value_description = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ThirdPartyResource {
                    description: value_description,
                    metadata: value_metadata,
                    versions: value_versions,
                })
            }
        }

        deserializer.deserialize_struct(
            "ThirdPartyResource",
            &[
                "apiVersion",
                "kind",
                "description",
                "metadata",
                "versions",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ThirdPartyResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ThirdPartyResource",
            0 +
            2 +
            self.description.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.versions.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
        if let Some(value) = &self.description {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.versions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
