// Generated from definition io.k8s.api.authorization.v1.SelfSubjectRulesReview

/// SelfSubjectRulesReview enumerates the set of actions the current user can perform within a namespace. The returned list of actions may be incomplete depending on the server's authorization mode, and any errors experienced during the evaluation. SelfSubjectRulesReview should be used by UIs to show/hide actions, or to quickly let an end user reason about their permissions. It should NOT Be used by external systems to drive authorization decisions as this raises confused deputy, cache lifetime/revocation, and correctness concerns. SubjectAccessReview, and LocalAccessReview are the correct way to defer authorization decisions to the API server.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectRulesReview {
    pub metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec holds information about the request being evaluated.
    pub spec: crate::v1_12::api::authorization::v1::SelfSubjectRulesReviewSpec,

    /// Status is filled in by the server and indicates the set of actions a user can perform.
    pub status: Option<crate::v1_12::api::authorization::v1::SubjectRulesReviewStatus>,
}

// Begin authorization.k8s.io/v1/SelfSubjectRulesReview

// Generated from operation createAuthorizationV1SelfSubjectRulesReview

impl SelfSubjectRulesReview {
    /// create a SelfSubjectRulesReview
    ///
    /// Use [`CreateAuthorizationV1SelfSubjectRulesReviewResponse`](./enum.CreateAuthorizationV1SelfSubjectRulesReviewResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_authorization_v1_self_subject_rules_review(
        body: &crate::v1_12::api::authorization::v1::SelfSubjectRulesReview,
        optional: CreateAuthorizationV1SelfSubjectRulesReviewOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateAuthorizationV1SelfSubjectRulesReviewOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/authorization.k8s.io/v1/selfsubjectrulesreviews?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`SelfSubjectRulesReview::create_authorization_v1_self_subject_rules_review`](./struct.SelfSubjectRulesReview.html#method.create_authorization_v1_self_subject_rules_review)
#[derive(Debug, Default)]
pub struct CreateAuthorizationV1SelfSubjectRulesReviewOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// If IncludeUninitialized is specified, the object may be returned without completing initialization.
    pub include_uninitialized: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`SelfSubjectRulesReview::create_authorization_v1_self_subject_rules_review`](./struct.SelfSubjectRulesReview.html#method.create_authorization_v1_self_subject_rules_review)
#[derive(Debug)]
pub enum CreateAuthorizationV1SelfSubjectRulesReviewResponse {
    Ok(crate::v1_12::api::authorization::v1::SelfSubjectRulesReview),
    Created(crate::v1_12::api::authorization::v1::SelfSubjectRulesReview),
    Accepted(crate::v1_12::api::authorization::v1::SelfSubjectRulesReview),
    Unauthorized,
    Other,
}

impl crate::Response for CreateAuthorizationV1SelfSubjectRulesReviewResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAuthorizationV1SelfSubjectRulesReviewResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAuthorizationV1SelfSubjectRulesReviewResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAuthorizationV1SelfSubjectRulesReviewResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateAuthorizationV1SelfSubjectRulesReviewResponse::Unauthorized, 0)),
            _ => Ok((CreateAuthorizationV1SelfSubjectRulesReviewResponse::Other, 0)),
        }
    }
}

// End authorization.k8s.io/v1/SelfSubjectRulesReview

impl crate::Resource for SelfSubjectRulesReview {
    fn api_version() -> &'static str {
        "authorization.k8s.io/v1"
    }

    fn group() -> &'static str {
        "authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "SelfSubjectRulesReview"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for SelfSubjectRulesReview {
    type Ty = crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for SelfSubjectRulesReview {
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
            type Value = SelfSubjectRulesReview;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct SelfSubjectRulesReview")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_12::api::authorization::v1::SelfSubjectRulesReviewSpec> = None;
                let mut value_status: Option<crate::v1_12::api::authorization::v1::SubjectRulesReviewStatus> = None;

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
                        Field::Key_spec => value_spec = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SelfSubjectRulesReview {
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| serde::de::Error::missing_field("spec"))?,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectRulesReview",
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

impl serde::Serialize for SelfSubjectRulesReview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectRulesReview",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1 +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
