// Generated from definition io.k8s.api.admissionregistration.v1beta1.ValidatingWebhookConfiguration

/// ValidatingWebhookConfiguration describes the configuration of and admission webhook that accept or reject and object without changing it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidatingWebhookConfiguration {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Webhooks is a list of webhooks and the affected resources and operations.
    pub webhooks: Option<Vec<crate::v1_11::api::admissionregistration::v1beta1::Webhook>>,
}

// Begin admissionregistration.k8s.io/v1beta1/ValidatingWebhookConfiguration

// Generated from operation createAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// create a ValidatingWebhookConfiguration
    ///
    /// Use [`CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_admissionregistration_v1beta1_validating_webhook_configuration(
        body: &crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?");
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::create_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.create_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Created(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Accepted(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((CreateAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// delete collection of ValidatingWebhookConfiguration
    ///
    /// Use [`DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse`](./enum.DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn delete_admissionregistration_v1beta1_collection_validating_webhook_configuration(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?");
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::delete_admissionregistration_v1beta1_collection_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.delete_admissionregistration_v1beta1_collection_validating_webhook_configuration)
#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse {
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
                    Ok((DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// delete a ValidatingWebhookConfiguration
    ///
    /// Use [`DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
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
    ///     Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub fn delete_admissionregistration_v1beta1_validating_webhook_configuration(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::delete_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.delete_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
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
                    Ok((DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation listAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// list or watch objects of kind ValidatingWebhookConfiguration
    ///
    /// Use [`ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_admissionregistration_v1beta1_validating_webhook_configuration(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?");
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::list_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.list_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfigurationList),
    Unauthorized,
    Other,
}

impl crate::Response for ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ListAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// partially update the specified ValidatingWebhookConfiguration
    ///
    /// Use [`PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_admissionregistration_v1beta1_validating_webhook_configuration(
        name: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::patch_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.patch_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((PatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation readAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// read the specified ValidatingWebhookConfiguration
    ///
    /// Use [`ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
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
    pub fn read_admissionregistration_v1beta1_validating_webhook_configuration(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::read_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.read_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReadAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// replace the specified ValidatingWebhookConfiguration
    ///
    /// Use [`ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_admissionregistration_v1beta1_validating_webhook_configuration(
        name: &str,
        body: &crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::replace_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.replace_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Created(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// watch changes to an object of kind ValidatingWebhookConfiguration
    ///
    /// Use [`WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse`](./enum.WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_admissionregistration_v1beta1_validating_webhook_configuration(
        name: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/watch/validatingwebhookconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::watch_admissionregistration_v1beta1_validating_webhook_configuration`](./struct.ValidatingWebhookConfiguration.html#method.watch_admissionregistration_v1beta1_validating_webhook_configuration)
#[derive(Debug)]
pub enum WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse {
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
                Ok((WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1beta1ValidatingWebhookConfigurationList

impl ValidatingWebhookConfiguration {
    /// watch individual changes to a list of ValidatingWebhookConfiguration
    ///
    /// Use [`WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse`](./enum.WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
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
    ///     Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_admissionregistration_v1beta1_validating_webhook_configuration_list(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/watch/validatingwebhookconfigurations?");
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

/// Parses the HTTP response of [`ValidatingWebhookConfiguration::watch_admissionregistration_v1beta1_validating_webhook_configuration_list`](./struct.ValidatingWebhookConfiguration.html#method.watch_admissionregistration_v1beta1_validating_webhook_configuration_list)
#[derive(Debug)]
pub enum WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse {
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
                Ok((WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse::Unauthorized, 0)),
            _ => Ok((WatchAdmissionregistrationV1beta1ValidatingWebhookConfigurationListResponse::Other, 0)),
        }
    }
}

// End admissionregistration.k8s.io/v1beta1/ValidatingWebhookConfiguration

impl crate::Resource for ValidatingWebhookConfiguration {
    fn api_version() -> &'static str {
        "admissionregistration.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "admissionregistration.k8s.io"
    }

    fn kind() -> &'static str {
        "ValidatingWebhookConfiguration"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for ValidatingWebhookConfiguration {
    type Ty = crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&Self::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ValidatingWebhookConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_webhooks,
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
                            "webhooks" => Field::Key_webhooks,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ValidatingWebhookConfiguration;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ValidatingWebhookConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_webhooks: Option<Vec<crate::v1_11::api::admissionregistration::v1beta1::Webhook>> = None;

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
                        Field::Key_webhooks => value_webhooks = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidatingWebhookConfiguration {
                    metadata: value_metadata,
                    webhooks: value_webhooks,
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidatingWebhookConfiguration",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "webhooks",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ValidatingWebhookConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ValidatingWebhookConfiguration",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.webhooks.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.webhooks {
            serde::ser::SerializeStruct::serialize_field(&mut state, "webhooks", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
