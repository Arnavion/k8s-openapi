// Generated from definition io.k8s.api.node.v1beta1.RuntimeClass

/// RuntimeClass defines a class of container runtime supported in the cluster. The RuntimeClass is used to determine which container runtime is used to run all containers in a pod. RuntimeClasses are (currently) manually defined by a user or cluster provisioner, and referenced in the PodSpec. The Kubelet is responsible for resolving the RuntimeClassName reference before running the pod.  For more details, see https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuntimeClass {
    /// Handler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called "runc" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable.
    pub handler: String,

    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. For more details, see https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md This field is beta-level as of Kubernetes v1.18, and is only honored by servers that enable the PodOverhead feature.
    pub overhead: Option<crate::api::node::v1beta1::Overhead>,

    /// Scheduling holds the scheduling constraints to ensure that pods running with this RuntimeClass are scheduled to nodes that support it. If scheduling is nil, this RuntimeClass is assumed to be supported by all nodes.
    pub scheduling: Option<crate::api::node::v1beta1::Scheduling>,

}

#[cfg(feature = "dsl")]
impl RuntimeClass  {
    /// Set [`Self::handler`]
    pub  fn handler_set(&mut self, handler: impl Into<String>) -> &mut Self {
        self.handler = handler.into(); self
    }

    pub  fn handler(&mut self) -> &mut String {
        &mut self.handler
    }

    /// Modify [`Self::handler`] with a `func`
    pub  fn handler_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.handler); self
    }


    /// Set [`Self::metadata`]
    pub  fn metadata_set(&mut self, metadata: impl Into<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>) -> &mut Self {
        self.metadata = metadata.into(); self
    }

    pub  fn metadata(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        &mut self.metadata
    }

    /// Modify [`Self::metadata`] with a `func`
    pub  fn metadata_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::ObjectMeta)) -> &mut Self {
        func(&mut self.metadata); self
    }


    /// Set [`Self::overhead`]
    pub  fn overhead_set(&mut self, overhead: impl Into<Option<crate::api::node::v1beta1::Overhead>>) -> &mut Self {
        self.overhead = overhead.into(); self
    }

    pub  fn overhead(&mut self) -> &mut crate::api::node::v1beta1::Overhead {
        if self.overhead.is_none() { self.overhead = Some(Default::default()) }
        self.overhead.as_mut().unwrap()
    }

    /// Modify [`Self::overhead`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn overhead_with(&mut self, func: impl FnOnce(&mut crate::api::node::v1beta1::Overhead)) -> &mut Self {
        if self.overhead.is_none() { self.overhead = Some(Default::default()) };
        func(self.overhead.as_mut().unwrap()); self
    }


    /// Set [`Self::scheduling`]
    pub  fn scheduling_set(&mut self, scheduling: impl Into<Option<crate::api::node::v1beta1::Scheduling>>) -> &mut Self {
        self.scheduling = scheduling.into(); self
    }

    pub  fn scheduling(&mut self) -> &mut crate::api::node::v1beta1::Scheduling {
        if self.scheduling.is_none() { self.scheduling = Some(Default::default()) }
        self.scheduling.as_mut().unwrap()
    }

    /// Modify [`Self::scheduling`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scheduling_with(&mut self, func: impl FnOnce(&mut crate::api::node::v1beta1::Scheduling)) -> &mut Self {
        if self.scheduling.is_none() { self.scheduling = Some(Default::default()) };
        func(self.scheduling.as_mut().unwrap()); self
    }


}


// Begin node.k8s.io/v1beta1/RuntimeClass

