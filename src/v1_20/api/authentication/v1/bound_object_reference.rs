// Generated from definition io.k8s.api.authentication.v1.BoundObjectReference

/// BoundObjectReference is a reference to an object that a token is bound to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoundObjectReference {
    /// API version of the referent.
    pub api_version: Option<String>,

    /// Kind of the referent. Valid kinds are 'Pod' and 'Secret'.
    pub kind: Option<String>,

    /// Name of the referent.
    pub name: Option<String>,

    /// UID of the referent.
    pub uid: Option<String>,

}

#[cfg(feature = "dsl")]
impl BoundObjectReference  {
    /// Set [`Self::api_version`]
    pub  fn api_version_set(&mut self, api_version: impl Into<Option<String>>) -> &mut Self {
        self.api_version = api_version.into(); self
    }

    pub  fn api_version(&mut self) -> &mut String {
        if self.api_version.is_none() { self.api_version = Some(Default::default()) }
        self.api_version.as_mut().unwrap()
    }

    /// Modify [`Self::api_version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn api_version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.api_version.is_none() { self.api_version = Some(Default::default()) };
        func(self.api_version.as_mut().unwrap()); self
    }


    /// Set [`Self::kind`]
    pub  fn kind_set(&mut self, kind: impl Into<Option<String>>) -> &mut Self {
        self.kind = kind.into(); self
    }

    pub  fn kind(&mut self) -> &mut String {
        if self.kind.is_none() { self.kind = Some(Default::default()) }
        self.kind.as_mut().unwrap()
    }

    /// Modify [`Self::kind`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn kind_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.kind.is_none() { self.kind = Some(Default::default()) };
        func(self.kind.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<Option<String>>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        if self.name.is_none() { self.name = Some(Default::default()) }
        self.name.as_mut().unwrap()
    }

    /// Modify [`Self::name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.name.is_none() { self.name = Some(Default::default()) };
        func(self.name.as_mut().unwrap()); self
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


}


impl<'de> crate::serde::Deserialize<'de> for BoundObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
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
            type Value = BoundObjectReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BoundObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BoundObjectReference {
                    api_version: value_api_version,
                    kind: value_kind,
                    name: value_name,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "BoundObjectReference",
            &[
                "apiVersion",
                "kind",
                "name",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for BoundObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BoundObjectReference",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BoundObjectReference {
    fn schema_name() -> String {
        "io.k8s.api.authentication.v1.BoundObjectReference".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BoundObjectReference is a reference to an object that a token is bound to.".to_owned()),
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
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind of the referent. Valid kinds are 'Pod' and 'Secret'.".to_owned()),
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
                                description: Some("Name of the referent.".to_owned()),
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
                                description: Some("UID of the referent.".to_owned()),
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
