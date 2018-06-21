// Generated from definition io.k8s.api.authorization.v1beta1.LocalSubjectAccessReview

/// LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace. Having a namespace scoped resource makes it much easier to grant namespace scoped policy that includes permissions checking.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalSubjectAccessReview {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec holds information about the request being evaluated.  spec.namespace must be equal to the namespace you made the request against.  If empty, it is defaulted.
    pub spec: ::v1_8::api::authorization::v1beta1::SubjectAccessReviewSpec,

    /// Status is filled in by the server and indicates whether the request is allowed or not
    pub status: Option<::v1_8::api::authorization::v1beta1::SubjectAccessReviewStatus>,
}

// Begin authorization.k8s.io/v1beta1/LocalSubjectAccessReview

// Generated from operation createAuthorizationV1beta1NamespacedLocalSubjectAccessReview

#[derive(Debug)]
pub enum CreateAuthorizationV1beta1NamespacedLocalSubjectAccessReviewResponse<R> where R: ::std::io::Read {
    Ok(::v1_8::api::authorization::v1beta1::LocalSubjectAccessReview),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl LocalSubjectAccessReview {
    /// create a LocalSubjectAccessReview
    pub fn create_authorization_v1beta1_namespaced_local_subject_access_review<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_8::api::authorization::v1beta1::LocalSubjectAccessReview,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateAuthorizationV1beta1NamespacedLocalSubjectAccessReviewResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/authorization.k8s.io/v1beta1/namespaces/{namespace}/localsubjectaccessreviews", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.post(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateAuthorizationV1beta1NamespacedLocalSubjectAccessReviewResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateAuthorizationV1beta1NamespacedLocalSubjectAccessReviewResponse::Unauthorized(response),
            other => CreateAuthorizationV1beta1NamespacedLocalSubjectAccessReviewResponse::Other(other, response),
        })
    }
}

// End authorization.k8s.io/v1beta1/LocalSubjectAccessReview

impl<'de> ::serde::Deserialize<'de> for LocalSubjectAccessReview {
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
            type Value = LocalSubjectAccessReview;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct LocalSubjectAccessReview")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_8::api::authorization::v1beta1::SubjectAccessReviewSpec> = None;
                let mut value_status: Option<::v1_8::api::authorization::v1beta1::SubjectAccessReviewStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LocalSubjectAccessReview {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| ::serde::de::Error::missing_field("spec"))?,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "LocalSubjectAccessReview",
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

impl ::serde::Serialize for LocalSubjectAccessReview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LocalSubjectAccessReview",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1 +
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
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
