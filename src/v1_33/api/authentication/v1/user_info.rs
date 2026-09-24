// Generated from definition io.k8s.api.authentication.v1.UserInfo

/// UserInfo holds the information about the user needed to implement the user.Info interface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserInfo {
    /// Any additional information provided by the authenticator.
    pub extra: Option<std::collections::BTreeMap<std::string::String, std::vec::Vec<std::string::String>>>,

    /// The names of groups this user is a part of.
    pub groups: Option<std::vec::Vec<std::string::String>>,

    /// A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs.
    pub uid: Option<std::string::String>,

    /// The name that uniquely identifies this user among all active users.
    pub username: Option<std::string::String>,
}

impl crate::DeepMerge for UserInfo {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.extra, other.extra, |current_item, other_item| {
            crate::merge_strategies::list::atomic(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.groups, other.groups);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
        crate::DeepMerge::merge_from(&mut self.username, other.username);
    }
}

impl<'de> crate::serde::Deserialize<'de> for UserInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_extra,
            Key_groups,
            Key_uid,
            Key_username,
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
                            "extra" => Field::Key_extra,
                            "groups" => Field::Key_groups,
                            "uid" => Field::Key_uid,
                            "username" => Field::Key_username,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = UserInfo;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("UserInfo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_extra: Option<std::collections::BTreeMap<std::string::String, std::vec::Vec<std::string::String>>> = None;
                let mut value_groups: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_uid: Option<std::string::String> = None;
                let mut value_username: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(UserInfo {
                    extra: value_extra,
                    groups: value_groups,
                    uid: value_uid,
                    username: value_username,
                })
            }
        }

        deserializer.deserialize_struct(
            "UserInfo",
            &[
                "extra",
                "groups",
                "uid",
                "username",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for UserInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "UserInfo",
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.username {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for UserInfo {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authentication.v1.UserInfo".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "UserInfo holds the information about the user needed to implement the user.Info interface.",
            "type": "object",
            "properties": {
                "extra": {
                    "description": "Any additional information provided by the authenticator.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "array",
                        "items": {
                            "type": "string",
                        },
                    },
                },
                "groups": {
                    "description": "The names of groups this user is a part of.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "uid": {
                    "description": "A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs.",
                    "type": "string",
                },
                "username": {
                    "description": "The name that uniquely identifies this user among all active users.",
                    "type": "string",
                },
            },
        })
    }
}
