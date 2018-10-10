// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewStatus

/// TokenReviewStatus is the result of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewStatus {
    /// Authenticated indicates that the token was associated with a known user.
    pub authenticated: Option<bool>,

    /// Error indicates that the token couldn't be checked
    pub error: Option<String>,

    /// User is the UserInfo associated with the provided token.
    pub user: Option<::v1_12::api::authentication::v1beta1::UserInfo>,
}

impl<'de> ::serde::Deserialize<'de> for TokenReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_authenticated,
            Key_error,
            Key_user,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TokenReviewStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct TokenReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_authenticated: Option<bool> = None;
                let mut value_error: Option<String> = None;
                let mut value_user: Option<::v1_12::api::authentication::v1beta1::UserInfo> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_authenticated => value_authenticated = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_error => value_error = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenReviewStatus {
                    authenticated: value_authenticated,
                    error: value_error,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenReviewStatus",
            &[
                "authenticated",
                "error",
                "user",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for TokenReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenReviewStatus",
            0 +
            self.authenticated.as_ref().map_or(0, |_| 1) +
            self.error.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.authenticated {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "authenticated", value)?;
        }
        if let Some(value) = &self.error {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "error", value)?;
        }
        if let Some(value) = &self.user {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
