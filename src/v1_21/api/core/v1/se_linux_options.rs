// Generated from definition io.k8s.api.core.v1.SELinuxOptions

/// SELinuxOptions are the labels to be applied to the container
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SELinuxOptions {
    /// Level is SELinux level label that applies to the container.
    pub level: Option<String>,

    /// Role is a SELinux role label that applies to the container.
    pub role: Option<String>,

    /// Type is a SELinux type label that applies to the container.
    pub type_: Option<String>,

    /// User is a SELinux user label that applies to the container.
    pub user: Option<String>,

}

#[cfg(feature = "dsl")]
impl SELinuxOptions  {
    /// Set [`Self::level`]
    pub  fn level_set(&mut self, level: impl Into<Option<String>>) -> &mut Self {
        self.level = level.into(); self
    }

    pub  fn level(&mut self) -> &mut String {
        if self.level.is_none() { self.level = Some(Default::default()) }
        self.level.as_mut().unwrap()
    }

    /// Modify [`Self::level`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn level_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.level.is_none() { self.level = Some(Default::default()) };
        func(self.level.as_mut().unwrap()); self
    }


    /// Set [`Self::role`]
    pub  fn role_set(&mut self, role: impl Into<Option<String>>) -> &mut Self {
        self.role = role.into(); self
    }

    pub  fn role(&mut self) -> &mut String {
        if self.role.is_none() { self.role = Some(Default::default()) }
        self.role.as_mut().unwrap()
    }

    /// Modify [`Self::role`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn role_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.role.is_none() { self.role = Some(Default::default()) };
        func(self.role.as_mut().unwrap()); self
    }


    /// Set [`Self::type_`]
    pub  fn type_set(&mut self, type_: impl Into<Option<String>>) -> &mut Self {
        self.type_ = type_.into(); self
    }

    pub  fn type_(&mut self) -> &mut String {
        if self.type_.is_none() { self.type_ = Some(Default::default()) }
        self.type_.as_mut().unwrap()
    }

    /// Modify [`Self::type_`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.type_.is_none() { self.type_ = Some(Default::default()) };
        func(self.type_.as_mut().unwrap()); self
    }


    /// Set [`Self::user`]
    pub  fn user_set(&mut self, user: impl Into<Option<String>>) -> &mut Self {
        self.user = user.into(); self
    }

    pub  fn user(&mut self) -> &mut String {
        if self.user.is_none() { self.user = Some(Default::default()) }
        self.user.as_mut().unwrap()
    }

    /// Modify [`Self::user`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn user_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.user.is_none() { self.user = Some(Default::default()) };
        func(self.user.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for SELinuxOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_level,
            Key_role,
            Key_type_,
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
                            "level" => Field::Key_level,
                            "role" => Field::Key_role,
                            "type" => Field::Key_type_,
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
            type Value = SELinuxOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SELinuxOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_level: Option<String> = None;
                let mut value_role: Option<String> = None;
                let mut value_type_: Option<String> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_level => value_level = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_role => value_role = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SELinuxOptions {
                    level: value_level,
                    role: value_role,
                    type_: value_type_,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "SELinuxOptions",
            &[
                "level",
                "role",
                "type",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SELinuxOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SELinuxOptions",
            self.level.as_ref().map_or(0, |_| 1) +
            self.role.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.level {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "level", value)?;
        }
        if let Some(value) = &self.role {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "role", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SELinuxOptions {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.SELinuxOptions".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("SELinuxOptions are the labels to be applied to the container".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "level".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Level is SELinux level label that applies to the container.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "role".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Role is a SELinux role label that applies to the container.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type is a SELinux type label that applies to the container.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "user".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("User is a SELinux user label that applies to the container.".to_owned()),
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
