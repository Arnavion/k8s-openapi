// Generated from definition io.k8s.api.core.v1.Namespace

/// Namespace provides a scope for Names. Use of multiple namespaces is optional.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Namespace {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the behavior of the Namespace. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_14::api::core::v1::NamespaceSpec>,

    /// Status describes the current status of a Namespace. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_14::api::core::v1::NamespaceStatus>,
}

// Begin /v1/Namespace

// Generated from operation createCoreV1Namespace

impl Namespace {
    /// create a Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespaceResponse`]`>` constructor, or [`CreateNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_namespace(
        body: &crate::v1_14::api::core::v1::Namespace,
        optional: CreateNamespaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespaceResponse>), crate::RequestError> {
        let CreateNamespaceOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = "/api/v1/namespaces?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
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

/// Optional parameters of [`Namespace::create_namespace`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespaceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::create_namespace`]
#[derive(Debug)]
pub enum CreateNamespaceResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Created(crate::v1_14::api::core::v1::Namespace),
    Accepted(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespaceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespaceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespaceResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespaceResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespaceResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespaceResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteCoreV1Namespace

impl Namespace {
    /// delete a Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespaceResponse`]`>` constructor, or [`DeleteNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespace(
        name: &str,
        optional: DeleteNamespaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespaceResponse>), crate::RequestError> {
        let DeleteNamespaceOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Namespace::delete_namespace`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteNamespaceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::delete_namespace`]
#[derive(Debug)]
pub enum DeleteNamespaceResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::core::v1::Namespace),
    Accepted(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteNamespaceResponse {
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
                    Ok((DeleteNamespaceResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespaceResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteNamespaceResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteNamespaceResponse::Unauthorized, 0)),
            _ => Ok((DeleteNamespaceResponse::Other, 0)),
        }
    }
}

// Generated from operation listCoreV1Namespace

