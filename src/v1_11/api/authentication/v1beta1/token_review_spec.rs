// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewSpec

/// TokenReviewSpec is a description of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewSpec {
    /// Token is the opaque bearer token.
    pub token: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for TokenReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_token,
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
                            "token" => Field::Key_token,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TokenReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TokenReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_token: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_token => value_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for TokenReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenReviewSpec",
            self.token.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "token", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for TokenReviewSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "TokenReviewSpec is a description of the token authentication request.",
          "properties": {
            "token": {
              "description": "Token is the opaque bearer token.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
