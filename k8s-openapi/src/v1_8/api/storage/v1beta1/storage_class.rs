// Generated from definition io.k8s.api.storage.v1beta1.StorageClass

/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageClass {
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    pub allow_volume_expansion: Option<bool>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. \["ro", "soft"\]. Not validated - mount of the PVs will simply fail if one is invalid.
    pub mount_options: Option<Vec<String>>,

    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    pub parameters: Option<std::collections::BTreeMap<String, String>>,

    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.
    pub reclaim_policy: Option<String>,
}

// Begin storage.k8s.io/v1beta1/StorageClass

// Generated from operation createStorageV1beta1StorageClass

impl StorageClass {
    /// create a StorageClass
    ///
    /// Use [`CreateStorageV1beta1StorageClassResponse`](./enum.CreateStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_storage_v1beta1_storage_class(
        body: &crate::v1_8::api::storage::v1beta1::StorageClass,
        optional: CreateStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateStorageV1beta1StorageClassOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses?");
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

/// Optional parameters of [`StorageClass::create_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.create_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct CreateStorageV1beta1StorageClassOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::create_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.create_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum CreateStorageV1beta1StorageClassResponse {
    Ok(crate::v1_8::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for CreateStorageV1beta1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateStorageV1beta1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((CreateStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteStorageV1beta1CollectionStorageClass

impl StorageClass {
    /// delete collection of StorageClass
    ///
    /// Use [`DeleteStorageV1beta1CollectionStorageClassResponse`](./enum.DeleteStorageV1beta1CollectionStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_storage_v1beta1_collection_storage_class(
        optional: DeleteStorageV1beta1CollectionStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteStorageV1beta1CollectionStorageClassOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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

/// Optional parameters of [`StorageClass::delete_storage_v1beta1_collection_storage_class`](./struct.StorageClass.html#method.delete_storage_v1beta1_collection_storage_class)
#[derive(Debug, Default)]
pub struct DeleteStorageV1beta1CollectionStorageClassOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::delete_storage_v1beta1_collection_storage_class`](./struct.StorageClass.html#method.delete_storage_v1beta1_collection_storage_class)
#[derive(Debug)]
pub enum DeleteStorageV1beta1CollectionStorageClassResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteStorageV1beta1CollectionStorageClassResponse {
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
                    Ok((DeleteStorageV1beta1CollectionStorageClassResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteStorageV1beta1CollectionStorageClassResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteStorageV1beta1CollectionStorageClassResponse::Unauthorized, 0)),
            _ => Ok((DeleteStorageV1beta1CollectionStorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteStorageV1beta1StorageClass

impl StorageClass {
    /// delete a StorageClass
    ///
    /// Use [`DeleteStorageV1beta1StorageClassResponse`](./enum.DeleteStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_storage_v1beta1_storage_class(
        name: &str,
        optional: DeleteStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteStorageV1beta1StorageClassOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::delete_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.delete_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct DeleteStorageV1beta1StorageClassOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::delete_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.delete_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum DeleteStorageV1beta1StorageClassResponse {
    OkStatus(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_8::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteStorageV1beta1StorageClassResponse {
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
                    Ok((DeleteStorageV1beta1StorageClassResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteStorageV1beta1StorageClassResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((DeleteStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation listStorageV1beta1StorageClass

impl StorageClass {
    /// list or watch objects of kind StorageClass
    ///
    /// Use [`ListStorageV1beta1StorageClassResponse`](./enum.ListStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_storage_v1beta1_storage_class(
        optional: ListStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListStorageV1beta1StorageClassOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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

/// Optional parameters of [`StorageClass::list_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.list_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct ListStorageV1beta1StorageClassOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::list_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.list_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum ListStorageV1beta1StorageClassResponse {
    Ok(crate::v1_8::api::storage::v1beta1::StorageClassList),
    Unauthorized,
    Other,
}

impl crate::Response for ListStorageV1beta1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListStorageV1beta1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((ListStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation patchStorageV1beta1StorageClass

impl StorageClass {
    /// partially update the specified StorageClass
    ///
    /// Use [`PatchStorageV1beta1StorageClassResponse`](./enum.PatchStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_storage_v1beta1_storage_class(
        name: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchStorageV1beta1StorageClassOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::patch_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.patch_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct PatchStorageV1beta1StorageClassOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::patch_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.patch_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum PatchStorageV1beta1StorageClassResponse {
    Ok(crate::v1_8::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for PatchStorageV1beta1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchStorageV1beta1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((PatchStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation readStorageV1beta1StorageClass

impl StorageClass {
    /// read the specified StorageClass
    ///
    /// Use [`ReadStorageV1beta1StorageClassResponse`](./enum.ReadStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_storage_v1beta1_storage_class(
        name: &str,
        optional: ReadStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadStorageV1beta1StorageClassOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::read_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.read_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct ReadStorageV1beta1StorageClassOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::read_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.read_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum ReadStorageV1beta1StorageClassResponse {
    Ok(crate::v1_8::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for ReadStorageV1beta1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadStorageV1beta1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((ReadStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceStorageV1beta1StorageClass

impl StorageClass {
    /// replace the specified StorageClass
    ///
    /// Use [`ReplaceStorageV1beta1StorageClassResponse`](./enum.ReplaceStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_storage_v1beta1_storage_class(
        name: &str,
        body: &crate::v1_8::api::storage::v1beta1::StorageClass,
        optional: ReplaceStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceStorageV1beta1StorageClassOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::replace_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.replace_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct ReplaceStorageV1beta1StorageClassOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::replace_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.replace_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum ReplaceStorageV1beta1StorageClassResponse {
    Ok(crate::v1_8::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceStorageV1beta1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceStorageV1beta1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((ReplaceStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation watchStorageV1beta1StorageClass

impl StorageClass {
    /// watch changes to an object of kind StorageClass
    ///
    /// Use [`WatchStorageV1beta1StorageClassResponse`](./enum.WatchStorageV1beta1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_storage_v1beta1_storage_class(
        name: &str,
        optional: WatchStorageV1beta1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchStorageV1beta1StorageClassOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/watch/storageclasses/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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

/// Optional parameters of [`StorageClass::watch_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.watch_storage_v1beta1_storage_class)
#[derive(Debug, Default)]
pub struct WatchStorageV1beta1StorageClassOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::watch_storage_v1beta1_storage_class`](./struct.StorageClass.html#method.watch_storage_v1beta1_storage_class)
#[derive(Debug)]
pub enum WatchStorageV1beta1StorageClassResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchStorageV1beta1StorageClassResponse {
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
                Ok((WatchStorageV1beta1StorageClassResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchStorageV1beta1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((WatchStorageV1beta1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation watchStorageV1beta1StorageClassList

impl StorageClass {
    /// watch individual changes to a list of StorageClass
    ///
    /// Use [`WatchStorageV1beta1StorageClassListResponse`](./enum.WatchStorageV1beta1StorageClassListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_storage_v1beta1_storage_class_list(
        optional: WatchStorageV1beta1StorageClassListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchStorageV1beta1StorageClassListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/watch/storageclasses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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

/// Optional parameters of [`StorageClass::watch_storage_v1beta1_storage_class_list`](./struct.StorageClass.html#method.watch_storage_v1beta1_storage_class_list)
#[derive(Debug, Default)]
pub struct WatchStorageV1beta1StorageClassListOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::watch_storage_v1beta1_storage_class_list`](./struct.StorageClass.html#method.watch_storage_v1beta1_storage_class_list)
#[derive(Debug)]
pub enum WatchStorageV1beta1StorageClassListResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchStorageV1beta1StorageClassListResponse {
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
                Ok((WatchStorageV1beta1StorageClassListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchStorageV1beta1StorageClassListResponse::Unauthorized, 0)),
            _ => Ok((WatchStorageV1beta1StorageClassListResponse::Other, 0)),
        }
    }
}

// End storage.k8s.io/v1beta1/StorageClass

impl crate::Resource for StorageClass {
    fn api_version() -> &'static str {
        "storage.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "storage.k8s.io"
    }

    fn kind() -> &'static str {
        "StorageClass"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for StorageClass {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for StorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_allow_volume_expansion,
            Key_metadata,
            Key_mount_options,
            Key_parameters,
            Key_provisioner,
            Key_reclaim_policy,
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
                            "allowVolumeExpansion" => Field::Key_allow_volume_expansion,
                            "metadata" => Field::Key_metadata,
                            "mountOptions" => Field::Key_mount_options,
                            "parameters" => Field::Key_parameters,
                            "provisioner" => Field::Key_provisioner,
                            "reclaimPolicy" => Field::Key_reclaim_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageClass;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct StorageClass")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allow_volume_expansion: Option<bool> = None;
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_parameters: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_provisioner: Option<String> = None;
                let mut value_reclaim_policy: Option<String> = None;

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
                        Field::Key_allow_volume_expansion => value_allow_volume_expansion = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provisioner => value_provisioner = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reclaim_policy => value_reclaim_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageClass {
                    allow_volume_expansion: value_allow_volume_expansion,
                    metadata: value_metadata,
                    mount_options: value_mount_options,
                    parameters: value_parameters,
                    provisioner: value_provisioner.ok_or_else(|| serde::de::Error::missing_field("provisioner"))?,
                    reclaim_policy: value_reclaim_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageClass",
            &[
                "apiVersion",
                "kind",
                "allowVolumeExpansion",
                "metadata",
                "mountOptions",
                "parameters",
                "provisioner",
                "reclaimPolicy",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for StorageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageClass",
            0 +
            2 +
            self.allow_volume_expansion.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.mount_options.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1) +
            1 +
            self.reclaim_policy.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.allow_volume_expansion {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowVolumeExpansion", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.mount_options {
            serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
        }
        if let Some(value) = &self.parameters {
            serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "provisioner", &self.provisioner)?;
        if let Some(value) = &self.reclaim_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reclaimPolicy", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
