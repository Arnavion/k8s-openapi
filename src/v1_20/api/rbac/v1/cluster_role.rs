// Generated from definition io.k8s.api.rbac.v1.ClusterRole

/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRole {
    /// AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
    pub aggregation_rule: Option<crate::api::rbac::v1::AggregationRule>,

    /// Standard object's metadata.
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Rules holds all the PolicyRules for this ClusterRole
    pub rules: Option<Vec<crate::api::rbac::v1::PolicyRule>>,
}

// Begin rbac.authorization.k8s.io/v1/ClusterRole

// Generated from operation createRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// create a ClusterRole
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
    pub fn create_cluster_role(
        body: &crate::api::rbac::v1::ClusterRole,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::CreateResponse<Self>>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1/clusterroles?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// delete a ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<Self>>` constructor, or [`crate::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_cluster_role(
        name: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/clusterroles/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1CollectionClusterRole

impl ClusterRole {
    /// delete collection of ClusterRole
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
    pub fn delete_collection_cluster_role(
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<crate::List<Self>>>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1/clusterroles?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&delete_optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// list or watch objects of kind ClusterRole
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
    pub fn list_cluster_role(
        optional: crate::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1/clusterroles?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// partially update the specified ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_cluster_role(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/clusterroles/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
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

// Generated from operation readRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// read the specified ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadClusterRoleResponse`]`>` constructor, or [`ReadClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_cluster_role(
        name: &str,
        optional: ReadClusterRoleOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadClusterRoleResponse>), crate::RequestError> {
        let ReadClusterRoleOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/clusterroles/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ClusterRole::read_cluster_role`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadClusterRoleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::read_cluster_role`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadClusterRoleResponse {
    Ok(crate::api::rbac::v1::ClusterRole),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadClusterRoleResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadClusterRoleResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadClusterRoleResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// replace the specified ClusterRole
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_cluster_role(
        name: &str,
        body: &crate::api::rbac::v1::ClusterRole,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1/clusterroles/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1ClusterRole

impl ClusterRole {
    /// list or watch objects of kind ClusterRole
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
    pub fn watch_cluster_role(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = "/apis/rbac.authorization.k8s.io/v1/clusterroles?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// End rbac.authorization.k8s.io/v1/ClusterRole

impl crate::Resource for ClusterRole {
    const API_VERSION: &'static str = "rbac.authorization.k8s.io/v1";
    const GROUP: &'static str = "rbac.authorization.k8s.io";
    const KIND: &'static str = "ClusterRole";
    const VERSION: &'static str = "v1";
}

impl crate::ListableResource for ClusterRole {
    const LIST_KIND: &'static str = concat!("ClusterRole", "List");
}

impl crate::Metadata for ClusterRole {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for ClusterRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_aggregation_rule,
            Key_metadata,
            Key_rules,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "aggregationRule" => Field::Key_aggregation_rule,
                            "metadata" => Field::Key_metadata,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRole;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_aggregation_rule: Option<crate::api::rbac::v1::AggregationRule> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_rules: Option<Vec<crate::api::rbac::v1::PolicyRule>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_aggregation_rule => value_aggregation_rule = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rules => value_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRole {
                    aggregation_rule: value_aggregation_rule,
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    rules: value_rules,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "aggregationRule",
                "metadata",
                "rules",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.aggregation_rule.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.aggregation_rule {
            serde::ser::SerializeStruct::serialize_field(&mut state, "aggregationRule", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
