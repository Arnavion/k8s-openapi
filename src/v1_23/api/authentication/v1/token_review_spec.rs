// Generated from definition io.k8s.api.authentication.v1.TokenReviewSpec

/// TokenReviewSpec is a description of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewSpec {
    /// Audiences is a list of the identifiers that the resource server presented with the token identifies as. Audience-aware token authenticators will verify that the token was intended for at least one of the audiences in this list. If no audiences are provided, the audience will default to the audience of the Kubernetes apiserver.
    pub audiences: Option<Vec<String>>,

    /// Token is the opaque bearer token.
    pub token: Option<String>,

}

#[cfg(feature = "dsl")]
impl TokenReviewSpec  {
    /// Set [`Self::audiences`]
    pub  fn audiences_set(&mut self, audiences: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.audiences = audiences.into(); self
    }

    pub  fn audiences(&mut self) -> &mut Vec<String> {
        if self.audiences.is_none() { self.audiences = Some(Default::default()) }
        self.audiences.as_mut().unwrap()
    }

    /// Modify [`Self::audiences`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn audiences_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.audiences.is_none() { self.audiences = Some(Default::default()) };
        func(self.audiences.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::audiences`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn audiences_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.audiences.is_none() {
            self.audiences = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.audiences.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::audiences`]
    pub  fn audiences_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.audiences.is_none() { self.audiences = Some(Vec::new()); }
         let audiences = &mut self.audiences.as_mut().unwrap();
         for item in other.borrow() {
             audiences.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::token`]
    pub  fn token_set(&mut self, token: impl Into<Option<String>>) -> &mut Self {
        self.token = token.into(); self
    }

    pub  fn token(&mut self) -> &mut String {
        if self.token.is_none() { self.token = Some(Default::default()) }
        self.token.as_mut().unwrap()
    }

    /// Modify [`Self::token`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn token_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.token.is_none() { self.token = Some(Default::default()) };
        func(self.token.as_mut().unwrap()); self
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TokenReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audiences: Option<Vec<String>> = None;
                let mut value_token: Option<String> = None;

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
    fn schema_name() -> String {
        "io.k8s.api.authentication.v1.TokenReviewSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("TokenReviewSpec is a description of the token authentication request.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "audiences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Audiences is a list of the identifiers that the resource server presented with the token identifies as. Audience-aware token authenticators will verify that the token was intended for at least one of the audiences in this list. If no audiences are provided, the audience will default to the audience of the Kubernetes apiserver.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
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
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
