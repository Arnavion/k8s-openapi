// Generated from definition io.k8s.api.autoscaling.v1.Scale

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<crate::v1_10::api::autoscaling::v1::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<crate::v1_10::api::autoscaling::v1::ScaleStatus>,
}

// Begin autoscaling/v1/Scale

// Generated from operation patchAppsV1NamespacedDeploymentScale

impl Scale {
    /// partially update scale of the specified Deployment
    ///
    /// Use [`PatchAppsV1NamespacedDeploymentScaleResponse`](./enum.PatchAppsV1NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
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
    pub fn patch_apps_v1_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchAppsV1NamespacedDeploymentScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchAppsV1NamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_apps_v1_namespaced_deployment_scale`](./struct.Scale.html#method.patch_apps_v1_namespaced_deployment_scale)
#[derive(Debug, Default)]
pub struct PatchAppsV1NamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_apps_v1_namespaced_deployment_scale`](./struct.Scale.html#method.patch_apps_v1_namespaced_deployment_scale)
#[derive(Debug)]
pub enum PatchAppsV1NamespacedDeploymentScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchAppsV1NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchAppsV1NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchAppsV1NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchAppsV1NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAppsV1NamespacedReplicaSetScale

impl Scale {
    /// partially update scale of the specified ReplicaSet
    ///
    /// Use [`PatchAppsV1NamespacedReplicaSetScaleResponse`](./enum.PatchAppsV1NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
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
    pub fn patch_apps_v1_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchAppsV1NamespacedReplicaSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchAppsV1NamespacedReplicaSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_apps_v1_namespaced_replica_set_scale`](./struct.Scale.html#method.patch_apps_v1_namespaced_replica_set_scale)
#[derive(Debug, Default)]
pub struct PatchAppsV1NamespacedReplicaSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_apps_v1_namespaced_replica_set_scale`](./struct.Scale.html#method.patch_apps_v1_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum PatchAppsV1NamespacedReplicaSetScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchAppsV1NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchAppsV1NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchAppsV1NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchAppsV1NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAppsV1NamespacedStatefulSetScale

impl Scale {
    /// partially update scale of the specified StatefulSet
    ///
    /// Use [`PatchAppsV1NamespacedStatefulSetScaleResponse`](./enum.PatchAppsV1NamespacedStatefulSetScaleResponse.html) to parse the HTTP response.
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
    pub fn patch_apps_v1_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchAppsV1NamespacedStatefulSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchAppsV1NamespacedStatefulSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_apps_v1_namespaced_stateful_set_scale`](./struct.Scale.html#method.patch_apps_v1_namespaced_stateful_set_scale)
#[derive(Debug, Default)]
pub struct PatchAppsV1NamespacedStatefulSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_apps_v1_namespaced_stateful_set_scale`](./struct.Scale.html#method.patch_apps_v1_namespaced_stateful_set_scale)
#[derive(Debug)]
pub enum PatchAppsV1NamespacedStatefulSetScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchAppsV1NamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchAppsV1NamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchAppsV1NamespacedStatefulSetScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchAppsV1NamespacedStatefulSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedReplicationControllerScale

impl Scale {
    /// partially update scale of the specified ReplicationController
    ///
    /// Use [`PatchCoreV1NamespacedReplicationControllerScaleResponse`](./enum.PatchCoreV1NamespacedReplicationControllerScaleResponse.html) to parse the HTTP response.
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
    pub fn patch_core_v1_namespaced_replication_controller_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchCoreV1NamespacedReplicationControllerScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchCoreV1NamespacedReplicationControllerScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::patch_core_v1_namespaced_replication_controller_scale`](./struct.Scale.html#method.patch_core_v1_namespaced_replication_controller_scale)
#[derive(Debug, Default)]
pub struct PatchCoreV1NamespacedReplicationControllerScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::patch_core_v1_namespaced_replication_controller_scale`](./struct.Scale.html#method.patch_core_v1_namespaced_replication_controller_scale)
#[derive(Debug)]
pub enum PatchCoreV1NamespacedReplicationControllerScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCoreV1NamespacedReplicationControllerScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCoreV1NamespacedReplicationControllerScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCoreV1NamespacedReplicationControllerScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchCoreV1NamespacedReplicationControllerScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1NamespacedDeploymentScale

impl Scale {
    /// read scale of the specified Deployment
    ///
    /// Use [`ReadAppsV1NamespacedDeploymentScaleResponse`](./enum.ReadAppsV1NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
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
    pub fn read_apps_v1_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        optional: ReadAppsV1NamespacedDeploymentScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadAppsV1NamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_apps_v1_namespaced_deployment_scale`](./struct.Scale.html#method.read_apps_v1_namespaced_deployment_scale)
#[derive(Debug, Default)]
pub struct ReadAppsV1NamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_apps_v1_namespaced_deployment_scale`](./struct.Scale.html#method.read_apps_v1_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReadAppsV1NamespacedDeploymentScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadAppsV1NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadAppsV1NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadAppsV1NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadAppsV1NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1NamespacedReplicaSetScale

impl Scale {
    /// read scale of the specified ReplicaSet
    ///
    /// Use [`ReadAppsV1NamespacedReplicaSetScaleResponse`](./enum.ReadAppsV1NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
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
    pub fn read_apps_v1_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        optional: ReadAppsV1NamespacedReplicaSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadAppsV1NamespacedReplicaSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_apps_v1_namespaced_replica_set_scale`](./struct.Scale.html#method.read_apps_v1_namespaced_replica_set_scale)
#[derive(Debug, Default)]
pub struct ReadAppsV1NamespacedReplicaSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_apps_v1_namespaced_replica_set_scale`](./struct.Scale.html#method.read_apps_v1_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReadAppsV1NamespacedReplicaSetScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadAppsV1NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadAppsV1NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadAppsV1NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadAppsV1NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1NamespacedStatefulSetScale

impl Scale {
    /// read scale of the specified StatefulSet
    ///
    /// Use [`ReadAppsV1NamespacedStatefulSetScaleResponse`](./enum.ReadAppsV1NamespacedStatefulSetScaleResponse.html) to parse the HTTP response.
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
    pub fn read_apps_v1_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        optional: ReadAppsV1NamespacedStatefulSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadAppsV1NamespacedStatefulSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_apps_v1_namespaced_stateful_set_scale`](./struct.Scale.html#method.read_apps_v1_namespaced_stateful_set_scale)
#[derive(Debug, Default)]
pub struct ReadAppsV1NamespacedStatefulSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_apps_v1_namespaced_stateful_set_scale`](./struct.Scale.html#method.read_apps_v1_namespaced_stateful_set_scale)
#[derive(Debug)]
pub enum ReadAppsV1NamespacedStatefulSetScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadAppsV1NamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadAppsV1NamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadAppsV1NamespacedStatefulSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadAppsV1NamespacedStatefulSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1NamespacedReplicationControllerScale

impl Scale {
    /// read scale of the specified ReplicationController
    ///
    /// Use [`ReadCoreV1NamespacedReplicationControllerScaleResponse`](./enum.ReadCoreV1NamespacedReplicationControllerScaleResponse.html) to parse the HTTP response.
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
    pub fn read_core_v1_namespaced_replication_controller_scale(
        name: &str,
        namespace: &str,
        optional: ReadCoreV1NamespacedReplicationControllerScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadCoreV1NamespacedReplicationControllerScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::read_core_v1_namespaced_replication_controller_scale`](./struct.Scale.html#method.read_core_v1_namespaced_replication_controller_scale)
#[derive(Debug, Default)]
pub struct ReadCoreV1NamespacedReplicationControllerScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::read_core_v1_namespaced_replication_controller_scale`](./struct.Scale.html#method.read_core_v1_namespaced_replication_controller_scale)
#[derive(Debug)]
pub enum ReadCoreV1NamespacedReplicationControllerScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1NamespacedReplicationControllerScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1NamespacedReplicationControllerScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1NamespacedReplicationControllerScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1NamespacedReplicationControllerScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1NamespacedDeploymentScale

impl Scale {
    /// replace scale of the specified Deployment
    ///
    /// Use [`ReplaceAppsV1NamespacedDeploymentScaleResponse`](./enum.ReplaceAppsV1NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
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
    pub fn replace_apps_v1_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::api::autoscaling::v1::Scale,
        optional: ReplaceAppsV1NamespacedDeploymentScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceAppsV1NamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_apps_v1_namespaced_deployment_scale`](./struct.Scale.html#method.replace_apps_v1_namespaced_deployment_scale)
#[derive(Debug, Default)]
pub struct ReplaceAppsV1NamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_apps_v1_namespaced_deployment_scale`](./struct.Scale.html#method.replace_apps_v1_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReplaceAppsV1NamespacedDeploymentScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Created(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceAppsV1NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1NamespacedDeploymentScaleResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceAppsV1NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAppsV1NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1NamespacedReplicaSetScale

impl Scale {
    /// replace scale of the specified ReplicaSet
    ///
    /// Use [`ReplaceAppsV1NamespacedReplicaSetScaleResponse`](./enum.ReplaceAppsV1NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
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
    pub fn replace_apps_v1_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::api::autoscaling::v1::Scale,
        optional: ReplaceAppsV1NamespacedReplicaSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceAppsV1NamespacedReplicaSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_apps_v1_namespaced_replica_set_scale`](./struct.Scale.html#method.replace_apps_v1_namespaced_replica_set_scale)
#[derive(Debug, Default)]
pub struct ReplaceAppsV1NamespacedReplicaSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_apps_v1_namespaced_replica_set_scale`](./struct.Scale.html#method.replace_apps_v1_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReplaceAppsV1NamespacedReplicaSetScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Created(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceAppsV1NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1NamespacedReplicaSetScaleResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceAppsV1NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAppsV1NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1NamespacedStatefulSetScale

impl Scale {
    /// replace scale of the specified StatefulSet
    ///
    /// Use [`ReplaceAppsV1NamespacedStatefulSetScaleResponse`](./enum.ReplaceAppsV1NamespacedStatefulSetScaleResponse.html) to parse the HTTP response.
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
    pub fn replace_apps_v1_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::api::autoscaling::v1::Scale,
        optional: ReplaceAppsV1NamespacedStatefulSetScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceAppsV1NamespacedStatefulSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_apps_v1_namespaced_stateful_set_scale`](./struct.Scale.html#method.replace_apps_v1_namespaced_stateful_set_scale)
#[derive(Debug, Default)]
pub struct ReplaceAppsV1NamespacedStatefulSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_apps_v1_namespaced_stateful_set_scale`](./struct.Scale.html#method.replace_apps_v1_namespaced_stateful_set_scale)
#[derive(Debug)]
pub enum ReplaceAppsV1NamespacedStatefulSetScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Created(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceAppsV1NamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1NamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1NamespacedStatefulSetScaleResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceAppsV1NamespacedStatefulSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAppsV1NamespacedStatefulSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedReplicationControllerScale

impl Scale {
    /// replace scale of the specified ReplicationController
    ///
    /// Use [`ReplaceCoreV1NamespacedReplicationControllerScaleResponse`](./enum.ReplaceCoreV1NamespacedReplicationControllerScaleResponse.html) to parse the HTTP response.
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
    pub fn replace_core_v1_namespaced_replication_controller_scale(
        name: &str,
        namespace: &str,
        body: &crate::v1_10::api::autoscaling::v1::Scale,
        optional: ReplaceCoreV1NamespacedReplicationControllerScaleOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceCoreV1NamespacedReplicationControllerScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/replicationcontrollers/{name}/scale?", name = name, namespace = namespace);
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

/// Optional parameters of [`Scale::replace_core_v1_namespaced_replication_controller_scale`](./struct.Scale.html#method.replace_core_v1_namespaced_replication_controller_scale)
#[derive(Debug, Default)]
pub struct ReplaceCoreV1NamespacedReplicationControllerScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Scale::replace_core_v1_namespaced_replication_controller_scale`](./struct.Scale.html#method.replace_core_v1_namespaced_replication_controller_scale)
#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedReplicationControllerScaleResponse {
    Ok(crate::v1_10::api::autoscaling::v1::Scale),
    Created(crate::v1_10::api::autoscaling::v1::Scale),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCoreV1NamespacedReplicationControllerScaleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedReplicationControllerScaleResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCoreV1NamespacedReplicationControllerScaleResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCoreV1NamespacedReplicationControllerScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCoreV1NamespacedReplicationControllerScaleResponse::Other, 0)),
        }
    }
}

// End autoscaling/v1/Scale

impl crate::Resource for Scale {
    fn api_version() -> &'static str {
        "autoscaling/v1"
    }

    fn group() -> &'static str {
        "autoscaling"
    }

    fn kind() -> &'static str {
        "Scale"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for Scale {
    type Ty = crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta;

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
                let mut value_metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_10::api::autoscaling::v1::ScaleSpec> = None;
                let mut value_status: Option<crate::v1_10::api::autoscaling::v1::ScaleStatus> = None;

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
            0 +
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
