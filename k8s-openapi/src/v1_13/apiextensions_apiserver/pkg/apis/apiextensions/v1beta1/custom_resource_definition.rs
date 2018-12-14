// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinition

/// CustomResourceDefinition represents a resource that should be exposed on the API server.  Its name MUST be in the format <.spec.name>.<.spec.group>.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinition {
    pub metadata: Option<::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec describes how the user wants the resources to appear
    pub spec: ::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec,

    /// Status indicates the actual state of the CustomResourceDefinition
    pub status: Option<::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus>,
}

// Begin apiextensions.k8s.io/v1beta1/CustomResourceDefinition

// Generated from operation createApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// create a CustomResourceDefinition
    ///
    /// Use [`CreateApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.CreateApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_apiextensions_v1beta1_custom_resource_definition(
        body: &::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        dry_run: Option<&str>,
        include_uninitialized: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::create_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.create_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum CreateApiextensionsV1beta1CustomResourceDefinitionResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Accepted(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for CreateApiextensionsV1beta1CustomResourceDefinitionResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::ACCEPTED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Accepted(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiextensionsV1beta1CollectionCustomResourceDefinition

impl CustomResourceDefinition {
    /// delete collection of CustomResourceDefinition
    ///
    /// Use [`DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse`](./enum.DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    ///     This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    pub fn delete_apiextensions_v1beta1_collection_custom_resource_definition(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::delete_apiextensions_v1beta1_collection_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.delete_apiextensions_v1beta1_collection_custom_resource_definition)
#[derive(Debug)]
pub enum DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse {
    OkStatus(::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse {
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
                    Ok((DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((DeleteApiextensionsV1beta1CollectionCustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// delete a CustomResourceDefinition
    ///
    /// Use [`DeleteApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.DeleteApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
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
    pub fn delete_apiextensions_v1beta1_custom_resource_definition(
        name: &str,
        dry_run: Option<&str>,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
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

/// Parses the HTTP response of [`CustomResourceDefinition::delete_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.delete_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum DeleteApiextensionsV1beta1CustomResourceDefinitionResponse {
    OkStatus(::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Accepted(::v1_13::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl ::Response for DeleteApiextensionsV1beta1CustomResourceDefinitionResponse {
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
                    Ok((DeleteApiextensionsV1beta1CustomResourceDefinitionResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteApiextensionsV1beta1CustomResourceDefinitionResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::ACCEPTED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((DeleteApiextensionsV1beta1CustomResourceDefinitionResponse::Accepted(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((DeleteApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation listApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// list or watch objects of kind CustomResourceDefinition
    ///
    /// Use [`ListApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.ListApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    ///     This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    pub fn list_apiextensions_v1beta1_custom_resource_definition(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::list_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.list_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum ListApiextensionsV1beta1CustomResourceDefinitionResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionList),
    Unauthorized,
    Other,
}

impl ::Response for ListApiextensionsV1beta1CustomResourceDefinitionResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListApiextensionsV1beta1CustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((ListApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation patchApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// partially update the specified CustomResourceDefinition
    ///
    /// Use [`PatchApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.PatchApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_apiextensions_v1beta1_custom_resource_definition(
        name: &str,
        body: &::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::patch_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.patch_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum PatchApiextensionsV1beta1CustomResourceDefinitionResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for PatchApiextensionsV1beta1CustomResourceDefinitionResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchApiextensionsV1beta1CustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((PatchApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation patchApiextensionsV1beta1CustomResourceDefinitionStatus

impl CustomResourceDefinition {
    /// partially update status of the specified CustomResourceDefinition
    ///
    /// Use [`PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse`](./enum.PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_apiextensions_v1beta1_custom_resource_definition_status(
        name: &str,
        body: &::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}/status?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::patch_apiextensions_v1beta1_custom_resource_definition_status`](./struct.CustomResourceDefinition.html#method.patch_apiextensions_v1beta1_custom_resource_definition_status)
#[derive(Debug)]
pub enum PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// read the specified CustomResourceDefinition
    ///
    /// Use [`ReadApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.ReadApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
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
    pub fn read_apiextensions_v1beta1_custom_resource_definition(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
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

/// Parses the HTTP response of [`CustomResourceDefinition::read_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.read_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum ReadApiextensionsV1beta1CustomResourceDefinitionResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for ReadApiextensionsV1beta1CustomResourceDefinitionResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadApiextensionsV1beta1CustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((ReadApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation readApiextensionsV1beta1CustomResourceDefinitionStatus

impl CustomResourceDefinition {
    /// read status of the specified CustomResourceDefinition
    ///
    /// Use [`ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse`](./enum.ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_apiextensions_v1beta1_custom_resource_definition_status(
        name: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}/status?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::read_apiextensions_v1beta1_custom_resource_definition_status`](./struct.CustomResourceDefinition.html#method.read_apiextensions_v1beta1_custom_resource_definition_status)
#[derive(Debug)]
pub enum ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// replace the specified CustomResourceDefinition
    ///
    /// Use [`ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_apiextensions_v1beta1_custom_resource_definition(
        name: &str,
        body: &::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::replace_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.replace_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceApiextensionsV1beta1CustomResourceDefinitionStatus

impl CustomResourceDefinition {
    /// replace status of the specified CustomResourceDefinition
    ///
    /// Use [`ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse`](./enum.ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_apiextensions_v1beta1_custom_resource_definition_status(
        name: &str,
        body: &::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}/status?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::replace_apiextensions_v1beta1_custom_resource_definition_status`](./struct.CustomResourceDefinition.html#method.replace_apiextensions_v1beta1_custom_resource_definition_status)
#[derive(Debug)]
pub enum ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse {
    Ok(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Created(::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinition),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceApiextensionsV1beta1CustomResourceDefinitionStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchApiextensionsV1beta1CustomResourceDefinition

impl CustomResourceDefinition {
    /// watch changes to an object of kind CustomResourceDefinition. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use [`WatchApiextensionsV1beta1CustomResourceDefinitionResponse`](./enum.WatchApiextensionsV1beta1CustomResourceDefinitionResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CustomResourceDefinition
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    ///     This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    pub fn watch_apiextensions_v1beta1_custom_resource_definition(
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
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/watch/customresourcedefinitions/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::watch_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.watch_apiextensions_v1beta1_custom_resource_definition)
#[derive(Debug)]
pub enum WatchApiextensionsV1beta1CustomResourceDefinitionResponse {
    Ok(::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchApiextensionsV1beta1CustomResourceDefinitionResponse {
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
                Ok((WatchApiextensionsV1beta1CustomResourceDefinitionResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchApiextensionsV1beta1CustomResourceDefinitionResponse::Unauthorized, 0)),
            _ => Ok((WatchApiextensionsV1beta1CustomResourceDefinitionResponse::Other, 0)),
        }
    }
}

// Generated from operation watchApiextensionsV1beta1CustomResourceDefinitionList

impl CustomResourceDefinition {
    /// watch individual changes to a list of CustomResourceDefinition. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchApiextensionsV1beta1CustomResourceDefinitionListResponse`](./enum.WatchApiextensionsV1beta1CustomResourceDefinitionListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    ///     This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
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
    pub fn watch_apiextensions_v1beta1_custom_resource_definition_list(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apiextensions.k8s.io/v1beta1/watch/customresourcedefinitions?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`CustomResourceDefinition::watch_apiextensions_v1beta1_custom_resource_definition_list`](./struct.CustomResourceDefinition.html#method.watch_apiextensions_v1beta1_custom_resource_definition_list)
#[derive(Debug)]
pub enum WatchApiextensionsV1beta1CustomResourceDefinitionListResponse {
    Ok(::v1_13::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchApiextensionsV1beta1CustomResourceDefinitionListResponse {
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
                Ok((WatchApiextensionsV1beta1CustomResourceDefinitionListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchApiextensionsV1beta1CustomResourceDefinitionListResponse::Unauthorized, 0)),
            _ => Ok((WatchApiextensionsV1beta1CustomResourceDefinitionListResponse::Other, 0)),
        }
    }
}

// End apiextensions.k8s.io/v1beta1/CustomResourceDefinition

impl ::Resource for CustomResourceDefinition {
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

impl<'de> ::serde::Deserialize<'de> for CustomResourceDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
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
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinition;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceDefinition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionSpec> = None;
                let mut value_status: Option<::v1_13::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionStatus> = None;

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
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinition {
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| ::serde::de::Error::missing_field("spec"))?,
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

impl ::serde::Serialize for CustomResourceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinition",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1 +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
