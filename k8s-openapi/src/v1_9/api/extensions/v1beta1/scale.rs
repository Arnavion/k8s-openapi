// Generated from definition io.k8s.api.extensions.v1beta1.Scale

/// represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_9::api::extensions::v1beta1::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<::v1_9::api::extensions::v1beta1::ScaleStatus>,
}

// Begin extensions/v1beta1/Scale

// Generated from operation patchExtensionsV1beta1NamespacedDeploymentScale

#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedDeploymentScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// partially update scale of the specified Deployment
    pub fn patch_extensions_v1beta1_namespaced_deployment_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchExtensionsV1beta1NamespacedDeploymentScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchExtensionsV1beta1NamespacedDeploymentScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchExtensionsV1beta1NamespacedDeploymentScaleResponse::Unauthorized(response),
            other => PatchExtensionsV1beta1NamespacedDeploymentScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedReplicaSetScale

#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// partially update scale of the specified ReplicaSet
    pub fn patch_extensions_v1beta1_namespaced_replica_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse::Unauthorized(response),
            other => PatchExtensionsV1beta1NamespacedReplicaSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedReplicationControllerDummyScale

#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// partially update scale of the specified ReplicationControllerDummy
    pub fn patch_extensions_v1beta1_namespaced_replication_controller_dummy_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Unauthorized(response),
            other => PatchExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation readExtensionsV1beta1NamespacedDeploymentScale

#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedDeploymentScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// read scale of the specified Deployment
    pub fn read_extensions_v1beta1_namespaced_deployment_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadExtensionsV1beta1NamespacedDeploymentScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadExtensionsV1beta1NamespacedDeploymentScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadExtensionsV1beta1NamespacedDeploymentScaleResponse::Unauthorized(response),
            other => ReadExtensionsV1beta1NamespacedDeploymentScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation readExtensionsV1beta1NamespacedReplicaSetScale

#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// read scale of the specified ReplicaSet
    pub fn read_extensions_v1beta1_namespaced_replica_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse::Unauthorized(response),
            other => ReadExtensionsV1beta1NamespacedReplicaSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation readExtensionsV1beta1NamespacedReplicationControllerDummyScale

#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// read scale of the specified ReplicationControllerDummy
    pub fn read_extensions_v1beta1_namespaced_replication_controller_dummy_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Unauthorized(response),
            other => ReadExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedDeploymentScale

#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Created(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// replace scale of the specified Deployment
    pub fn replace_extensions_v1beta1_namespaced_deployment_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::extensions::v1beta1::Scale,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Unauthorized(response),
            other => ReplaceExtensionsV1beta1NamespacedDeploymentScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedReplicaSetScale

#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Created(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// replace scale of the specified ReplicaSet
    pub fn replace_extensions_v1beta1_namespaced_replica_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::extensions::v1beta1::Scale,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicasets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Unauthorized(response),
            other => ReplaceExtensionsV1beta1NamespacedReplicaSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedReplicationControllerDummyScale

#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::extensions::v1beta1::Scale),
    Created(::v1_9::api::extensions::v1beta1::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// replace scale of the specified ReplicationControllerDummy
    pub fn replace_extensions_v1beta1_namespaced_replication_controller_dummy_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::extensions::v1beta1::Scale,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/replicationcontrollers/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Unauthorized(response),
            other => ReplaceExtensionsV1beta1NamespacedReplicationControllerDummyScaleResponse::Other(other, response),
        })
    }
}

// End extensions/v1beta1/Scale

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
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_9::api::extensions::v1beta1::ScaleSpec> = None;
                let mut value_status: Option<::v1_9::api::extensions::v1beta1::ScaleStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scale {
                    api_version: value_api_version,
                    kind: value_kind,
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
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
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
