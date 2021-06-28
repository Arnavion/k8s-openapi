// Generated from definition io.k8s.api.authorization.v1.SubjectAccessReviewSpec

/// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectAccessReviewSpec {
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
    pub extra: std::collections::BTreeMap<String, Vec<String>>,

    /// Groups is the groups you're testing for.
    pub groups: Vec<String>,

    /// NonResourceAttributes describes information for a non-resource access request
    pub non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    pub resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes>,

    /// UID information about the requesting user.
    pub uid: Option<String>,

    /// User is the user you're testing for. If you specify "User" but not "Groups", then is it interpreted as "What if User were not a member of any groups
    pub user: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for SubjectAccessReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_extra,
            Key_groups,
            Key_non_resource_attributes,
            Key_resource_attributes,
            Key_uid,
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
                            "extra" => Field::Key_extra,
                            "groups" => Field::Key_groups,
                            "nonResourceAttributes" => Field::Key_non_resource_attributes,
                            "resourceAttributes" => Field::Key_resource_attributes,
                            "uid" => Field::Key_uid,
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
            type Value = SubjectAccessReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SubjectAccessReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_extra: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes> = None;
                let mut value_resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes> = None;
                let mut value_uid: Option<String> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_attributes => value_non_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_attributes => value_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectAccessReviewSpec {
                    extra: value_extra.unwrap_or_default(),
                    groups: value_groups.unwrap_or_default(),
                    non_resource_attributes: value_non_resource_attributes,
                    resource_attributes: value_resource_attributes,
                    uid: value_uid,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectAccessReviewSpec",
            &[
                "extra",
                "groups",
                "nonResourceAttributes",
                "resourceAttributes",
                "uid",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SubjectAccessReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectAccessReviewSpec",
            usize::from(!self.extra.is_empty()) +
            usize::from(!self.groups.is_empty()) +
            self.non_resource_attributes.as_ref().map_or(0, |_| 1) +
            self.resource_attributes.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if !self.extra.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", &self.extra)?;
        }
        if !self.groups.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", &self.groups)?;
        }
        if let Some(value) = &self.non_resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceAttributes", value)?;
        }
        if let Some(value) = &self.resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceAttributes", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for SubjectAccessReviewSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set",
          "properties": {
            "extra": {
              "additionalProperties": {
                "items": {
                  "type": "string"
                },
                "type": "array"
              },
              "description": "Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.",
              "type": "object"
            },
            "groups": {
              "description": "Groups is the groups you're testing for.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "nonResourceAttributes": crate::schema_ref_with_description(crate::api::authorization::v1::NonResourceAttributes::schema(), "NonResourceAttributes describes information for a non-resource access request"),
            "resourceAttributes": crate::schema_ref_with_description(crate::api::authorization::v1::ResourceAttributes::schema(), "ResourceAuthorizationAttributes describes information for a resource access request"),
            "uid": {
              "description": "UID information about the requesting user.",
              "type": "string"
            },
            "user": {
              "description": "User is the user you're testing for. If you specify \"User\" but not \"Groups\", then is it interpreted as \"What if User were not a member of any groups",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
