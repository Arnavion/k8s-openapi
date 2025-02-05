// Generated from definition io.k8s.api.flowcontrol.v1beta3.Subject

/// Subject matches the originator of a request, as identified by the request authentication system. There are three ways of matching an originator; by user, group, or service account.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Subject {
    /// `group` matches based on user group name.
    pub group: Option<crate::api::flowcontrol::v1beta3::GroupSubject>,

    /// `kind` indicates which one of the other fields is non-empty. Required
    pub kind: std::string::String,

    /// `serviceAccount` matches ServiceAccounts.
    pub service_account: Option<crate::api::flowcontrol::v1beta3::ServiceAccountSubject>,

    /// `user` matches based on username.
    pub user: Option<crate::api::flowcontrol::v1beta3::UserSubject>,
}

impl crate::DeepMerge for Subject {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.group, other.group);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.service_account, other.service_account);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Subject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group: Option<crate::api::flowcontrol::v1beta3::GroupSubject> = None;
                let mut value_kind: Option<std::string::String> = None;
                let mut value_service_account: Option<crate::api::flowcontrol::v1beta3::ServiceAccountSubject> = None;
                let mut value_user: Option<crate::api::flowcontrol::v1beta3::UserSubject> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account => value_service_account = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Subject {
                    group: value_group,
                    kind: value_kind.unwrap_or_default(),
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Subject {
    fn schema_name() -> std::string::String {
        "io.k8s.api.flowcontrol.v1beta3.Subject".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Subject matches the originator of a request, as identified by the request authentication system. There are three ways of matching an originator; by user, group, or service account.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "group".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1beta3::GroupSubject>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`group` matches based on user group name.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "kind".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`kind` indicates which one of the other fields is non-empty. Required".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serviceAccount".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1beta3::ServiceAccountSubject>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`serviceAccount` matches ServiceAccounts.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "user".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1beta3::UserSubject>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`user` matches based on username.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "kind".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
