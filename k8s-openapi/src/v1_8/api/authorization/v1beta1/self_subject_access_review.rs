// Generated from definition io.k8s.api.authorization.v1beta1.SelfSubjectAccessReview

/// SelfSubjectAccessReview checks whether or the current user can perform an action.  Not filling in a spec.namespace means "in all namespaces".  Self is a special case, because users should always be able to check whether they can perform an action
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectAccessReview {
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec holds information about the request being evaluated.  user and groups must be empty
    pub spec: crate::v1_8::api::authorization::v1beta1::SelfSubjectAccessReviewSpec,

    /// Status is filled in by the server and indicates whether the request is allowed or not
    pub status: Option<crate::v1_8::api::authorization::v1beta1::SubjectAccessReviewStatus>,
}

// Begin authorization.k8s.io/v1beta1/SelfSubjectAccessReview

// Generated from operation createAuthorizationV1beta1SelfSubjectAccessReview

impl SelfSubjectAccessReview {
    /// create a SelfSubjectAccessReview
    ///
    /// Use [`CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse`](./enum.CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_authorization_v1beta1_self_subject_access_review(
        body: &crate::v1_8::api::authorization::v1beta1::SelfSubjectAccessReview,
        pretty: Option<&str>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let __url = format!("/apis/authorization.k8s.io/v1beta1/selfsubjectaccessreviews?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Parses the HTTP response of [`SelfSubjectAccessReview::create_authorization_v1beta1_self_subject_access_review`](./struct.SelfSubjectAccessReview.html#method.create_authorization_v1beta1_self_subject_access_review)
#[derive(Debug)]
pub enum CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse {
    Ok(crate::v1_8::api::authorization::v1beta1::SelfSubjectAccessReview),
    Unauthorized,
    Other,
}

impl crate::Response for CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse::Unauthorized, 0)),
            _ => Ok((CreateAuthorizationV1beta1SelfSubjectAccessReviewResponse::Other, 0)),
        }
    }
}

// End authorization.k8s.io/v1beta1/SelfSubjectAccessReview

impl crate::Resource for SelfSubjectAccessReview {
    fn api_version() -> &'static str {
        "authorization.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "SelfSubjectAccessReview"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for SelfSubjectAccessReview {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&Self::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for SelfSubjectAccessReview {
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
            type Value = SelfSubjectAccessReview;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct SelfSubjectAccessReview")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_8::api::authorization::v1beta1::SelfSubjectAccessReviewSpec> = None;
                let mut value_status: Option<crate::v1_8::api::authorization::v1beta1::SubjectAccessReviewStatus> = None;

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

                Ok(SelfSubjectAccessReview {
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| serde::de::Error::missing_field("spec"))?,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectAccessReview",
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

impl serde::Serialize for SelfSubjectAccessReview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectAccessReview",
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
