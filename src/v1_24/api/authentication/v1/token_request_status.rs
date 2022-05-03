// Generated from definition io.k8s.api.authentication.v1.TokenRequestStatus

/// TokenRequestStatus is the result of a token request.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenRequestStatus {
    /// ExpirationTimestamp is the time of expiration of the returned token.
    pub expiration_timestamp: crate::apimachinery::pkg::apis::meta::v1::Time,

    /// Token is the opaque bearer token.
    pub token: String,
}

impl<'de> crate::serde::Deserialize<'de> for TokenRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expiration_timestamp,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TokenRequestStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TokenRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expiration_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_token: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expiration_timestamp => value_expiration_timestamp = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_token => value_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenRequestStatus {
                    expiration_timestamp: value_expiration_timestamp.ok_or_else(|| crate::serde::de::Error::missing_field("expirationTimestamp"))?,
                    token: value_token.unwrap_or_default(),
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

impl crate::serde::Serialize for TokenRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenRequestStatus",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expirationTimestamp", &self.expiration_timestamp)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "token", &self.token)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TokenRequestStatus {
    fn schema_name() -> String {
        "io.k8s.api.authentication.v1.TokenRequestStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("TokenRequestStatus is the result of a token request.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "expirationTimestamp".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ExpirationTimestamp is the time of expiration of the returned token.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "token".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Token is the opaque bearer token.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "expirationTimestamp".to_owned(),
                    "token".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
