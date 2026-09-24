// Generated from definition io.k8s.api.authentication.v1.TokenReviewStatus

/// TokenReviewStatus is the result of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewStatus {
    /// Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is "true", the token is valid against the audience of the Kubernetes API server.
    pub audiences: Option<std::vec::Vec<std::string::String>>,

    /// Authenticated indicates that the token was associated with a known user.
    pub authenticated: Option<bool>,

    /// Error indicates that the token couldn't be checked
    pub error: Option<std::string::String>,

    /// User is the UserInfo associated with the provided token.
    pub user: Option<crate::api::authentication::v1::UserInfo>,
}

impl crate::DeepMerge for TokenReviewStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.audiences, other.audiences);
        crate::DeepMerge::merge_from(&mut self.authenticated, other.authenticated);
        crate::DeepMerge::merge_from(&mut self.error, other.error);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TokenReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audiences,
            Key_authenticated,
            Key_error,
            Key_user,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "audiences" => Field::Key_audiences,
                            "authenticated" => Field::Key_authenticated,
                            "error" => Field::Key_error,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TokenReviewStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TokenReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audiences: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_authenticated: Option<bool> = None;
                let mut value_error: Option<std::string::String> = None;
                let mut value_user: Option<crate::api::authentication::v1::UserInfo> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audiences => value_audiences = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_authenticated => value_authenticated = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_error => value_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenReviewStatus {
                    audiences: value_audiences,
                    authenticated: value_authenticated,
                    error: value_error,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenReviewStatus",
            &[
                "audiences",
                "authenticated",
                "error",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TokenReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenReviewStatus",
            self.audiences.as_ref().map_or(0, |_| 1) +
            self.authenticated.as_ref().map_or(0, |_| 1) +
            self.error.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.audiences {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "audiences", value)?;
        }
        if let Some(value) = &self.authenticated {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "authenticated", value)?;
        }
        if let Some(value) = &self.error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "error", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TokenReviewStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authentication.v1.TokenReviewStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "TokenReviewStatus is the result of the token authentication request.",
            "type": "object",
            "properties": {
                "audiences": {
                    "description": "Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is \"true\", the token is valid against the audience of the Kubernetes API server.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "authenticated": {
                    "description": "Authenticated indicates that the token was associated with a known user.",
                    "type": "boolean",
                },
                "error": {
                    "description": "Error indicates that the token couldn't be checked",
                    "type": "string",
                },
                "user": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::authentication::v1::UserInfo>();
                    schema_obj.ensure_object().insert("description".into(), "User is the UserInfo associated with the provided token.".into());
                    schema_obj
                }),
            },
        })
    }
}
