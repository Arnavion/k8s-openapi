// Generated from definition io.k8s.api.authentication.v1.TokenRequestStatus

/// TokenRequestStatus is the result of a token request.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenRequestStatus {
    /// ExpirationTimestamp is the time of expiration of the returned token.
    pub expiration_timestamp: crate::apimachinery::pkg::apis::meta::v1::Time,

    /// Token is the opaque bearer token.
    pub token: String,
}

impl<'de> serde::Deserialize<'de> for TokenRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expiration_timestamp,
            Key_token,
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
                            "expirationTimestamp" => Field::Key_expiration_timestamp,
                            "token" => Field::Key_token,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TokenRequestStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TokenRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_expiration_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_token: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expiration_timestamp => value_expiration_timestamp = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_token => value_token = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenRequestStatus {
                    expiration_timestamp: value_expiration_timestamp.ok_or_else(|| serde::de::Error::missing_field("expirationTimestamp"))?,
                    token: value_token.ok_or_else(|| serde::de::Error::missing_field("token"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TokenRequestStatus",
            &[
                "expirationTimestamp",
                "token",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TokenRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenRequestStatus",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "expirationTimestamp", &self.expiration_timestamp)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "token", &self.token)?;
        serde::ser::SerializeStruct::end(state)
    }
}
