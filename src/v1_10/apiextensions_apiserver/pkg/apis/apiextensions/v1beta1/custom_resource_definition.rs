// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinition

/// CustomResourceDefinition represents a resource that should be exposed on the API server.  Its name MUST be in the format \<.spec.name\>.\<.spec.group\>.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinition {
    pub metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec describes how the user wants the resources to appear
    pub spec: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec>,

    /// Status indicates the actual state of the CustomResourceDefinition
    pub status: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus>,
}

// Begin apiextensions.k8s.io/v1beta1/CustomResourceDefinition

// Generated from operation createApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// create a CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateCustomResourceDefinitionResponse`]`>` constructor, or [`CreateCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_custom_resource_definition(
        body: &crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        optional: CreateCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateCustomResourceDefinitionResponse>), crate::RequestError> {
        let CreateCustomResourceDefinitionOptional {
            pretty,
        } = optional;
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
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

/// Optional parameters of [`CustomResourceDefinition::create_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateCustomResourceDefinitionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::create_custom_resource_definition`]
#[derive(Debug)]
pub enum CreateCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Accepted(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCustomResourceDefinitionResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCustomResourceDefinitionResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((CreateCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiextensionsV1beta1CollectionCustomResourceDefinition

impl CustomResourceDefinition {
    /// delete collection of CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionCustomResourceDefinitionResponse`]`>` constructor, or [`DeleteCollectionCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_custom_resource_definition(
        optional: DeleteCollectionCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionCustomResourceDefinitionResponse>), crate::RequestError> {
        let DeleteCollectionCustomResourceDefinitionOptional {
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
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
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

/// Optional parameters of [`CustomResourceDefinition::delete_collection_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionCustomResourceDefinitionOptional<'a> {
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

/// Use `<DeleteCollectionCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::delete_collection_custom_resource_definition`]
#[derive(Debug)]
pub enum DeleteCollectionCustomResourceDefinitionResponse {
    OkStatus(crate::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionCustomResourceDefinitionResponse {
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
                    Ok((DeleteCollectionCustomResourceDefinitionResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionCustomResourceDefinitionResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// delete a CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCustomResourceDefinitionResponse`]`>` constructor, or [`DeleteCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_custom_resource_definition(
        name: &str,
        optional: DeleteCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCustomResourceDefinitionResponse>), crate::RequestError> {
        let DeleteCustomResourceDefinitionOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
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

/// Optional parameters of [`CustomResourceDefinition::delete_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCustomResourceDefinitionOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::delete_custom_resource_definition`]
#[derive(Debug)]
pub enum DeleteCustomResourceDefinitionResponse {
    OkStatus(crate::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCustomResourceDefinitionResponse {
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
                    Ok((DeleteCustomResourceDefinitionResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCustomResourceDefinitionResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((DeleteCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation listApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// list or watch objects of kind CustomResourceDefinition
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListCustomResourceDefinitionResponse`]`>` constructor, or [`ListCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_custom_resource_definition(
        optional: crate::v1_10::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListCustomResourceDefinitionResponse>), crate::RequestError> {
        let crate::v1_10::ListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
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

/// Use `<ListCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::list_custom_resource_definition`]
#[derive(Debug)]
pub enum ListCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((ListCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation patchApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// partially update the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchCustomResourceDefinitionResponse`]`>` constructor, or [`PatchCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_custom_resource_definition(
        name: &str,
        body: &crate::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchCustomResourceDefinitionResponse>), crate::RequestError> {
        let PatchCustomResourceDefinitionOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
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

/// Optional parameters of [`CustomResourceDefinition::patch_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchCustomResourceDefinitionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::patch_custom_resource_definition`]
#[derive(Debug)]
pub enum PatchCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((PatchCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation readApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// read the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadCustomResourceDefinitionResponse`]`>` constructor, or [`ReadCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_custom_resource_definition(
        name: &str,
        optional: ReadCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadCustomResourceDefinitionResponse>), crate::RequestError> {
        let ReadCustomResourceDefinitionOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
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

/// Optional parameters of [`CustomResourceDefinition::read_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadCustomResourceDefinitionOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::read_custom_resource_definition`]
#[derive(Debug)]
pub enum ReadCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((ReadCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// replace the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceCustomResourceDefinitionResponse`]`>` constructor, or [`ReplaceCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_custom_resource_definition(
        name: &str,
        body: &crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        optional: ReplaceCustomResourceDefinitionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceCustomResourceDefinitionResponse>), crate::RequestError> {
        let ReplaceCustomResourceDefinitionOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
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

/// Optional parameters of [`CustomResourceDefinition::replace_custom_resource_definition`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceCustomResourceDefinitionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::replace_custom_resource_definition`]
#[derive(Debug)]
pub enum ReplaceCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCustomResourceDefinitionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiextensionsV1beta1CustomResourceDefinitionStatus

impl CustomResourceDefinition {
    /// replace status of the specified CustomResourceDefinition
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceCustomResourceDefinitionStatusResponse`]`>` constructor, or [`ReplaceCustomResourceDefinitionStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_custom_resource_definition_status(
        name: &str,
        body: &crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        optional: ReplaceCustomResourceDefinitionStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceCustomResourceDefinitionStatusResponse>), crate::RequestError> {
        let ReplaceCustomResourceDefinitionStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}/status?", name = name);
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

/// Optional parameters of [`CustomResourceDefinition::replace_custom_resource_definition_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceCustomResourceDefinitionStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceCustomResourceDefinitionStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::replace_custom_resource_definition_status`]
#[derive(Debug)]
pub enum ReplaceCustomResourceDefinitionStatusResponse {
    Ok(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCustomResourceDefinitionStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCustomResourceDefinitionStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCustomResourceDefinitionStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCustomResourceDefinitionStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// list or watch objects of kind CustomResourceDefinition
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchCustomResourceDefinitionResponse`]`>` constructor, or [`WatchCustomResourceDefinitionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_custom_resource_definition(
        optional: crate::v1_10::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchCustomResourceDefinitionResponse>), crate::RequestError> {
        let crate::v1_10::WatchOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?".to_string();
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

/// Use `<WatchCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::watch_custom_resource_definition`]
#[derive(Debug)]
pub enum WatchCustomResourceDefinitionResponse {
    Ok(crate::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCustomResourceDefinitionResponse {
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
                Ok((WatchCustomResourceDefinitionResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((WatchCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// End apiextensions.k8s.io/v1beta1/CustomResourceDefinition

impl crate::Resource for CustomResourceDefinition {
    fn api_version() -> &'static str {
        "apiextensions.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "apiextensions.k8s.io"
    }

    fn kind() -> &'static str {
        "CustomResourceDefinition"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for CustomResourceDefinition {
    type Ty = crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for CustomResourceDefinition {
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
            type Value = CustomResourceDefinition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct CustomResourceDefinition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec> = None;
                let mut value_status: Option<crate::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus> = None;

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

                Ok(CustomResourceDefinition {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinition",
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

impl serde::Serialize for CustomResourceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinition",
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
