// Generated from definition io.k8s.api.storage.v1.TokenRequest

/// TokenRequest contains parameters of a service account token.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenRequest {
    /// Audience is the intended audience of the token in "TokenRequestSpec". It will default to the audiences of kube apiserver.
    pub audience: String,

    /// ExpirationSeconds is the duration of validity of the token in "TokenRequestSpec". It has the same default value of "ExpirationSeconds" in "TokenRequestSpec".
    pub expiration_seconds: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for TokenRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audience,
            Key_expiration_seconds,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "audience" => Field::Key_audience,
                            "expirationSeconds" => Field::Key_expiration_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TokenRequest;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TokenRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_audience: Option<String> = None;
                let mut value_expiration_seconds: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audience => value_audience = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_expiration_seconds => value_expiration_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenRequest {
                    audience: value_audience.ok_or_else(|| serde::de::Error::missing_field("audience"))?,
                    expiration_seconds: value_expiration_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenRequest",
            &[
                "audience",
                "expirationSeconds",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TokenRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenRequest",
            1 +
            self.expiration_seconds.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "audience", &self.audience)?;
        if let Some(value) = &self.expiration_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "expirationSeconds", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
