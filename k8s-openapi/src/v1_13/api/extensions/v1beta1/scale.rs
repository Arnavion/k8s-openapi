// Generated from definition io.k8s.api.extensions.v1beta1.Scale

/// represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_13::api::extensions::v1beta1::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<::v1_13::api::extensions::v1beta1::ScaleStatus>,
}

// Begin extensions/v1beta1/Scale

// Generated from operation patchExtensionsV1beta1NamespacedDeploymentScale

impl Scale {
    /// partially update scale of the specified Deployment
    ///
    /// Use [`PatchExtensionsV1beta1NamespacedDeploymentScaleResponse`](./enum.PatchExtensionsV1beta1NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn patch_extensions_v1beta1_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::patch_extensions_v1beta1_namespaced_deployment_scale`](./struct.Scale.html#method.patch_extensions_v1beta1_namespaced_deployment_scale)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedDeploymentScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for PatchExtensionsV1beta1NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedReplicaSetScale

impl Scale {
    /// partially update scale of the specified ReplicaSet
    ///
    /// Use [`PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse`](./enum.PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn patch_extensions_v1beta1_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::patch_extensions_v1beta1_namespaced_replica_set_scale`](./struct.Scale.html#method.patch_extensions_v1beta1_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedReplicationControllerDummyScale

impl Scale {
    /// partially update scale of the specified ReplicationControllerDummy
    ///
    /// Use [`PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse`](./enum.PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn patch_extensions_v1beta1_namespaced_replication_controller_dummy_scale(
        name: &str,
        namespace: &str,
        body: &::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::patch_extensions_v1beta1_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.patch_extensions_v1beta1_namespaced_replication_controller_dummy_scale)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedDeploymentScale

impl Scale {
    /// read scale of the specified Deployment
    ///
    /// Use [`ReadExtensionsV1beta1NamespacedDeploymentScaleResponse`](./enum.ReadExtensionsV1beta1NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_extensions_v1beta1_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::read_extensions_v1beta1_namespaced_deployment_scale`](./struct.Scale.html#method.read_extensions_v1beta1_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedDeploymentScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReadExtensionsV1beta1NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedReplicaSetScale

impl Scale {
    /// read scale of the specified ReplicaSet
    ///
    /// Use [`ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse`](./enum.ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_extensions_v1beta1_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::read_extensions_v1beta1_namespaced_replica_set_scale`](./struct.Scale.html#method.read_extensions_v1beta1_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedReplicationControllerDummyScale

impl Scale {
    /// read scale of the specified ReplicationControllerDummy
    ///
    /// Use [`ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse`](./enum.ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_extensions_v1beta1_namespaced_replication_controller_dummy_scale(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::read_extensions_v1beta1_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.read_extensions_v1beta1_namespaced_replication_controller_dummy_scale)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedDeploymentScale

impl Scale {
    /// replace scale of the specified Deployment
    ///
    /// Use [`ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse`](./enum.ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn replace_extensions_v1beta1_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &::v1_13::api::extensions::v1beta1::Scale,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::replace_extensions_v1beta1_namespaced_deployment_scale`](./struct.Scale.html#method.replace_extensions_v1beta1_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Created(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedReplicaSetScale

impl Scale {
    /// replace scale of the specified ReplicaSet
    ///
    /// Use [`ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse`](./enum.ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn replace_extensions_v1beta1_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &::v1_13::api::extensions::v1beta1::Scale,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::replace_extensions_v1beta1_namespaced_replica_set_scale`](./struct.Scale.html#method.replace_extensions_v1beta1_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Created(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedReplicationControllerDummyScale

impl Scale {
    /// replace scale of the specified ReplicationControllerDummy
    ///
    /// Use [`ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse`](./enum.ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
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
    pub fn replace_extensions_v1beta1_namespaced_replication_controller_dummy_scale(
        name: &str,
        namespace: &str,
        body: &::v1_13::api::extensions::v1beta1::Scale,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Parses the HTTP response of [`Scale::replace_extensions_v1beta1_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.replace_extensions_v1beta1_namespaced_replication_controller_dummy_scale)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse {
    Ok(::v1_13::api::extensions::v1beta1::Scale),
    Created(::v1_13::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Other, 0)),
        }
    }
}

// End extensions/v1beta1/Scale

impl ::Resource for Scale {
    fn api_version() -> &'static str {
        "extensions/v1beta1"
    }

    fn group() -> &'static str {
        "extensions"
    }

    fn kind() -> &'static str {
        "Scale"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl<'de> ::serde::Deserialize<'de> for Scale {
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
            type Value = Scale;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Scale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_13::api::extensions::v1beta1::ScaleSpec> = None;
                let mut value_status: Option<::v1_13::api::extensions::v1beta1::ScaleStatus> = None;

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
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scale {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Scale",
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

impl ::serde::Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scale",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
