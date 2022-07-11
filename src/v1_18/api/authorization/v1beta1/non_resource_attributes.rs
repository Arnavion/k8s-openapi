// Generated from definition io.k8s.api.authorization.v1beta1.NonResourceAttributes

/// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request
    pub path: Option<String>,

    /// Verb is the standard HTTP verb
    pub verb: Option<String>,

}

#[cfg(feature = "dsl")]
impl NonResourceAttributes  {
    /// Set [`Self::path`]
    pub  fn path_set(&mut self, path: impl Into<Option<String>>) -> &mut Self {
        self.path = path.into(); self
    }

    pub  fn path(&mut self) -> &mut String {
        if self.path.is_none() { self.path = Some(Default::default()) }
        self.path.as_mut().unwrap()
    }

    /// Modify [`Self::path`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.path.is_none() { self.path = Some(Default::default()) };
        func(self.path.as_mut().unwrap()); self
    }


    /// Set [`Self::verb`]
    pub  fn verb_set(&mut self, verb: impl Into<Option<String>>) -> &mut Self {
        self.verb = verb.into(); self
    }

    pub  fn verb(&mut self) -> &mut String {
        if self.verb.is_none() { self.verb = Some(Default::default()) }
        self.verb.as_mut().unwrap()
    }

    /// Modify [`Self::verb`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn verb_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.verb.is_none() { self.verb = Some(Default::default()) };
        func(self.verb.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for NonResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_path,
            Key_verb,
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
                            "path" => Field::Key_path,
                            "verb" => Field::Key_verb,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NonResourceAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NonResourceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_path: Option<String> = None;
                let mut value_verb: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verb => value_verb = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourceAttributes {
                    path: value_path,
                    verb: value_verb,
                })
            }
        }

        deserializer.deserialize_struct(
            "NonResourceAttributes",
            &[
                "path",
                "verb",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NonResourceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourceAttributes",
            self.path.as_ref().map_or(0, |_| 1) +
            self.verb.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.verb {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verb", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NonResourceAttributes {
    fn schema_name() -> String {
        "io.k8s.api.authorization.v1beta1.NonResourceAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "path".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path is the URL path of the request".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "verb".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Verb is the standard HTTP verb".to_owned()),
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
