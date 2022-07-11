// Generated from definition io.k8s.api.authorization.v1beta1.SubjectAccessReviewSpec

/// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectAccessReviewSpec {
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
    pub extra: Option<std::collections::BTreeMap<String, Vec<String>>>,

    /// Groups is the groups you're testing for.
    pub group: Option<Vec<String>>,

    /// NonResourceAttributes describes information for a non-resource access request
    pub non_resource_attributes: Option<crate::api::authorization::v1beta1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    pub resource_attributes: Option<crate::api::authorization::v1beta1::ResourceAttributes>,

    /// UID information about the requesting user.
    pub uid: Option<String>,

    /// User is the user you're testing for. If you specify "User" but not "Group", then is it interpreted as "What if User were not a member of any groups
    pub user: Option<String>,

}

#[cfg(feature = "dsl")]
impl SubjectAccessReviewSpec  {
    /// Set [`Self::extra`]
    pub  fn extra_set(&mut self, extra: impl Into<Option<std::collections::BTreeMap<String, Vec<String>>>>) -> &mut Self {
        self.extra = extra.into(); self
    }

    pub  fn extra(&mut self) -> &mut std::collections::BTreeMap<String, Vec<String>> {
        if self.extra.is_none() { self.extra = Some(Default::default()) }
        self.extra.as_mut().unwrap()
    }

    /// Modify [`Self::extra`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn extra_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, Vec<String>>)) -> &mut Self {
        if self.extra.is_none() { self.extra = Some(Default::default()) };
        func(self.extra.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::extra`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn extra_insert_with(&mut self, name: &str, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.extra.is_none() {
            self.extra = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.extra.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::extra`]
    pub  fn extra_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, Vec<String>>>) -> &mut Self {
         if self.extra.is_none() { self.extra = Some(std::collections::BTreeMap::new()); }
         let extra = &mut self.extra.as_mut().unwrap();
         for (name, value) in other.borrow() {
             extra.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::group`]
    pub  fn group_set(&mut self, group: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.group = group.into(); self
    }

    pub  fn group(&mut self) -> &mut Vec<String> {
        if self.group.is_none() { self.group = Some(Default::default()) }
        self.group.as_mut().unwrap()
    }

    /// Modify [`Self::group`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn group_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.group.is_none() { self.group = Some(Default::default()) };
        func(self.group.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::group`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn group_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.group.is_none() {
            self.group = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.group.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::group`]
    pub  fn group_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.group.is_none() { self.group = Some(Vec::new()); }
         let group = &mut self.group.as_mut().unwrap();
         for item in other.borrow() {
             group.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::non_resource_attributes`]
    pub  fn non_resource_attributes_set(&mut self, non_resource_attributes: impl Into<Option<crate::api::authorization::v1beta1::NonResourceAttributes>>) -> &mut Self {
        self.non_resource_attributes = non_resource_attributes.into(); self
    }

    pub  fn non_resource_attributes(&mut self) -> &mut crate::api::authorization::v1beta1::NonResourceAttributes {
        if self.non_resource_attributes.is_none() { self.non_resource_attributes = Some(Default::default()) }
        self.non_resource_attributes.as_mut().unwrap()
    }

    /// Modify [`Self::non_resource_attributes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn non_resource_attributes_with(&mut self, func: impl FnOnce(&mut crate::api::authorization::v1beta1::NonResourceAttributes)) -> &mut Self {
        if self.non_resource_attributes.is_none() { self.non_resource_attributes = Some(Default::default()) };
        func(self.non_resource_attributes.as_mut().unwrap()); self
    }


    /// Set [`Self::resource_attributes`]
    pub  fn resource_attributes_set(&mut self, resource_attributes: impl Into<Option<crate::api::authorization::v1beta1::ResourceAttributes>>) -> &mut Self {
        self.resource_attributes = resource_attributes.into(); self
    }

    pub  fn resource_attributes(&mut self) -> &mut crate::api::authorization::v1beta1::ResourceAttributes {
        if self.resource_attributes.is_none() { self.resource_attributes = Some(Default::default()) }
        self.resource_attributes.as_mut().unwrap()
    }

    /// Modify [`Self::resource_attributes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_attributes_with(&mut self, func: impl FnOnce(&mut crate::api::authorization::v1beta1::ResourceAttributes)) -> &mut Self {
        if self.resource_attributes.is_none() { self.resource_attributes = Some(Default::default()) };
        func(self.resource_attributes.as_mut().unwrap()); self
    }


    /// Set [`Self::uid`]
    pub  fn uid_set(&mut self, uid: impl Into<Option<String>>) -> &mut Self {
        self.uid = uid.into(); self
    }

    pub  fn uid(&mut self) -> &mut String {
        if self.uid.is_none() { self.uid = Some(Default::default()) }
        self.uid.as_mut().unwrap()
    }

    /// Modify [`Self::uid`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn uid_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.uid.is_none() { self.uid = Some(Default::default()) };
        func(self.uid.as_mut().unwrap()); self
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


impl<'de> crate::serde::Deserialize<'de> for SubjectAccessReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_extra,
            Key_group,
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
                            "group" => Field::Key_group,
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
                let mut value_group: Option<Vec<String>> = None;
                let mut value_non_resource_attributes: Option<crate::api::authorization::v1beta1::NonResourceAttributes> = None;
                let mut value_resource_attributes: Option<crate::api::authorization::v1beta1::ResourceAttributes> = None;
                let mut value_uid: Option<String> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_attributes => value_non_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_attributes => value_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectAccessReviewSpec {
                    extra: value_extra,
                    group: value_group,
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
                "group",
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
            self.extra.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.non_resource_attributes.as_ref().map_or(0, |_| 1) +
            self.resource_attributes.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SubjectAccessReviewSpec {
    fn schema_name() -> String {
        "io.k8s.api.authorization.v1beta1.SubjectAccessReviewSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "extra".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
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
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "group".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Groups is the groups you're testing for.".to_owned()),
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
                        "nonResourceAttributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1beta1::NonResourceAttributes>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NonResourceAttributes describes information for a non-resource access request".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceAttributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1beta1::ResourceAttributes>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceAuthorizationAttributes describes information for a resource access request".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID information about the requesting user.".to_owned()),
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
                                description: Some("User is the user you're testing for. If you specify \"User\" but not \"Group\", then is it interpreted as \"What if User were not a member of any groups".to_owned()),
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