// Generated from operation createNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// create a RuntimeClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::CreateResponse`]`<Self>>` constructor, or [`crate::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create(
        body: &crate::api::node::v1beta1::RuntimeClass,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::CreateResponse<Self>>), crate::RequestError> {
        let __url = "/apis/node.k8s.io/v1beta1/runtimeclasses?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteNodeV1beta1CollectionRuntimeClass

impl RuntimeClass {
    /// delete collection of RuntimeClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>>` constructor, or [`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection(
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<crate::List<Self>>>), crate::RequestError> {
        let __url = "/apis/node.k8s.io/v1beta1/runtimeclasses?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = crate::serde_json::to_vec(&delete_optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// delete a RuntimeClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<Self>>` constructor, or [`crate::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RuntimeClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete(
        name: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/node.k8s.io/v1beta1/runtimeclasses/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::delete(__url);
        let __body = crate::serde_json::to_vec(&optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation listNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// list or watch objects of kind RuntimeClass
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ListResponse`]`<Self>>` constructor, or [`crate::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list(
        optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = "/apis/node.k8s.io/v1beta1/runtimeclasses?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// partially update the specified RuntimeClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RuntimeClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/node.k8s.io/v1beta1/runtimeclasses/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation readNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// read the specified RuntimeClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadRuntimeClassResponse`]`>` constructor, or [`ReadRuntimeClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RuntimeClass
    #[cfg(feature = "api")]
    pub fn read(
        name: &str,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadRuntimeClassResponse>), crate::RequestError> {
        let __url = format!("/apis/node.k8s.io/v1beta1/runtimeclasses/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ReadRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::read`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadRuntimeClassResponse {
    Ok(crate::api::node::v1beta1::RuntimeClass),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadRuntimeClassResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadRuntimeClassResponse::Ok(result), buf.len()))
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
                Ok((ReadRuntimeClassResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// replace the specified RuntimeClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the RuntimeClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace(
        name: &str,
        body: &crate::api::node::v1beta1::RuntimeClass,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/node.k8s.io/v1beta1/runtimeclasses/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation watchNodeV1beta1RuntimeClass

impl RuntimeClass {
    /// list or watch objects of kind RuntimeClass
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::WatchResponse`]`<Self>>` constructor, or [`crate::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = "/apis/node.k8s.io/v1beta1/runtimeclasses?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// End node.k8s.io/v1beta1/RuntimeClass

impl crate::Resource for RuntimeClass {
    const API_VERSION: &'static str = "node.k8s.io/v1beta1";
    const GROUP: &'static str = "node.k8s.io";
    const KIND: &'static str = "RuntimeClass";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "runtimeclasses";
    type Scope = crate::ClusterResourceScope;
}

impl crate::ListableResource for RuntimeClass {
    const LIST_KIND: &'static str = "RuntimeClassList";
}

impl crate::Metadata for RuntimeClass {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> crate::serde::Deserialize<'de> for RuntimeClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_handler,
            Key_metadata,
            Key_overhead,
            Key_scheduling,
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
                            "handler" => Field::Key_handler,
                            "metadata" => Field::Key_metadata,
                            "overhead" => Field::Key_overhead,
                            "scheduling" => Field::Key_scheduling,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RuntimeClass;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_handler: Option<String> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_overhead: Option<crate::api::node::v1beta1::Overhead> = None;
                let mut value_scheduling: Option<crate::api::node::v1beta1::Scheduling> = None;

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
                        Field::Key_handler => value_handler = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_overhead => value_overhead = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling => value_scheduling = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RuntimeClass {
                    handler: value_handler.unwrap_or_default(),
                    metadata: value_metadata.unwrap_or_default(),
                    overhead: value_overhead,
                    scheduling: value_scheduling,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "handler",
                "metadata",
                "overhead",
                "scheduling",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RuntimeClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            self.overhead.as_ref().map_or(0, |_| 1) +
            self.scheduling.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "handler", &self.handler)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.overhead {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "overhead", value)?;
        }
        if let Some(value) = &self.scheduling {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scheduling", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RuntimeClass {
    fn schema_name() -> String {
        "io.k8s.api.node.v1beta1.RuntimeClass".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RuntimeClass defines a class of container runtime supported in the cluster. The RuntimeClass is used to determine which container runtime is used to run all containers in a pod. RuntimeClasses are (currently) manually defined by a user or cluster provisioner, and referenced in the PodSpec. The Kubelet is responsible for resolving the RuntimeClassName reference before running the pod.  For more details, see https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
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
                        "handler".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Handler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called \"runc\" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable.".to_owned()),
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
                                description: Some("More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "overhead".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::node::v1beta1::Overhead>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. For more details, see https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md This field is beta-level as of Kubernetes v1.18, and is only honored by servers that enable the PodOverhead feature.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scheduling".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::node::v1beta1::Scheduling>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Scheduling holds the scheduling constraints to ensure that pods running with this RuntimeClass are scheduled to nodes that support it. If scheduling is nil, this RuntimeClass is assumed to be supported by all nodes.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "handler".to_owned(),
                    "metadata".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
