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
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateValidatingWebhookConfigurationResponse`]`>` constructor, or [`CreateValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_validating_webhook_configuration(
        body: &crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration,
        optional: CreateValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let CreateValidatingWebhookConfigurationOptional {
            pretty,
        } = optional;
        let __url = "/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::create_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateValidatingWebhookConfigurationOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::create_validating_webhook_configuration`]
#[derive(Debug)]
pub enum CreateValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Created(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Accepted(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for CreateValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateValidatingWebhookConfigurationResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateValidatingWebhookConfigurationResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((CreateValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1beta1CollectionValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// delete collection of ValidatingWebhookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionValidatingWebhookConfigurationResponse`]`>` constructor, or [`DeleteCollectionValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_validating_webhook_configuration(
        optional: DeleteCollectionValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let DeleteCollectionValidatingWebhookConfigurationOptional {
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
        let __url = "/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?".to_string();
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::delete_collection_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionValidatingWebhookConfigurationOptional<'a> {
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
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<DeleteCollectionValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::delete_collection_validating_webhook_configuration`]
#[derive(Debug)]
pub enum DeleteCollectionValidatingWebhookConfigurationResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionValidatingWebhookConfigurationResponse {
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
                    Ok((DeleteCollectionValidatingWebhookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionValidatingWebhookConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// delete a ValidatingWebhookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteValidatingWebhookConfigurationResponse`]`>` constructor, or [`DeleteValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_validating_webhook_configuration(
        name: &str,
        optional: DeleteValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let DeleteValidatingWebhookConfigurationOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::delete_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteValidatingWebhookConfigurationOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::delete_validating_webhook_configuration`]
#[derive(Debug)]
pub enum DeleteValidatingWebhookConfigurationResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteValidatingWebhookConfigurationResponse {
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
                    Ok((DeleteValidatingWebhookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteValidatingWebhookConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation listAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// list or watch objects of kind ValidatingWebhookConfiguration
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListValidatingWebhookConfigurationResponse`]`>` constructor, or [`ListValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_validating_webhook_configuration(
        optional: ListValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let ListValidatingWebhookConfigurationOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?".to_string();
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
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::list_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListValidatingWebhookConfigurationOptional<'a> {
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
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
}

/// Use `<ListValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::list_validating_webhook_configuration`]
#[derive(Debug)]
pub enum ListValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfigurationList),
    Unauthorized,
    Other,
}

impl crate::Response for ListValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ListValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// partially update the specified ValidatingWebhookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchValidatingWebhookConfigurationResponse`]`>` constructor, or [`PatchValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_validating_webhook_configuration(
        name: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let PatchValidatingWebhookConfigurationOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::patch_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchValidatingWebhookConfigurationOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::patch_validating_webhook_configuration`]
#[derive(Debug)]
pub enum PatchValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for PatchValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((PatchValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation readAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// read the specified ValidatingWebhookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadValidatingWebhookConfigurationResponse`]`>` constructor, or [`ReadValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_validating_webhook_configuration(
        name: &str,
        optional: ReadValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let ReadValidatingWebhookConfigurationOptional {
            exact,
            export,
            pretty,
        } = optional;
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
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::read_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadValidatingWebhookConfigurationOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::read_validating_webhook_configuration`]
#[derive(Debug)]
pub enum ReadValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for ReadValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReadValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// replace the specified ValidatingWebhookConfiguration
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceValidatingWebhookConfigurationResponse`]`>` constructor, or [`ReplaceValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ValidatingWebhookConfiguration
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_validating_webhook_configuration(
        name: &str,
        body: &crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration,
        optional: ReplaceValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let ReplaceValidatingWebhookConfigurationOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::replace_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceValidatingWebhookConfigurationOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::replace_validating_webhook_configuration`]
#[derive(Debug)]
pub enum ReplaceValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Created(crate::v1_11::api::admissionregistration::v1beta1::ValidatingWebhookConfiguration),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceValidatingWebhookConfigurationResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceValidatingWebhookConfigurationResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceValidatingWebhookConfigurationResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReplaceValidatingWebhookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1beta1ValidatingWebhookConfiguration

impl ValidatingWebhookConfiguration {
    /// list or watch objects of kind ValidatingWebhookConfiguration
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchValidatingWebhookConfigurationResponse`]`>` constructor, or [`WatchValidatingWebhookConfigurationResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_validating_webhook_configuration(
        optional: WatchValidatingWebhookConfigurationOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchValidatingWebhookConfigurationResponse>), crate::RequestError> {
        let WatchValidatingWebhookConfigurationOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/admissionregistration.k8s.io/v1beta1/validatingwebhookconfigurations?".to_string();
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
        __query_pairs.append_pair("watch", "true");
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ValidatingWebhookConfiguration::watch_validating_webhook_configuration`]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchValidatingWebhookConfigurationOptional<'a> {
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
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
}

/// Use `<WatchValidatingWebhookConfigurationResponse as Response>::try_from_parts` to parse the HTTP response body of [`ValidatingWebhookConfiguration::watch_validating_webhook_configuration`]
#[derive(Debug)]
pub enum WatchValidatingWebhookConfigurationResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchValidatingWebhookConfigurationResponse {
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
                Ok((WatchValidatingWebhookConfigurationResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchValidatingWebhookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((WatchValidatingWebhookConfigurationResponse::Other, 0)),
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

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
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
