// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewStatus

/// TokenReviewStatus is the result of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewStatus {
    /// Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is "true", the token is valid against the audience of the Kubernetes API server.
    pub audiences: Vec<String>,

    /// Authenticated indicates that the token was associated with a known user.
    pub authenticated: Option<bool>,

    /// Error indicates that the token couldn't be checked
    pub error: Option<String>,

    /// User is the UserInfo associated with the provided token.
    pub user: Option<crate::api::authentication::v1beta1::UserInfo>,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TokenReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audiences: Option<Vec<String>> = None;
                let mut value_authenticated: Option<bool> = None;
                let mut value_error: Option<String> = None;
                let mut value_user: Option<crate::api::authentication::v1beta1::UserInfo> = None;

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
                    audiences: value_audiences.unwrap_or_default(),
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
            usize::from(!self.audiences.is_empty()) +
            self.authenticated.as_ref().map_or(0, |_| 1) +
            self.error.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if !self.audiences.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "audiences", &self.audiences)?;
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

#[cfg(feature = "schema")]
impl crate::Schema for TokenReviewStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "TokenReviewStatus is the result of the token authentication request.",
          "properties": {
            "audiences": {
              "description": "Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is \"true\", the token is valid against the audience of the Kubernetes API server.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "authenticated": {
              "description": "Authenticated indicates that the token was associated with a known user.",
              "type": "boolean"
            },
            "error": {
              "description": "Error indicates that the token couldn't be checked",
              "type": "string"
            },
            "user": crate::schema_ref_with_values(crate::api::authentication::v1beta1::UserInfo::schema(), serde_json::json!({"description": "User is the UserInfo associated with the provided token."}))
          },
          "type": "object"
        })
    }
}
