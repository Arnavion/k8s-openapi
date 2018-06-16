// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewSpec

/// TokenReviewSpec is a description of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewSpec {
    /// Token is the opaque bearer token.
    pub token: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for TokenReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_token,
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
                            "token" => Field::Key_token,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TokenReviewSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct TokenReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_token: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_token => value_token = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenReviewSpec {
                    token: value_token,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenReviewSpec",
            &[
                "token",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for TokenReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenReviewSpec",
            0 +
            self.token.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.token {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "token", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
