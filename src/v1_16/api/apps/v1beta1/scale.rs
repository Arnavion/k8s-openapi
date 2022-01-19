// Generated from definition io.k8s.api.apps.v1beta1.Scale

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    pub spec: Option<crate::api::apps::v1beta1::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<crate::api::apps::v1beta1::ScaleStatus>,
}

// Begin apps/v1beta1/Scale

// Generated from operation patchAppsV1beta1NamespacedDeploymentScale

impl Scale {
    /// partially update scale of the specified Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn patch_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchAppsV1beta1NamespacedStatefulSetScale

impl Scale {
    /// partially update scale of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn patch_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/scale?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation readAppsV1beta1NamespacedDeploymentScale

impl Scale {
    /// read scale of the specified Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedDeploymentScaleResponse`]`>` constructor, or [`ReadNamespacedDeploymentScaleResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn read_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedDeploymentScaleOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadNamespacedDeploymentScaleResponse>), crate::RequestError> {
        let ReadNamespacedDeploymentScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Scale::read_namespaced_deployment_scale`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedDeploymentScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedDeploymentScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::read_namespaced_deployment_scale`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedDeploymentScaleResponse {
    Ok(crate::api::apps::v1beta1::Scale),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedDeploymentScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readAppsV1beta1NamespacedStatefulSetScale

impl Scale {
    /// read scale of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedStatefulSetScaleResponse`]`>` constructor, or [`ReadNamespacedStatefulSetScaleResponse`] directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn read_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedStatefulSetScaleOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadNamespacedStatefulSetScaleResponse>), crate::RequestError> {
        let ReadNamespacedStatefulSetScaleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/scale?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Scale::read_namespaced_stateful_set_scale`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedStatefulSetScaleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedStatefulSetScaleResponse as Response>::try_from_parts` to parse the HTTP response body of [`Scale::read_namespaced_stateful_set_scale`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedStatefulSetScaleResponse {
    Ok(crate::api::apps::v1beta1::Scale),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedStatefulSetScaleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceAppsV1beta1NamespacedDeploymentScale

impl Scale {
    /// replace scale of the specified Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn replace_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &crate::api::apps::v1beta1::Scale,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation replaceAppsV1beta1NamespacedStatefulSetScale

impl Scale {
    /// replace scale of the specified StatefulSet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn replace_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &crate::api::apps::v1beta1::Scale,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/scale?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// End apps/v1beta1/Scale

impl crate::Resource for Scale {
    const API_VERSION: &'static str = "apps/v1beta1";
    const GROUP: &'static str = "apps";
    const KIND: &'static str = "Scale";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "scale";
    type Scope = crate::SubResourceScope;
}

impl crate::Metadata for Scale {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> crate::serde::Deserialize<'de> for Scale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Scale;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::apps::v1beta1::ScaleSpec> = None;
                let mut value_status: Option<crate::api::apps::v1beta1::ScaleStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scale {
                    metadata: value_metadata.unwrap_or_default(),
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
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

impl crate::serde::Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.spec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Scale {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1beta1.Scale".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Scale represents a scaling request for a resource.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "spec".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1beta1::ScaleSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "status".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1beta1::ScaleStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: IntoIterator::into_iter([
                    "metadata",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
