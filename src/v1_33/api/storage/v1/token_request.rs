// Generated from definition io.k8s.api.storage.v1.TokenRequest

/// TokenRequest contains parameters of a service account token.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenRequest {
    /// audience is the intended audience of the token in "TokenRequestSpec". It will default to the audiences of kube apiserver.
    pub audience: std::string::String,

    /// expirationSeconds is the duration of validity of the token in "TokenRequestSpec". It has the same default value of "ExpirationSeconds" in "TokenRequestSpec".
    pub expiration_seconds: Option<i64>,
}

impl crate::DeepMerge for TokenRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.audience, other.audience);
        crate::DeepMerge::merge_from(&mut self.expiration_seconds, other.expiration_seconds);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TokenRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audience,
            Key_expiration_seconds,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TokenRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TokenRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audience: Option<std::string::String> = None;
                let mut value_expiration_seconds: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audience => value_audience = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expiration_seconds => value_expiration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TokenRequest {
                    audience: value_audience.unwrap_or_default(),
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

impl crate::serde::Serialize for TokenRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TokenRequest",
            1 +
            self.expiration_seconds.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "audience", &self.audience)?;
        if let Some(value) = &self.expiration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expirationSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TokenRequest {
    fn schema_name() -> std::string::String {
        "io.k8s.api.storage.v1.TokenRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("TokenRequest contains parameters of a service account token.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "audience".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("audience is the intended audience of the token in \"TokenRequestSpec\". It will default to the audiences of kube apiserver.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "expirationSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("expirationSeconds is the duration of validity of the token in \"TokenRequestSpec\". It has the same default value of \"ExpirationSeconds\" in \"TokenRequestSpec\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "audience".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
