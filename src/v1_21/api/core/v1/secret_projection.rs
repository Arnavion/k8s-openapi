// Generated from definition io.k8s.api.core.v1.SecretProjection

/// Adapts a secret into a projected volume.
///
/// The contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecretProjection {
    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    pub items: Option<Vec<crate::api::core::v1::KeyToPath>>,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Specify whether the Secret or its key must be defined
    pub optional: Option<bool>,

}

#[cfg(feature = "dsl")]
impl SecretProjection  {
    /// Set [`Self::items`]
    pub  fn items_set(&mut self, items: impl Into<Option<Vec<crate::api::core::v1::KeyToPath>>>) -> &mut Self {
        self.items = items.into(); self
    }

    pub  fn items(&mut self) -> &mut Vec<crate::api::core::v1::KeyToPath> {
        if self.items.is_none() { self.items = Some(Default::default()) }
        self.items.as_mut().unwrap()
    }

    /// Modify [`Self::items`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn items_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::KeyToPath>)) -> &mut Self {
        if self.items.is_none() { self.items = Some(Default::default()) };
        func(self.items.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::items`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn items_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::KeyToPath)) -> &mut Self {
        if self.items.is_none() {
            self.items = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.items.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::items`]
    pub  fn items_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::KeyToPath]>) -> &mut Self {
         if self.items.is_none() { self.items = Some(Vec::new()); }
         let items = &mut self.items.as_mut().unwrap();
         for item in other.borrow() {
             items.push(item.to_owned());
         }
         self
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


    /// Set [`Self::optional`]
    pub  fn optional_set(&mut self, optional: impl Into<Option<bool>>) -> &mut Self {
        self.optional = optional.into(); self
    }

    pub  fn optional(&mut self) -> &mut bool {
        if self.optional.is_none() { self.optional = Some(Default::default()) }
        self.optional.as_mut().unwrap()
    }

    /// Modify [`Self::optional`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn optional_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.optional.is_none() { self.optional = Some(Default::default()) };
        func(self.optional.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for SecretProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_items,
            Key_name,
            Key_optional,
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
                            "items" => Field::Key_items,
                            "name" => Field::Key_name,
                            "optional" => Field::Key_optional,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SecretProjection;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecretProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_items: Option<Vec<crate::api::core::v1::KeyToPath>> = None;
                let mut value_name: Option<String> = None;
                let mut value_optional: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecretProjection {
                    items: value_items,
                    name: value_name,
                    optional: value_optional,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecretProjection",
            &[
                "items",
                "name",
                "optional",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SecretProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecretProjection",
            self.items.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SecretProjection {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.SecretProjection".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Adapts a secret into a projected volume.\n\nThe contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "items".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::KeyToPath>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "optional".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specify whether the Secret or its key must be defined".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
