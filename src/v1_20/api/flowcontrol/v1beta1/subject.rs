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

impl<'de> serde::Deserialize<'de> for Subject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_kind,
            Key_service_account,
            Key_user,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Subject;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Subject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_group: Option<crate::api::flowcontrol::v1beta1::GroupSubject> = None;
                let mut value_kind: Option<String> = None;
                let mut value_service_account: Option<crate::api::flowcontrol::v1beta1::ServiceAccountSubject> = None;
                let mut value_user: Option<crate::api::flowcontrol::v1beta1::UserSubject> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_service_account => value_service_account = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Subject {
                    group: value_group,
                    kind: value_kind.ok_or_else(|| serde::de::Error::missing_field("kind"))?,
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

impl serde::Serialize for Subject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Subject",
            1 +
            self.group.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.group {
            serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        if let Some(value) = &self.service_account {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.user {
            serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
