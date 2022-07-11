// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReference

/// OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OwnerReference {
    /// API version of the referent.
    pub api_version: String,

    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
    pub block_owner_deletion: Option<bool>,

    /// If true, this reference points to the managing controller.
    pub controller: Option<bool>,

    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,

    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,

    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: String,

}

#[cfg(feature = "dsl")]
impl OwnerReference  {
    /// Set [`Self::api_version`]
    pub  fn api_version_set(&mut self, api_version: impl Into<String>) -> &mut Self {
        self.api_version = api_version.into(); self
    }

    pub  fn api_version(&mut self) -> &mut String {
        &mut self.api_version
    }

    /// Modify [`Self::api_version`] with a `func`
    pub  fn api_version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.api_version); self
    }


    /// Set [`Self::block_owner_deletion`]
    pub  fn block_owner_deletion_set(&mut self, block_owner_deletion: impl Into<Option<bool>>) -> &mut Self {
        self.block_owner_deletion = block_owner_deletion.into(); self
    }

    pub  fn block_owner_deletion(&mut self) -> &mut bool {
        if self.block_owner_deletion.is_none() { self.block_owner_deletion = Some(Default::default()) }
        self.block_owner_deletion.as_mut().unwrap()
    }

    /// Modify [`Self::block_owner_deletion`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn block_owner_deletion_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.block_owner_deletion.is_none() { self.block_owner_deletion = Some(Default::default()) };
        func(self.block_owner_deletion.as_mut().unwrap()); self
    }


    /// Set [`Self::controller`]
    pub  fn controller_set(&mut self, controller: impl Into<Option<bool>>) -> &mut Self {
        self.controller = controller.into(); self
    }

    pub  fn controller(&mut self) -> &mut bool {
        if self.controller.is_none() { self.controller = Some(Default::default()) }
        self.controller.as_mut().unwrap()
    }

    /// Modify [`Self::controller`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn controller_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.controller.is_none() { self.controller = Some(Default::default()) };
        func(self.controller.as_mut().unwrap()); self
    }


    /// Set [`Self::kind`]
    pub  fn kind_set(&mut self, kind: impl Into<String>) -> &mut Self {
        self.kind = kind.into(); self
    }

    pub  fn kind(&mut self) -> &mut String {
        &mut self.kind
    }

    /// Modify [`Self::kind`] with a `func`
    pub  fn kind_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.kind); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::uid`]
    pub  fn uid_set(&mut self, uid: impl Into<String>) -> &mut Self {
        self.uid = uid.into(); self
    }

    pub  fn uid(&mut self) -> &mut String {
        &mut self.uid
    }

    /// Modify [`Self::uid`] with a `func`
    pub  fn uid_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.uid); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for OwnerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_block_owner_deletion,
            Key_controller,
            Key_kind,
            Key_name,
            Key_uid,
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
                            "apiVersion" => Field::Key_api_version,
                            "blockOwnerDeletion" => Field::Key_block_owner_deletion,
                            "controller" => Field::Key_controller,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = OwnerReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("OwnerReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_block_owner_deletion: Option<bool> = None;
                let mut value_controller: Option<bool> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_block_owner_deletion => value_block_owner_deletion = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_controller => value_controller = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(OwnerReference {
                    api_version: value_api_version.unwrap_or_default(),
                    block_owner_deletion: value_block_owner_deletion,
                    controller: value_controller,
                    kind: value_kind.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    uid: value_uid.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "OwnerReference",
            &[
                "apiVersion",
                "blockOwnerDeletion",
                "controller",
                "kind",
                "name",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for OwnerReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "OwnerReference",
            4 +
            self.block_owner_deletion.as_ref().map_or(0, |_| 1) +
            self.controller.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", &self.api_version)?;
        if let Some(value) = &self.block_owner_deletion {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "blockOwnerDeletion", value)?;
        }
        if let Some(value) = &self.controller {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controller", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", &self.uid)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for OwnerReference {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReference".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("API version of the referent.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "blockOwnerDeletion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "controller".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If true, this reference points to the managing controller.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "apiVersion".to_owned(),
                    "kind".to_owned(),
                    "name".to_owned(),
                    "uid".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
