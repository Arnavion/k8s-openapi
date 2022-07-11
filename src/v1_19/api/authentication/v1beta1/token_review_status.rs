// Generated from definition io.k8s.api.authentication.v1beta1.TokenReviewStatus

/// TokenReviewStatus is the result of the token authentication request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewStatus {
    /// Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is "true", the token is valid against the audience of the Kubernetes API server.
    pub audiences: Option<Vec<String>>,

    /// Authenticated indicates that the token was associated with a known user.
    pub authenticated: Option<bool>,

    /// Error indicates that the token couldn't be checked
    pub error: Option<String>,

    /// User is the UserInfo associated with the provided token.
    pub user: Option<crate::api::authentication::v1beta1::UserInfo>,

}

#[cfg(feature = "dsl")]
impl TokenReviewStatus  {
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


    /// Set [`Self::authenticated`]
    pub  fn authenticated_set(&mut self, authenticated: impl Into<Option<bool>>) -> &mut Self {
        self.authenticated = authenticated.into(); self
    }

    pub  fn authenticated(&mut self) -> &mut bool {
        if self.authenticated.is_none() { self.authenticated = Some(Default::default()) }
        self.authenticated.as_mut().unwrap()
    }

    /// Modify [`Self::authenticated`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn authenticated_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.authenticated.is_none() { self.authenticated = Some(Default::default()) };
        func(self.authenticated.as_mut().unwrap()); self
    }


    /// Set [`Self::error`]
    pub  fn error_set(&mut self, error: impl Into<Option<String>>) -> &mut Self {
        self.error = error.into(); self
    }

    pub  fn error(&mut self) -> &mut String {
        if self.error.is_none() { self.error = Some(Default::default()) }
        self.error.as_mut().unwrap()
    }

    /// Modify [`Self::error`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn error_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.error.is_none() { self.error = Some(Default::default()) };
        func(self.error.as_mut().unwrap()); self
    }


    /// Set [`Self::user`]
    pub  fn user_set(&mut self, user: impl Into<Option<crate::api::authentication::v1beta1::UserInfo>>) -> &mut Self {
        self.user = user.into(); self
    }

    pub  fn user(&mut self) -> &mut crate::api::authentication::v1beta1::UserInfo {
        if self.user.is_none() { self.user = Some(Default::default()) }
        self.user.as_mut().unwrap()
    }

    /// Modify [`Self::user`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn user_with(&mut self, func: impl FnOnce(&mut crate::api::authentication::v1beta1::UserInfo)) -> &mut Self {
        if self.user.is_none() { self.user = Some(Default::default()) };
        func(self.user.as_mut().unwrap()); self
    }


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
                    audiences: value_audiences,
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
            self.audiences.as_ref().map_or(0, |_| 1) +
            self.authenticated.as_ref().map_or(0, |_| 1) +
            self.error.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.audiences {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "audiences", value)?;
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TokenReviewStatus {
    fn schema_name() -> String {
        "io.k8s.api.authentication.v1beta1.TokenReviewStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("TokenReviewStatus is the result of the token authentication request.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "audiences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is \"true\", the token is valid against the audience of the Kubernetes API server.".to_owned()),
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
                        "authenticated".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Authenticated indicates that the token was associated with a known user.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "error".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Error indicates that the token couldn't be checked".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "user".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authentication::v1beta1::UserInfo>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("User is the UserInfo associated with the provided token.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
