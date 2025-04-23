// Generated from definition io.k8s.api.core.v1.SELinuxOptions

/// SELinuxOptions are the labels to be applied to the container
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SELinuxOptions {
    /// Level is SELinux level label that applies to the container.
    pub level: Option<std::string::String>,

    /// Role is a SELinux role label that applies to the container.
    pub role: Option<std::string::String>,

    /// Type is a SELinux type label that applies to the container.
    pub type_: Option<std::string::String>,

    /// User is a SELinux user label that applies to the container.
    pub user: Option<std::string::String>,
}

impl crate::DeepMerge for SELinuxOptions {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.level, other.level);
        crate::DeepMerge::merge_from(&mut self.role, other.role);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SELinuxOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_level: Option<std::string::String> = None;
                let mut value_role: Option<std::string::String> = None;
                let mut value_type_: Option<std::string::String> = None;
                let mut value_user: Option<std::string::String> = None;

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
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.SELinuxOptions".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SELinuxOptions are the labels to be applied to the container".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "level".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Level is SELinux level label that applies to the container.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "role".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Role is a SELinux role label that applies to the container.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type is a SELinux type label that applies to the container.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "user".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("User is a SELinux user label that applies to the container.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