impl Namespace {
    /// list or watch objects of kind Namespace
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespaceResponse`]`>` constructor, or [`ListNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_namespace(
        optional: crate::v1_14::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespaceResponse>), crate::RequestError> {
        let crate::v1_14::ListOptional {
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/namespaces?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
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

/// Use `<ListNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::list_namespace`]
#[derive(Debug)]
pub enum ListNamespaceResponse {
    Ok(crate::v1_14::api::core::v1::NamespaceList),
    Unauthorized,
    Other,
}

impl crate::Response for ListNamespaceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespaceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListNamespaceResponse::Unauthorized, 0)),
            _ => Ok((ListNamespaceResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1Namespace

impl Namespace {
    /// partially update the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespaceResponse`]`>` constructor, or [`PatchNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespace(
        name: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespaceResponse>), crate::RequestError> {
        let PatchNamespaceOptional {
            dry_run,
            field_manager,
            force,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", &force.to_string());
        }
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

/// Optional parameters of [`Namespace::patch_namespace`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespaceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::patch_namespace`]
#[derive(Debug)]
pub enum PatchNamespaceResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespaceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespaceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespaceResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespaceResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespaceStatus

impl Namespace {
    /// partially update status of the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespaceStatusResponse`]`>` constructor, or [`PatchNamespaceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespace_status(
        name: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespaceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespaceStatusResponse>), crate::RequestError> {
        let PatchNamespaceStatusOptional {
            dry_run,
            field_manager,
            force,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", &force.to_string());
        }
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

/// Optional parameters of [`Namespace::patch_namespace_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespaceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchNamespaceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::patch_namespace_status`]
#[derive(Debug)]
pub enum PatchNamespaceStatusResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespaceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespaceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespaceStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespaceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1Namespace

impl Namespace {
    /// read the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespaceResponse`]`>` constructor, or [`ReadNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespace(
        name: &str,
        optional: ReadNamespaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespaceResponse>), crate::RequestError> {
        let ReadNamespaceOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}?", name = name);
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

/// Optional parameters of [`Namespace::read_namespace`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespaceOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::read_namespace`]
#[derive(Debug)]
pub enum ReadNamespaceResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespaceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespaceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespaceResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespaceResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespaceStatus

impl Namespace {
    /// read status of the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespaceStatusResponse`]`>` constructor, or [`ReadNamespaceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespace_status(
        name: &str,
        optional: ReadNamespaceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespaceStatusResponse>), crate::RequestError> {
        let ReadNamespaceStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`Namespace::read_namespace_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespaceStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespaceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::read_namespace_status`]
#[derive(Debug)]
pub enum ReadNamespaceStatusResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespaceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespaceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespaceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespaceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1Namespace

impl Namespace {
    /// replace the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespaceResponse`]`>` constructor, or [`ReplaceNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespace(
        name: &str,
        body: &crate::v1_14::api::core::v1::Namespace,
        optional: ReplaceNamespaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespaceResponse>), crate::RequestError> {
        let ReplaceNamespaceOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
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

/// Optional parameters of [`Namespace::replace_namespace`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespaceOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::replace_namespace`]
#[derive(Debug)]
pub enum ReplaceNamespaceResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Created(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespaceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespaceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespaceResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespaceResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespaceResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespaceFinalize

impl Namespace {
    /// replace finalize of the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespaceFinalizeResponse`]`>` constructor, or [`ReplaceNamespaceFinalizeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespace_finalize(
        name: &str,
        body: &crate::v1_14::api::core::v1::Namespace,
        optional: ReplaceNamespaceFinalizeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespaceFinalizeResponse>), crate::RequestError> {
        let ReplaceNamespaceFinalizeOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}/finalize?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
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

/// Optional parameters of [`Namespace::replace_namespace_finalize`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespaceFinalizeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespaceFinalizeResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::replace_namespace_finalize`]
#[derive(Debug)]
pub enum ReplaceNamespaceFinalizeResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Created(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespaceFinalizeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespaceFinalizeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespaceFinalizeResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespaceFinalizeResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespaceFinalizeResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespaceStatus

impl Namespace {
    /// replace status of the specified Namespace
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespaceStatusResponse`]`>` constructor, or [`ReplaceNamespaceStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Namespace
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespace_status(
        name: &str,
        body: &crate::v1_14::api::core::v1::Namespace,
        optional: ReplaceNamespaceStatusOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespaceStatusResponse>), crate::RequestError> {
        let ReplaceNamespaceStatusOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{name}/status?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
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

/// Optional parameters of [`Namespace::replace_namespace_status`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespaceStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespaceStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::replace_namespace_status`]
#[derive(Debug)]
pub enum ReplaceNamespaceStatusResponse {
    Ok(crate::v1_14::api::core::v1::Namespace),
    Created(crate::v1_14::api::core::v1::Namespace),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespaceStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespaceStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespaceStatusResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespaceStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespaceStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchCoreV1Namespace

impl Namespace {
    /// list or watch objects of kind Namespace
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespaceResponse`]`>` constructor, or [`WatchNamespaceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespace(
        optional: crate::v1_14::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespaceResponse>), crate::RequestError> {
        let crate::v1_14::WatchOptional {
            field_selector,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/api/v1/namespaces?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
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

/// Use `<WatchNamespaceResponse as Response>::try_from_parts` to parse the HTTP response body of [`Namespace::watch_namespace`]
#[derive(Debug)]
pub enum WatchNamespaceResponse {
    Ok(crate::v1_14::apimachinery::pkg::apis::meta::v1::WatchEvent<Namespace>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchNamespaceResponse {
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
                Ok((WatchNamespaceResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchNamespaceResponse::Unauthorized, 0)),
            _ => Ok((WatchNamespaceResponse::Other, 0)),
        }
    }
}

// End /v1/Namespace

impl crate::Resource for Namespace {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "Namespace"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for Namespace {
    type Ty = crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Namespace {
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
            type Value = Namespace;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Namespace")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_14::api::core::v1::NamespaceSpec> = None;
                let mut value_status: Option<crate::v1_14::api::core::v1::NamespaceStatus> = None;

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

                Ok(Namespace {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Namespace",
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

impl serde::Serialize for Namespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Namespace",
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
