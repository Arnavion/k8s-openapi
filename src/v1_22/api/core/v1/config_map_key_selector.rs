// Generated from definition io.k8s.api.core.v1.ConfigMapKeySelector

/// Selects a key from a ConfigMap.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapKeySelector {
    /// The key to select.
    pub key: String,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Specify whether the ConfigMap or its key must be defined
    pub optional: Option<bool>,

}

#[cfg(feature = "dsl")]
impl ConfigMapKeySelector  {
    /// Set [`Self::key`]
    pub  fn key_set(&mut self, key: impl Into<String>) -> &mut Self {
        self.key = key.into(); self
    }

    pub  fn key(&mut self) -> &mut String {
        &mut self.key
    }

    /// Modify [`Self::key`] with a `func`
    pub  fn key_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.key); self
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


impl<'de> crate::serde::Deserialize<'de> for ConfigMapKeySelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
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
                            "key" => Field::Key_key,
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
            type Value = ConfigMapKeySelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ConfigMapKeySelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_optional: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapKeySelector {
                    key: value_key.unwrap_or_default(),
                    name: value_name,
                    optional: value_optional,
                })
            }
        }

        deserializer.deserialize_struct(
            "ConfigMapKeySelector",
            &[
                "key",
                "name",
                "optional",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ConfigMapKeySelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ConfigMapKeySelector",
            1 +
            self.name.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
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
impl crate::schemars::JsonSchema for ConfigMapKeySelector {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ConfigMapKeySelector".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Selects a key from a ConfigMap.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The key to select.".to_owned()),
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
                                description: Some("Specify whether the ConfigMap or its key must be defined".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "key".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
