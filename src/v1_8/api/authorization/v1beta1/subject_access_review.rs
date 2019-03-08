// Generated from definition io.k8s.api.authorization.v1beta1.SubjectAccessReview

/// SubjectAccessReview checks whether or not a user or group can perform an action.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectAccessReview {
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec holds information about the request being evaluated
    pub spec: crate::v1_8::api::authorization::v1beta1::SubjectAccessReviewSpec,

    /// Status is filled in by the server and indicates whether the request is allowed or not
    pub status: Option<crate::v1_8::api::authorization::v1beta1::SubjectAccessReviewStatus>,
}

// Begin authorization.k8s.io/v1beta1/SubjectAccessReview

// Generated from operation createAuthorizationV1beta1SubjectAccessReview

impl SubjectAccessReview {
    /// create a SubjectAccessReview
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateSubjectAccessReviewResponse`]`>` constructor, or [`CreateSubjectAccessReviewResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_subject_access_review(
        body: &crate::v1_8::api::authorization::v1beta1::SubjectAccessReview,
        optional: CreateSubjectAccessReviewOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateSubjectAccessReviewResponse>), crate::RequestError> {
        let CreateSubjectAccessReviewOptional {
            pretty,
        } = optional;
        let __url = "/apis/authorization.k8s.io/v1beta1/subjectaccessreviews?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`SubjectAccessReview::create_subject_access_review`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateSubjectAccessReviewOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateSubjectAccessReviewResponse as Response>::try_from_parts` to parse the HTTP response body of [`SubjectAccessReview::create_subject_access_review`]
#[derive(Debug)]
pub enum CreateSubjectAccessReviewResponse {
    Ok(crate::v1_8::api::authorization::v1beta1::SubjectAccessReview),
    Unauthorized,
    Other,
}

impl crate::Response for CreateSubjectAccessReviewResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubjectAccessReviewResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateSubjectAccessReviewResponse::Unauthorized, 0)),
            _ => Ok((CreateSubjectAccessReviewResponse::Other, 0)),
        }
    }
}

// End authorization.k8s.io/v1beta1/SubjectAccessReview

impl crate::Resource for SubjectAccessReview {
    fn api_version() -> &'static str {
        "authorization.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "SubjectAccessReview"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for SubjectAccessReview {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for SubjectAccessReview {
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
            type Value = SubjectAccessReview;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct SubjectAccessReview")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_8::api::authorization::v1beta1::SubjectAccessReviewSpec> = None;
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

                Ok(SubjectAccessReview {
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| serde::de::Error::missing_field("spec"))?,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectAccessReview",
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

impl serde::Serialize for SubjectAccessReview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectAccessReview",
            3 +
            self.metadata.as_ref().map_or(0, |_| 1) +
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
