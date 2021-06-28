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

#[cfg(feature = "schema")]
impl crate::Schema for SELinuxOptions {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "SELinuxOptions are the labels to be applied to the container",
          "properties": {
            "level": {
              "description": "Level is SELinux level label that applies to the container.",
              "type": "string"
            },
            "role": {
              "description": "Role is a SELinux role label that applies to the container.",
              "type": "string"
            },
            "type": {
              "description": "Type is a SELinux type label that applies to the container.",
              "type": "string"
            },
            "user": {
              "description": "User is a SELinux user label that applies to the container.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
