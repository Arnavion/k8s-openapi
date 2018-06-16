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

impl<'de> ::serde::Deserialize<'de> for SELinuxOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_level,
            Key_role,
            Key_type_,
            Key_user,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SELinuxOptions;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct SELinuxOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_level: Option<String> = None;
                let mut value_role: Option<String> = None;
                let mut value_type_: Option<String> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_level => value_level = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_role => value_role = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for SELinuxOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SELinuxOptions",
            0 +
            self.level.as_ref().map_or(0, |_| 1) +
            self.role.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.level {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "level", value)?;
        }
        if let Some(value) = &self.role {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "role", value)?;
        }
        if let Some(value) = &self.type_ {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        if let Some(value) = &self.user {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
