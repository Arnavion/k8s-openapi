// Generated from definition io.k8s.api.authorization.v1.NonResourceRule

/// NonResourceRule holds information that describes a rule for the non-resource
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NonResourceRule {
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  "*" means all.
    pub non_resource_urls: Option<Vec<String>>,

    /// Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.
    pub verbs: Vec<String>,

}

#[cfg(feature = "dsl")]
impl NonResourceRule  {
    /// Set [`Self::non_resource_urls`]
    pub  fn non_resource_urls_set(&mut self, non_resource_urls: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.non_resource_urls = non_resource_urls.into(); self
    }

    pub  fn non_resource_urls(&mut self) -> &mut Vec<String> {
        if self.non_resource_urls.is_none() { self.non_resource_urls = Some(Default::default()) }
        self.non_resource_urls.as_mut().unwrap()
    }

    /// Modify [`Self::non_resource_urls`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn non_resource_urls_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.non_resource_urls.is_none() { self.non_resource_urls = Some(Default::default()) };
        func(self.non_resource_urls.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::non_resource_urls`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn non_resource_urls_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.non_resource_urls.is_none() {
            self.non_resource_urls = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.non_resource_urls.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::non_resource_urls`]
    pub  fn non_resource_urls_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.non_resource_urls.is_none() { self.non_resource_urls = Some(Vec::new()); }
         let non_resource_urls = &mut self.non_resource_urls.as_mut().unwrap();
         for item in other.borrow() {
             non_resource_urls.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::verbs`]
    pub  fn verbs_set(&mut self, verbs: impl Into<Vec<String>>) -> &mut Self {
        self.verbs = verbs.into(); self
    }

    pub  fn verbs(&mut self) -> &mut Vec<String> {
        &mut self.verbs
    }

    /// Modify [`Self::verbs`] with a `func`
    pub  fn verbs_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.verbs); self
    }

    /// Push new element to [`Self::verbs`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn verbs_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.verbs.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::verbs`]
    pub  fn verbs_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.verbs.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for NonResourceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_urls,
            Key_verbs,
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
                            "nonResourceURLs" => Field::Key_non_resource_urls,
                            "verbs" => Field::Key_verbs,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NonResourceRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NonResourceRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_urls => value_non_resource_urls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourceRule {
                    non_resource_urls: value_non_resource_urls,
                    verbs: value_verbs.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NonResourceRule",
            &[
                "nonResourceURLs",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NonResourceRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourceRule",
            1 +
            self.non_resource_urls.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_urls {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NonResourceRule {
    fn schema_name() -> String {
        "io.k8s.api.authorization.v1.NonResourceRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NonResourceRule holds information that describes a rule for the non-resource".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nonResourceURLs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  \"*\" means all.".to_owned()),
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
                        "verbs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  \"*\" means all.".to_owned()),
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
                ].into(),
                required: [
                    "verbs".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
