// Generated from definition io.k8s.kubernetes.pkg.apis.batch.v2alpha1.CronJob

/// CronJob represents the configuration of a single cron job.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CronJob {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of a cron job, including the schedule. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobSpec>,

    /// Current status of a cron job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobStatus>,
}

// Begin batch/v2alpha1/CronJob

// Generated from operation createBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// create a CronJob
    ///
    /// Use [`CreateBatchV2alpha1NamespacedCronJobResponse`](./enum.CreateBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_batch_v2alpha1_namespaced_cron_job(
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob,
        optional: CreateBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateBatchV2alpha1NamespacedCronJobOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::create_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.create_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct CreateBatchV2alpha1NamespacedCronJobOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::create_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.create_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum CreateBatchV2alpha1NamespacedCronJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for CreateBatchV2alpha1NamespacedCronJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateBatchV2alpha1NamespacedCronJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((CreateBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteBatchV2alpha1CollectionNamespacedCronJob

impl CronJob {
    /// delete collection of CronJob
    ///
    /// Use [`DeleteBatchV2alpha1CollectionNamespacedCronJobResponse`](./enum.DeleteBatchV2alpha1CollectionNamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_batch_v2alpha1_collection_namespaced_cron_job(
        namespace: &str,
        optional: DeleteBatchV2alpha1CollectionNamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteBatchV2alpha1CollectionNamespacedCronJobOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::delete_batch_v2alpha1_collection_namespaced_cron_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_collection_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct DeleteBatchV2alpha1CollectionNamespacedCronJobOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::delete_batch_v2alpha1_collection_namespaced_cron_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_collection_namespaced_cron_job)
#[derive(Debug)]
pub enum DeleteBatchV2alpha1CollectionNamespacedCronJobResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteBatchV2alpha1CollectionNamespacedCronJobResponse {
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
                    Ok((DeleteBatchV2alpha1CollectionNamespacedCronJobResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteBatchV2alpha1CollectionNamespacedCronJobResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteBatchV2alpha1CollectionNamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((DeleteBatchV2alpha1CollectionNamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// delete a CronJob
    ///
    /// Use [`DeleteBatchV2alpha1NamespacedCronJobResponse`](./enum.DeleteBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_batch_v2alpha1_namespaced_cron_job(
        name: &str,
        namespace: &str,
        optional: DeleteBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteBatchV2alpha1NamespacedCronJobOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::delete_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct DeleteBatchV2alpha1NamespacedCronJobOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::delete_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum DeleteBatchV2alpha1NamespacedCronJobResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteBatchV2alpha1NamespacedCronJobResponse {
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
                    Ok((DeleteBatchV2alpha1NamespacedCronJobResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteBatchV2alpha1NamespacedCronJobResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((DeleteBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation listBatchV2alpha1CronJobForAllNamespaces

impl CronJob {
    /// list or watch objects of kind CronJob
    ///
    /// Use [`ListBatchV2alpha1CronJobForAllNamespacesResponse`](./enum.ListBatchV2alpha1CronJobForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_batch_v2alpha1_cron_job_for_all_namespaces(
        optional: ListBatchV2alpha1CronJobForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListBatchV2alpha1CronJobForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/cronjobs?");
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

/// Optional parameters of [`CronJob::list_batch_v2alpha1_cron_job_for_all_namespaces`](./struct.CronJob.html#method.list_batch_v2alpha1_cron_job_for_all_namespaces)
#[derive(Debug, Default)]
pub struct ListBatchV2alpha1CronJobForAllNamespacesOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::list_batch_v2alpha1_cron_job_for_all_namespaces`](./struct.CronJob.html#method.list_batch_v2alpha1_cron_job_for_all_namespaces)
#[derive(Debug)]
pub enum ListBatchV2alpha1CronJobForAllNamespacesResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobList),
    Unauthorized,
    Other,
}

impl crate::Response for ListBatchV2alpha1CronJobForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListBatchV2alpha1CronJobForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListBatchV2alpha1CronJobForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListBatchV2alpha1CronJobForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation listBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// list or watch objects of kind CronJob
    ///
    /// Use [`ListBatchV2alpha1NamespacedCronJobResponse`](./enum.ListBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_batch_v2alpha1_namespaced_cron_job(
        namespace: &str,
        optional: ListBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListBatchV2alpha1NamespacedCronJobOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::list_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.list_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct ListBatchV2alpha1NamespacedCronJobOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::list_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.list_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum ListBatchV2alpha1NamespacedCronJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobList),
    Unauthorized,
    Other,
}

impl crate::Response for ListBatchV2alpha1NamespacedCronJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListBatchV2alpha1NamespacedCronJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((ListBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation patchBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// partially update the specified CronJob
    ///
    /// Use [`PatchBatchV2alpha1NamespacedCronJobResponse`](./enum.PatchBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_batch_v2alpha1_namespaced_cron_job(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchBatchV2alpha1NamespacedCronJobOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::patch_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct PatchBatchV2alpha1NamespacedCronJobOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::patch_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum PatchBatchV2alpha1NamespacedCronJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for PatchBatchV2alpha1NamespacedCronJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchBatchV2alpha1NamespacedCronJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((PatchBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation patchBatchV2alpha1NamespacedCronJobStatus

impl CronJob {
    /// partially update status of the specified CronJob
    ///
    /// Use [`PatchBatchV2alpha1NamespacedCronJobStatusResponse`](./enum.PatchBatchV2alpha1NamespacedCronJobStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_batch_v2alpha1_namespaced_cron_job_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchBatchV2alpha1NamespacedCronJobStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchBatchV2alpha1NamespacedCronJobStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::patch_batch_v2alpha1_namespaced_cron_job_status`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_cron_job_status)
#[derive(Debug, Default)]
pub struct PatchBatchV2alpha1NamespacedCronJobStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::patch_batch_v2alpha1_namespaced_cron_job_status`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_cron_job_status)
#[derive(Debug)]
pub enum PatchBatchV2alpha1NamespacedCronJobStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for PatchBatchV2alpha1NamespacedCronJobStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchBatchV2alpha1NamespacedCronJobStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchBatchV2alpha1NamespacedCronJobStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchBatchV2alpha1NamespacedCronJobStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// read the specified CronJob
    ///
    /// Use [`ReadBatchV2alpha1NamespacedCronJobResponse`](./enum.ReadBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_batch_v2alpha1_namespaced_cron_job(
        name: &str,
        namespace: &str,
        optional: ReadBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadBatchV2alpha1NamespacedCronJobOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::read_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct ReadBatchV2alpha1NamespacedCronJobOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::read_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum ReadBatchV2alpha1NamespacedCronJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReadBatchV2alpha1NamespacedCronJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadBatchV2alpha1NamespacedCronJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((ReadBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation readBatchV2alpha1NamespacedCronJobStatus

impl CronJob {
    /// read status of the specified CronJob
    ///
    /// Use [`ReadBatchV2alpha1NamespacedCronJobStatusResponse`](./enum.ReadBatchV2alpha1NamespacedCronJobStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_batch_v2alpha1_namespaced_cron_job_status(
        name: &str,
        namespace: &str,
        optional: ReadBatchV2alpha1NamespacedCronJobStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadBatchV2alpha1NamespacedCronJobStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`CronJob::read_batch_v2alpha1_namespaced_cron_job_status`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_cron_job_status)
#[derive(Debug, Default)]
pub struct ReadBatchV2alpha1NamespacedCronJobStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::read_batch_v2alpha1_namespaced_cron_job_status`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_cron_job_status)
#[derive(Debug)]
pub enum ReadBatchV2alpha1NamespacedCronJobStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReadBatchV2alpha1NamespacedCronJobStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadBatchV2alpha1NamespacedCronJobStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadBatchV2alpha1NamespacedCronJobStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadBatchV2alpha1NamespacedCronJobStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// replace the specified CronJob
    ///
    /// Use [`ReplaceBatchV2alpha1NamespacedCronJobResponse`](./enum.ReplaceBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_batch_v2alpha1_namespaced_cron_job(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob,
        optional: ReplaceBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceBatchV2alpha1NamespacedCronJobOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::replace_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct ReplaceBatchV2alpha1NamespacedCronJobOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::replace_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum ReplaceBatchV2alpha1NamespacedCronJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceBatchV2alpha1NamespacedCronJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceBatchV2alpha1NamespacedCronJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((ReplaceBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceBatchV2alpha1NamespacedCronJobStatus

impl CronJob {
    /// replace status of the specified CronJob
    ///
    /// Use [`ReplaceBatchV2alpha1NamespacedCronJobStatusResponse`](./enum.ReplaceBatchV2alpha1NamespacedCronJobStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_batch_v2alpha1_namespaced_cron_job_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob,
        optional: ReplaceBatchV2alpha1NamespacedCronJobStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceBatchV2alpha1NamespacedCronJobStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::replace_batch_v2alpha1_namespaced_cron_job_status`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_cron_job_status)
#[derive(Debug, Default)]
pub struct ReplaceBatchV2alpha1NamespacedCronJobStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::replace_batch_v2alpha1_namespaced_cron_job_status`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_cron_job_status)
#[derive(Debug)]
pub enum ReplaceBatchV2alpha1NamespacedCronJobStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceBatchV2alpha1NamespacedCronJobStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceBatchV2alpha1NamespacedCronJobStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceBatchV2alpha1NamespacedCronJobStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceBatchV2alpha1NamespacedCronJobStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchBatchV2alpha1CronJobListForAllNamespaces

impl CronJob {
    /// watch individual changes to a list of CronJob
    ///
    /// Use [`WatchBatchV2alpha1CronJobListForAllNamespacesResponse`](./enum.WatchBatchV2alpha1CronJobListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_batch_v2alpha1_cron_job_list_for_all_namespaces(
        optional: WatchBatchV2alpha1CronJobListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchBatchV2alpha1CronJobListForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/watch/cronjobs?");
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

/// Optional parameters of [`CronJob::watch_batch_v2alpha1_cron_job_list_for_all_namespaces`](./struct.CronJob.html#method.watch_batch_v2alpha1_cron_job_list_for_all_namespaces)
#[derive(Debug, Default)]
pub struct WatchBatchV2alpha1CronJobListForAllNamespacesOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::watch_batch_v2alpha1_cron_job_list_for_all_namespaces`](./struct.CronJob.html#method.watch_batch_v2alpha1_cron_job_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchBatchV2alpha1CronJobListForAllNamespacesResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchBatchV2alpha1CronJobListForAllNamespacesResponse {
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
                Ok((WatchBatchV2alpha1CronJobListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchBatchV2alpha1CronJobListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchBatchV2alpha1CronJobListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation watchBatchV2alpha1NamespacedCronJob

impl CronJob {
    /// watch changes to an object of kind CronJob
    ///
    /// Use [`WatchBatchV2alpha1NamespacedCronJobResponse`](./enum.WatchBatchV2alpha1NamespacedCronJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CronJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_batch_v2alpha1_namespaced_cron_job(
        name: &str,
        namespace: &str,
        optional: WatchBatchV2alpha1NamespacedCronJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchBatchV2alpha1NamespacedCronJobOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/watch/namespaces/{namespace}/cronjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::watch_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug, Default)]
pub struct WatchBatchV2alpha1NamespacedCronJobOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::watch_batch_v2alpha1_namespaced_cron_job`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_cron_job)
#[derive(Debug)]
pub enum WatchBatchV2alpha1NamespacedCronJobResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchBatchV2alpha1NamespacedCronJobResponse {
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
                Ok((WatchBatchV2alpha1NamespacedCronJobResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchBatchV2alpha1NamespacedCronJobResponse::Unauthorized, 0)),
            _ => Ok((WatchBatchV2alpha1NamespacedCronJobResponse::Other, 0)),
        }
    }
}

// Generated from operation watchBatchV2alpha1NamespacedCronJobList

impl CronJob {
    /// watch individual changes to a list of CronJob
    ///
    /// Use [`WatchBatchV2alpha1NamespacedCronJobListResponse`](./enum.WatchBatchV2alpha1NamespacedCronJobListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_batch_v2alpha1_namespaced_cron_job_list(
        namespace: &str,
        optional: WatchBatchV2alpha1NamespacedCronJobListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchBatchV2alpha1NamespacedCronJobListOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/watch/namespaces/{namespace}/cronjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::watch_batch_v2alpha1_namespaced_cron_job_list`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_cron_job_list)
#[derive(Debug, Default)]
pub struct WatchBatchV2alpha1NamespacedCronJobListOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::watch_batch_v2alpha1_namespaced_cron_job_list`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_cron_job_list)
#[derive(Debug)]
pub enum WatchBatchV2alpha1NamespacedCronJobListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchBatchV2alpha1NamespacedCronJobListResponse {
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
                Ok((WatchBatchV2alpha1NamespacedCronJobListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchBatchV2alpha1NamespacedCronJobListResponse::Unauthorized, 0)),
            _ => Ok((WatchBatchV2alpha1NamespacedCronJobListResponse::Other, 0)),
        }
    }
}

// End batch/v2alpha1/CronJob

// Begin batch/v2alpha1/ScheduledJob

// Generated from operation createBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// create a ScheduledJob
    ///
    /// Use [`CreateBatchV2alpha1NamespacedScheduledJobResponse`](./enum.CreateBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_batch_v2alpha1_namespaced_scheduled_job(
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob,
        optional: CreateBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateBatchV2alpha1NamespacedScheduledJobOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::create_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.create_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct CreateBatchV2alpha1NamespacedScheduledJobOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::create_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.create_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum CreateBatchV2alpha1NamespacedScheduledJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for CreateBatchV2alpha1NamespacedScheduledJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateBatchV2alpha1NamespacedScheduledJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((CreateBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteBatchV2alpha1CollectionNamespacedScheduledJob

impl CronJob {
    /// delete collection of ScheduledJob
    ///
    /// Use [`DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse`](./enum.DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_batch_v2alpha1_collection_namespaced_scheduled_job(
        namespace: &str,
        optional: DeleteBatchV2alpha1CollectionNamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteBatchV2alpha1CollectionNamespacedScheduledJobOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::delete_batch_v2alpha1_collection_namespaced_scheduled_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_collection_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct DeleteBatchV2alpha1CollectionNamespacedScheduledJobOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::delete_batch_v2alpha1_collection_namespaced_scheduled_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_collection_namespaced_scheduled_job)
#[derive(Debug)]
pub enum DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse {
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
                    Ok((DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((DeleteBatchV2alpha1CollectionNamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// delete a ScheduledJob
    ///
    /// Use [`DeleteBatchV2alpha1NamespacedScheduledJobResponse`](./enum.DeleteBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_batch_v2alpha1_namespaced_scheduled_job(
        name: &str,
        namespace: &str,
        optional: DeleteBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteBatchV2alpha1NamespacedScheduledJobOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::delete_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct DeleteBatchV2alpha1NamespacedScheduledJobOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::delete_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.delete_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum DeleteBatchV2alpha1NamespacedScheduledJobResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteBatchV2alpha1NamespacedScheduledJobResponse {
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
                    Ok((DeleteBatchV2alpha1NamespacedScheduledJobResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteBatchV2alpha1NamespacedScheduledJobResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((DeleteBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation listBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// list or watch objects of kind ScheduledJob
    ///
    /// Use [`ListBatchV2alpha1NamespacedScheduledJobResponse`](./enum.ListBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_batch_v2alpha1_namespaced_scheduled_job(
        namespace: &str,
        optional: ListBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListBatchV2alpha1NamespacedScheduledJobOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::list_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.list_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct ListBatchV2alpha1NamespacedScheduledJobOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::list_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.list_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum ListBatchV2alpha1NamespacedScheduledJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobList),
    Unauthorized,
    Other,
}

impl crate::Response for ListBatchV2alpha1NamespacedScheduledJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListBatchV2alpha1NamespacedScheduledJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((ListBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation listBatchV2alpha1ScheduledJobForAllNamespaces

impl CronJob {
    /// list or watch objects of kind ScheduledJob
    ///
    /// Use [`ListBatchV2alpha1ScheduledJobForAllNamespacesResponse`](./enum.ListBatchV2alpha1ScheduledJobForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_batch_v2alpha1_scheduled_job_for_all_namespaces(
        optional: ListBatchV2alpha1ScheduledJobForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListBatchV2alpha1ScheduledJobForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/scheduledjobs?");
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

/// Optional parameters of [`CronJob::list_batch_v2alpha1_scheduled_job_for_all_namespaces`](./struct.CronJob.html#method.list_batch_v2alpha1_scheduled_job_for_all_namespaces)
#[derive(Debug, Default)]
pub struct ListBatchV2alpha1ScheduledJobForAllNamespacesOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::list_batch_v2alpha1_scheduled_job_for_all_namespaces`](./struct.CronJob.html#method.list_batch_v2alpha1_scheduled_job_for_all_namespaces)
#[derive(Debug)]
pub enum ListBatchV2alpha1ScheduledJobForAllNamespacesResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobList),
    Unauthorized,
    Other,
}

impl crate::Response for ListBatchV2alpha1ScheduledJobForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListBatchV2alpha1ScheduledJobForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListBatchV2alpha1ScheduledJobForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListBatchV2alpha1ScheduledJobForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation patchBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// partially update the specified ScheduledJob
    ///
    /// Use [`PatchBatchV2alpha1NamespacedScheduledJobResponse`](./enum.PatchBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_batch_v2alpha1_namespaced_scheduled_job(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchBatchV2alpha1NamespacedScheduledJobOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::patch_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct PatchBatchV2alpha1NamespacedScheduledJobOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::patch_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum PatchBatchV2alpha1NamespacedScheduledJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for PatchBatchV2alpha1NamespacedScheduledJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchBatchV2alpha1NamespacedScheduledJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((PatchBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation patchBatchV2alpha1NamespacedScheduledJobStatus

impl CronJob {
    /// partially update status of the specified ScheduledJob
    ///
    /// Use [`PatchBatchV2alpha1NamespacedScheduledJobStatusResponse`](./enum.PatchBatchV2alpha1NamespacedScheduledJobStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_batch_v2alpha1_namespaced_scheduled_job_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchBatchV2alpha1NamespacedScheduledJobStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchBatchV2alpha1NamespacedScheduledJobStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::patch_batch_v2alpha1_namespaced_scheduled_job_status`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_scheduled_job_status)
#[derive(Debug, Default)]
pub struct PatchBatchV2alpha1NamespacedScheduledJobStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::patch_batch_v2alpha1_namespaced_scheduled_job_status`](./struct.CronJob.html#method.patch_batch_v2alpha1_namespaced_scheduled_job_status)
#[derive(Debug)]
pub enum PatchBatchV2alpha1NamespacedScheduledJobStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for PatchBatchV2alpha1NamespacedScheduledJobStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchBatchV2alpha1NamespacedScheduledJobStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchBatchV2alpha1NamespacedScheduledJobStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchBatchV2alpha1NamespacedScheduledJobStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// read the specified ScheduledJob
    ///
    /// Use [`ReadBatchV2alpha1NamespacedScheduledJobResponse`](./enum.ReadBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_batch_v2alpha1_namespaced_scheduled_job(
        name: &str,
        namespace: &str,
        optional: ReadBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadBatchV2alpha1NamespacedScheduledJobOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::read_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct ReadBatchV2alpha1NamespacedScheduledJobOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::read_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum ReadBatchV2alpha1NamespacedScheduledJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReadBatchV2alpha1NamespacedScheduledJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadBatchV2alpha1NamespacedScheduledJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((ReadBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation readBatchV2alpha1NamespacedScheduledJobStatus

impl CronJob {
    /// read status of the specified ScheduledJob
    ///
    /// Use [`ReadBatchV2alpha1NamespacedScheduledJobStatusResponse`](./enum.ReadBatchV2alpha1NamespacedScheduledJobStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_batch_v2alpha1_namespaced_scheduled_job_status(
        name: &str,
        namespace: &str,
        optional: ReadBatchV2alpha1NamespacedScheduledJobStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadBatchV2alpha1NamespacedScheduledJobStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`CronJob::read_batch_v2alpha1_namespaced_scheduled_job_status`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_scheduled_job_status)
#[derive(Debug, Default)]
pub struct ReadBatchV2alpha1NamespacedScheduledJobStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::read_batch_v2alpha1_namespaced_scheduled_job_status`](./struct.CronJob.html#method.read_batch_v2alpha1_namespaced_scheduled_job_status)
#[derive(Debug)]
pub enum ReadBatchV2alpha1NamespacedScheduledJobStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReadBatchV2alpha1NamespacedScheduledJobStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadBatchV2alpha1NamespacedScheduledJobStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadBatchV2alpha1NamespacedScheduledJobStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadBatchV2alpha1NamespacedScheduledJobStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// replace the specified ScheduledJob
    ///
    /// Use [`ReplaceBatchV2alpha1NamespacedScheduledJobResponse`](./enum.ReplaceBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_batch_v2alpha1_namespaced_scheduled_job(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob,
        optional: ReplaceBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceBatchV2alpha1NamespacedScheduledJobOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::replace_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct ReplaceBatchV2alpha1NamespacedScheduledJobOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::replace_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum ReplaceBatchV2alpha1NamespacedScheduledJobResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceBatchV2alpha1NamespacedScheduledJobResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceBatchV2alpha1NamespacedScheduledJobResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((ReplaceBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceBatchV2alpha1NamespacedScheduledJobStatus

impl CronJob {
    /// replace status of the specified ScheduledJob
    ///
    /// Use [`ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse`](./enum.ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_batch_v2alpha1_namespaced_scheduled_job_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob,
        optional: ReplaceBatchV2alpha1NamespacedScheduledJobStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceBatchV2alpha1NamespacedScheduledJobStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}/status?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::replace_batch_v2alpha1_namespaced_scheduled_job_status`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_scheduled_job_status)
#[derive(Debug, Default)]
pub struct ReplaceBatchV2alpha1NamespacedScheduledJobStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`CronJob::replace_batch_v2alpha1_namespaced_scheduled_job_status`](./struct.CronJob.html#method.replace_batch_v2alpha1_namespaced_scheduled_job_status)
#[derive(Debug)]
pub enum ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJob),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceBatchV2alpha1NamespacedScheduledJobStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchBatchV2alpha1NamespacedScheduledJob

impl CronJob {
    /// watch changes to an object of kind ScheduledJob
    ///
    /// Use [`WatchBatchV2alpha1NamespacedScheduledJobResponse`](./enum.WatchBatchV2alpha1NamespacedScheduledJobResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ScheduledJob
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_batch_v2alpha1_namespaced_scheduled_job(
        name: &str,
        namespace: &str,
        optional: WatchBatchV2alpha1NamespacedScheduledJobOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchBatchV2alpha1NamespacedScheduledJobOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/watch/namespaces/{namespace}/scheduledjobs/{name}?", name = name, namespace = namespace);
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

/// Optional parameters of [`CronJob::watch_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug, Default)]
pub struct WatchBatchV2alpha1NamespacedScheduledJobOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::watch_batch_v2alpha1_namespaced_scheduled_job`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_scheduled_job)
#[derive(Debug)]
pub enum WatchBatchV2alpha1NamespacedScheduledJobResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchBatchV2alpha1NamespacedScheduledJobResponse {
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
                Ok((WatchBatchV2alpha1NamespacedScheduledJobResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchBatchV2alpha1NamespacedScheduledJobResponse::Unauthorized, 0)),
            _ => Ok((WatchBatchV2alpha1NamespacedScheduledJobResponse::Other, 0)),
        }
    }
}

// Generated from operation watchBatchV2alpha1NamespacedScheduledJobList

impl CronJob {
    /// watch individual changes to a list of ScheduledJob
    ///
    /// Use [`WatchBatchV2alpha1NamespacedScheduledJobListResponse`](./enum.WatchBatchV2alpha1NamespacedScheduledJobListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_batch_v2alpha1_namespaced_scheduled_job_list(
        namespace: &str,
        optional: WatchBatchV2alpha1NamespacedScheduledJobListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchBatchV2alpha1NamespacedScheduledJobListOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/watch/namespaces/{namespace}/scheduledjobs?", namespace = namespace);
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

/// Optional parameters of [`CronJob::watch_batch_v2alpha1_namespaced_scheduled_job_list`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_scheduled_job_list)
#[derive(Debug, Default)]
pub struct WatchBatchV2alpha1NamespacedScheduledJobListOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::watch_batch_v2alpha1_namespaced_scheduled_job_list`](./struct.CronJob.html#method.watch_batch_v2alpha1_namespaced_scheduled_job_list)
#[derive(Debug)]
pub enum WatchBatchV2alpha1NamespacedScheduledJobListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchBatchV2alpha1NamespacedScheduledJobListResponse {
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
                Ok((WatchBatchV2alpha1NamespacedScheduledJobListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchBatchV2alpha1NamespacedScheduledJobListResponse::Unauthorized, 0)),
            _ => Ok((WatchBatchV2alpha1NamespacedScheduledJobListResponse::Other, 0)),
        }
    }
}

// Generated from operation watchBatchV2alpha1ScheduledJobListForAllNamespaces

impl CronJob {
    /// watch individual changes to a list of ScheduledJob
    ///
    /// Use [`WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse`](./enum.WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_batch_v2alpha1_scheduled_job_list_for_all_namespaces(
        optional: WatchBatchV2alpha1ScheduledJobListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchBatchV2alpha1ScheduledJobListForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/batch/v2alpha1/watch/scheduledjobs?");
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

/// Optional parameters of [`CronJob::watch_batch_v2alpha1_scheduled_job_list_for_all_namespaces`](./struct.CronJob.html#method.watch_batch_v2alpha1_scheduled_job_list_for_all_namespaces)
#[derive(Debug, Default)]
pub struct WatchBatchV2alpha1ScheduledJobListForAllNamespacesOptional<'a> {
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

/// Parses the HTTP response of [`CronJob::watch_batch_v2alpha1_scheduled_job_list_for_all_namespaces`](./struct.CronJob.html#method.watch_batch_v2alpha1_scheduled_job_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse {
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
                Ok((WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchBatchV2alpha1ScheduledJobListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// End batch/v2alpha1/ScheduledJob

impl<'de> serde::Deserialize<'de> for CronJob {
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

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            type Value = CronJob;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct CronJob")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobSpec> = None;
                let mut value_status: Option<crate::v1_7::kubernetes::pkg::apis::batch::v2alpha1::CronJobStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CronJob {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "CronJob",
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

impl serde::Serialize for CronJob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CronJob",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
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
