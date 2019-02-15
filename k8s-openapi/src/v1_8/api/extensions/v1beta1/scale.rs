// Generated from definition io.k8s.api.extensions.v1beta1.Scale

/// represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<crate::v1_8::api::extensions::v1beta1::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<crate::v1_8::api::extensions::v1beta1::ScaleStatus>,
}

// Begin extensions/v1beta1/Scale

// Generated from operation patchExtensionsV1beta1NamespacedDeploymentScale

impl Scale {
    /// partially update scale of the specified Deployment
    ///
    /// Use [`PatchNamespacedDeploymentScaleResponse`](./enum.PatchNamespacedDeploymentScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedDeploymentScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_namespaced_deployment_scale`](./struct.Scale.html#method.patch_namespaced_deployment_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_namespaced_deployment_scale`](./struct.Scale.html#method.patch_namespaced_deployment_scale)
#[derive(Debug)]
pub enum PatchNamespacedDeploymentScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedReplicaSetScale

impl Scale {
    /// partially update scale of the specified ReplicaSet
    ///
    /// Use [`PatchNamespacedReplicaSetScaleResponse`](./enum.PatchNamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedReplicaSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedReplicaSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_namespaced_replica_set_scale`](./struct.Scale.html#method.patch_namespaced_replica_set_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedReplicaSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_namespaced_replica_set_scale`](./struct.Scale.html#method.patch_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum PatchNamespacedReplicaSetScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedReplicationControllerDummyScale

impl Scale {
    /// partially update scale of the specified ReplicationControllerDummy
    ///
    /// Use [`PatchNamespacedReplicationControllerDummyScaleResponse`](./enum.PatchNamespacedReplicationControllerDummyScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_replication_controller_dummy_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedReplicationControllerDummyScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedReplicationControllerDummyScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.patch_namespaced_replication_controller_dummy_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchNamespacedReplicationControllerDummyScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.patch_namespaced_replication_controller_dummy_scale)
#[derive(Debug)]
pub enum PatchNamespacedReplicationControllerDummyScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchNamespacedReplicationControllerDummyScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedReplicationControllerDummyScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchNamespacedReplicationControllerDummyScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchNamespacedReplicationControllerDummyScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedDeploymentScale

impl Scale {
    /// read scale of the specified Deployment
    ///
    /// Use [`ReadNamespacedDeploymentScaleResponse`](./enum.ReadNamespacedDeploymentScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedDeploymentScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_namespaced_deployment_scale`](./struct.Scale.html#method.read_namespaced_deployment_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_namespaced_deployment_scale`](./struct.Scale.html#method.read_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReadNamespacedDeploymentScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedReplicaSetScale

impl Scale {
    /// read scale of the specified ReplicaSet
    ///
    /// Use [`ReadNamespacedReplicaSetScaleResponse`](./enum.ReadNamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedReplicaSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedReplicaSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_namespaced_replica_set_scale`](./struct.Scale.html#method.read_namespaced_replica_set_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedReplicaSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_namespaced_replica_set_scale`](./struct.Scale.html#method.read_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReadNamespacedReplicaSetScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedReplicationControllerDummyScale

impl Scale {
    /// read scale of the specified ReplicationControllerDummy
    ///
    /// Use [`ReadNamespacedReplicationControllerDummyScaleResponse`](./enum.ReadNamespacedReplicationControllerDummyScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_replication_controller_dummy_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedReplicationControllerDummyScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedReplicationControllerDummyScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.read_namespaced_replication_controller_dummy_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedReplicationControllerDummyScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.read_namespaced_replication_controller_dummy_scale)
#[derive(Debug)]
pub enum ReadNamespacedReplicationControllerDummyScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadNamespacedReplicationControllerDummyScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedReplicationControllerDummyScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadNamespacedReplicationControllerDummyScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadNamespacedReplicationControllerDummyScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedDeploymentScale

impl Scale {
    /// replace scale of the specified Deployment
    ///
    /// Use [`ReplaceNamespacedDeploymentScaleResponse`](./enum.ReplaceNamespacedDeploymentScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::extensions::v1beta1::Scale,
        optional: ReplaceNamespacedDeploymentScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_namespaced_deployment_scale`](./struct.Scale.html#method.replace_namespaced_deployment_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_namespaced_deployment_scale`](./struct.Scale.html#method.replace_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReplaceNamespacedDeploymentScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedReplicaSetScale

impl Scale {
    /// replace scale of the specified ReplicaSet
    ///
    /// Use [`ReplaceNamespacedReplicaSetScaleResponse`](./enum.ReplaceNamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::extensions::v1beta1::Scale,
        optional: ReplaceNamespacedReplicaSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedReplicaSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_namespaced_replica_set_scale`](./struct.Scale.html#method.replace_namespaced_replica_set_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedReplicaSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_namespaced_replica_set_scale`](./struct.Scale.html#method.replace_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReplaceNamespacedReplicaSetScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedReplicationControllerDummyScale

impl Scale {
    /// replace scale of the specified ReplicationControllerDummy
    ///
    /// Use [`ReplaceNamespacedReplicationControllerDummyScaleResponse`](./enum.ReplaceNamespacedReplicationControllerDummyScaleResponse.html) to parse the HTTP response.
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
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_replication_controller_dummy_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::extensions::v1beta1::Scale,
        optional: ReplaceNamespacedReplicationControllerDummyScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedReplicationControllerDummyScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.replace_namespaced_replication_controller_dummy_scale)
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedReplicationControllerDummyScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_namespaced_replication_controller_dummy_scale`](./struct.Scale.html#method.replace_namespaced_replication_controller_dummy_scale)
#[derive(Debug)]
pub enum ReplaceNamespacedReplicationControllerDummyScaleResponse {
    Ok(crate::v1_8::api::extensions::v1beta1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceNamespacedReplicationControllerDummyScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedReplicationControllerDummyScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceNamespacedReplicationControllerDummyScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceNamespacedReplicationControllerDummyScaleResponse::Other, 0)),
        }
    }
}

// End extensions/v1beta1/Scale

impl crate::Resource for Scale {
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

impl crate::Metadata for Scale {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Scale {
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
            type Value = Scale;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Scale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_8::api::extensions::v1beta1::ScaleSpec> = None;
                let mut value_status: Option<crate::v1_8::api::extensions::v1beta1::ScaleStatus> = None;

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

impl serde::Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scale",
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
