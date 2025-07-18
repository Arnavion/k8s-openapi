// Generated from definition io.k8s.api.authentication.v1.TokenReviewSpec

/// TokenReviewSpec is a description of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewSpec {
    /// Audiences is a list of the identifiers that the resource server presented with the token identifies as. Audience-aware token authenticators will verify that the token was intended for at least one of the audiences in this list. If no audiences are provided, the audience will default to the audience of the Kubernetes apiserver.
    pub audiences: Option<std::vec::Vec<std::string::String>>,

    /// Token is the opaque bearer token.
    pub token: Option<std::string::String>,
}

impl crate::DeepMerge for TokenReviewSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.audiences, other.audiences);
        crate::DeepMerge::merge_from(&mut self.token, other.token);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TokenReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audiences,
            Key_token,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TokenReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audiences: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_token: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audiences => value_audiences = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_token => value_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenReviewSpec {
                    audiences: value_audiences,
                    token: value_token,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenReviewSpec",
            &[
                "audiences",
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
            self.audiences.as_ref().map_or(0, |_| 1) +
            self.token.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.audiences {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "audiences", value)?;
        }
        if let Some(value) = &self.token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "token", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TokenReviewSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authentication.v1.TokenReviewSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "TokenReviewSpec is a description of the token authentication request.",
            "type": "object",
            "properties": {
                "audiences": {
                    "description": "Audiences is a list of the identifiers that the resource server presented with the token identifies as. Audience-aware token authenticators will verify that the token was intended for at least one of the audiences in this list. If no audiences are provided, the audience will default to the audience of the Kubernetes apiserver.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "token": {
                    "description": "Token is the opaque bearer token.",
                    "type": "string",
                },
            },
        })
    }
}
