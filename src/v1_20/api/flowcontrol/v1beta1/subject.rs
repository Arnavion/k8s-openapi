// Generated from definition io.k8s.api.flowcontrol.v1beta1.Subject

/// Subject matches the originator of a request, as identified by the request authentication system. There are three ways of matching an originator; by user, group, or service account.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Subject {
    pub group: Option<crate::api::flowcontrol::v1beta1::GroupSubject>,

    /// Required
    pub kind: String,

    pub service_account: Option<crate::api::flowcontrol::v1beta1::ServiceAccountSubject>,

    pub user: Option<crate::api::flowcontrol::v1beta1::UserSubject>,
}

impl<'de> crate::serde::Deserialize<'de> for Subject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_kind,
            Key_service_account,
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
                            "group" => Field::Key_group,
                            "kind" => Field::Key_kind,
                            "serviceAccount" => Field::Key_service_account,
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
            type Value = Subject;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Subject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group: Option<crate::api::flowcontrol::v1beta1::GroupSubject> = None;
                let mut value_kind: Option<String> = None;
                let mut value_service_account: Option<crate::api::flowcontrol::v1beta1::ServiceAccountSubject> = None;
                let mut value_user: Option<crate::api::flowcontrol::v1beta1::UserSubject> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_service_account => value_service_account = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Subject {
                    group: value_group,
                    kind: value_kind.ok_or_else(|| crate::serde::de::Error::missing_field("kind"))?,
                    service_account: value_service_account,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "Subject",
            &[
                "group",
                "kind",
                "serviceAccount",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Subject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Subject",
            1 +
            self.group.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        if let Some(value) = &self.service_account {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Subject {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Subject matches the originator of a request, as identified by the request authentication system. There are three ways of matching an originator; by user, group, or service account.",
          "properties": {
            "group": crate::api::flowcontrol::v1beta1::GroupSubject::schema(),
            "kind": {
              "description": "Required",
              "type": "string"
            },
            "serviceAccount": crate::api::flowcontrol::v1beta1::ServiceAccountSubject::schema(),
            "user": crate::api::flowcontrol::v1beta1::UserSubject::schema()
          },
          "required": [
            "kind"
          ],
          "type": "object"
        })
    }
}
