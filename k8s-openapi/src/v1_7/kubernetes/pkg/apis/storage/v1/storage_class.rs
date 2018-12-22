// Generated from definition io.k8s.kubernetes.pkg.apis.storage.v1.StorageClass

/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageClass {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    pub parameters: Option<std::collections::BTreeMap<String, String>>,

    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,
}

// Begin storage.k8s.io/v1/StorageClass

// Generated from operation createStorageV1StorageClass

impl StorageClass {
    /// create a StorageClass
    ///
    /// Use [`CreateStorageV1StorageClassResponse`](./enum.CreateStorageV1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_storage_v1_storage_class(
        body: &crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass,
        optional: CreateStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateStorageV1StorageClassOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses?");
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

/// Optional parameters of [`StorageClass::create_storage_v1_storage_class`](./struct.StorageClass.html#method.create_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct CreateStorageV1StorageClassOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::create_storage_v1_storage_class`](./struct.StorageClass.html#method.create_storage_v1_storage_class)
#[derive(Debug)]
pub enum CreateStorageV1StorageClassResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for CreateStorageV1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateStorageV1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((CreateStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteStorageV1CollectionStorageClass

impl StorageClass {
    /// delete collection of StorageClass
    ///
    /// Use [`DeleteStorageV1CollectionStorageClassResponse`](./enum.DeleteStorageV1CollectionStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_storage_v1_collection_storage_class(
        optional: DeleteStorageV1CollectionStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteStorageV1CollectionStorageClassOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses?");
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

/// Optional parameters of [`StorageClass::delete_storage_v1_collection_storage_class`](./struct.StorageClass.html#method.delete_storage_v1_collection_storage_class)
#[derive(Debug, Default)]
pub struct DeleteStorageV1CollectionStorageClassOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::delete_storage_v1_collection_storage_class`](./struct.StorageClass.html#method.delete_storage_v1_collection_storage_class)
#[derive(Debug)]
pub enum DeleteStorageV1CollectionStorageClassResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteStorageV1CollectionStorageClassResponse {
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
                    Ok((DeleteStorageV1CollectionStorageClassResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteStorageV1CollectionStorageClassResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteStorageV1CollectionStorageClassResponse::Unauthorized, 0)),
            _ => Ok((DeleteStorageV1CollectionStorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteStorageV1StorageClass

impl StorageClass {
    /// delete a StorageClass
    ///
    /// Use [`DeleteStorageV1StorageClassResponse`](./enum.DeleteStorageV1StorageClassResponse.html) to parse the HTTP response.
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
    pub fn delete_storage_v1_storage_class(
        name: &str,
        optional: DeleteStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteStorageV1StorageClassOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::delete_storage_v1_storage_class`](./struct.StorageClass.html#method.delete_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct DeleteStorageV1StorageClassOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::delete_storage_v1_storage_class`](./struct.StorageClass.html#method.delete_storage_v1_storage_class)
#[derive(Debug)]
pub enum DeleteStorageV1StorageClassResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteStorageV1StorageClassResponse {
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
                    Ok((DeleteStorageV1StorageClassResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteStorageV1StorageClassResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((DeleteStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation listStorageV1StorageClass

impl StorageClass {
    /// list or watch objects of kind StorageClass
    ///
    /// Use [`ListStorageV1StorageClassResponse`](./enum.ListStorageV1StorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_storage_v1_storage_class(
        optional: ListStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListStorageV1StorageClassOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses?");
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

/// Optional parameters of [`StorageClass::list_storage_v1_storage_class`](./struct.StorageClass.html#method.list_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct ListStorageV1StorageClassOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::list_storage_v1_storage_class`](./struct.StorageClass.html#method.list_storage_v1_storage_class)
#[derive(Debug)]
pub enum ListStorageV1StorageClassResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClassList),
    Unauthorized,
    Other,
}

impl crate::Response for ListStorageV1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListStorageV1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((ListStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation patchStorageV1StorageClass

impl StorageClass {
    /// partially update the specified StorageClass
    ///
    /// Use [`PatchStorageV1StorageClassResponse`](./enum.PatchStorageV1StorageClassResponse.html) to parse the HTTP response.
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
    pub fn patch_storage_v1_storage_class(
        name: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchStorageV1StorageClassOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::patch_storage_v1_storage_class`](./struct.StorageClass.html#method.patch_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct PatchStorageV1StorageClassOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::patch_storage_v1_storage_class`](./struct.StorageClass.html#method.patch_storage_v1_storage_class)
#[derive(Debug)]
pub enum PatchStorageV1StorageClassResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for PatchStorageV1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchStorageV1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((PatchStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation readStorageV1StorageClass

impl StorageClass {
    /// read the specified StorageClass
    ///
    /// Use [`ReadStorageV1StorageClassResponse`](./enum.ReadStorageV1StorageClassResponse.html) to parse the HTTP response.
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
    pub fn read_storage_v1_storage_class(
        name: &str,
        optional: ReadStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadStorageV1StorageClassOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::read_storage_v1_storage_class`](./struct.StorageClass.html#method.read_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct ReadStorageV1StorageClassOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::read_storage_v1_storage_class`](./struct.StorageClass.html#method.read_storage_v1_storage_class)
#[derive(Debug)]
pub enum ReadStorageV1StorageClassResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for ReadStorageV1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadStorageV1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((ReadStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceStorageV1StorageClass

impl StorageClass {
    /// replace the specified StorageClass
    ///
    /// Use [`ReplaceStorageV1StorageClassResponse`](./enum.ReplaceStorageV1StorageClassResponse.html) to parse the HTTP response.
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
    pub fn replace_storage_v1_storage_class(
        name: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass,
        optional: ReplaceStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceStorageV1StorageClassOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::replace_storage_v1_storage_class`](./struct.StorageClass.html#method.replace_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct ReplaceStorageV1StorageClassOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`StorageClass::replace_storage_v1_storage_class`](./struct.StorageClass.html#method.replace_storage_v1_storage_class)
#[derive(Debug)]
pub enum ReplaceStorageV1StorageClassResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::storage::v1::StorageClass),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceStorageV1StorageClassResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceStorageV1StorageClassResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((ReplaceStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation watchStorageV1StorageClass

impl StorageClass {
    /// watch changes to an object of kind StorageClass
    ///
    /// Use [`WatchStorageV1StorageClassResponse`](./enum.WatchStorageV1StorageClassResponse.html) to parse the HTTP response.
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
    pub fn watch_storage_v1_storage_class(
        name: &str,
        optional: WatchStorageV1StorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchStorageV1StorageClassOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/watch/storageclasses/{name}?", name = name);
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

/// Optional parameters of [`StorageClass::watch_storage_v1_storage_class`](./struct.StorageClass.html#method.watch_storage_v1_storage_class)
#[derive(Debug, Default)]
pub struct WatchStorageV1StorageClassOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::watch_storage_v1_storage_class`](./struct.StorageClass.html#method.watch_storage_v1_storage_class)
#[derive(Debug)]
pub enum WatchStorageV1StorageClassResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchStorageV1StorageClassResponse {
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
                Ok((WatchStorageV1StorageClassResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchStorageV1StorageClassResponse::Unauthorized, 0)),
            _ => Ok((WatchStorageV1StorageClassResponse::Other, 0)),
        }
    }
}

// Generated from operation watchStorageV1StorageClassList

impl StorageClass {
    /// watch individual changes to a list of StorageClass
    ///
    /// Use [`WatchStorageV1StorageClassListResponse`](./enum.WatchStorageV1StorageClassListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_storage_v1_storage_class_list(
        optional: WatchStorageV1StorageClassListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchStorageV1StorageClassListOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/watch/storageclasses?");
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

/// Optional parameters of [`StorageClass::watch_storage_v1_storage_class_list`](./struct.StorageClass.html#method.watch_storage_v1_storage_class_list)
#[derive(Debug, Default)]
pub struct WatchStorageV1StorageClassListOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`StorageClass::watch_storage_v1_storage_class_list`](./struct.StorageClass.html#method.watch_storage_v1_storage_class_list)
#[derive(Debug)]
pub enum WatchStorageV1StorageClassListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchStorageV1StorageClassListResponse {
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
                Ok((WatchStorageV1StorageClassListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchStorageV1StorageClassListResponse::Unauthorized, 0)),
            _ => Ok((WatchStorageV1StorageClassListResponse::Other, 0)),
        }
    }
}

// End storage.k8s.io/v1/StorageClass

impl crate::Resource for StorageClass {
    fn api_version() -> &'static str {
        "storage.k8s.io/v1"
    }

    fn group() -> &'static str {
        "storage.k8s.io"
    }

    fn kind() -> &'static str {
        "StorageClass"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for StorageClass {
    type Ty = crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
            Key_metadata,
            Key_parameters,
            Key_provisioner,
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
                            "metadata" => Field::Key_metadata,
                            "parameters" => Field::Key_parameters,
                            "provisioner" => Field::Key_provisioner,
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
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_parameters: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_provisioner: Option<String> = None;

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
                        Field::Key_parameters => value_parameters = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provisioner => value_provisioner = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageClass {
                    metadata: value_metadata,
                    parameters: value_parameters,
                    provisioner: value_provisioner.ok_or_else(|| serde::de::Error::missing_field("provisioner"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageClass",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "parameters",
                "provisioner",
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
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.parameters {
            serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "provisioner", &self.provisioner)?;
        serde::ser::SerializeStruct::end(state)
    }
}
