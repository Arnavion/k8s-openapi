// Generated from definition io.k8s.api.core.v1.Capabilities

/// Adds and removes POSIX capabilities from running containers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Capabilities {
    /// Added capabilities
    pub add: Option<Vec<String>>,

    /// Removed capabilities
    pub drop: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl Capabilities  {
    /// Set [`Self::add`]
    pub  fn add_set(&mut self, add: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.add = add.into(); self
    }

    pub  fn add(&mut self) -> &mut Vec<String> {
        if self.add.is_none() { self.add = Some(Default::default()) }
        self.add.as_mut().unwrap()
    }

    /// Modify [`Self::add`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn add_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.add.is_none() { self.add = Some(Default::default()) };
        func(self.add.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::add`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn add_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.add.is_none() {
            self.add = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.add.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::add`]
    pub  fn add_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.add.is_none() { self.add = Some(Vec::new()); }
         let add = &mut self.add.as_mut().unwrap();
         for item in other.borrow() {
             add.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::drop`]
    pub  fn drop_set(&mut self, drop: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.drop = drop.into(); self
    }

    pub  fn drop(&mut self) -> &mut Vec<String> {
        if self.drop.is_none() { self.drop = Some(Default::default()) }
        self.drop.as_mut().unwrap()
    }

    /// Modify [`Self::drop`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn drop_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.drop.is_none() { self.drop = Some(Default::default()) };
        func(self.drop.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::drop`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn drop_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.drop.is_none() {
            self.drop = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.drop.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::drop`]
    pub  fn drop_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.drop.is_none() { self.drop = Some(Vec::new()); }
         let drop = &mut self.drop.as_mut().unwrap();
         for item in other.borrow() {
             drop.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Capabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_add,
            Key_drop,
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
                            "add" => Field::Key_add,
                            "drop" => Field::Key_drop,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Capabilities;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Capabilities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_add: Option<Vec<String>> = None;
                let mut value_drop: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_add => value_add = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_drop => value_drop = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Capabilities {
                    add: value_add,
                    drop: value_drop,
                })
            }
        }

        deserializer.deserialize_struct(
            "Capabilities",
            &[
                "add",
                "drop",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Capabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Capabilities",
            self.add.as_ref().map_or(0, |_| 1) +
            self.drop.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.add {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "add", value)?;
        }
        if let Some(value) = &self.drop {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "drop", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Capabilities {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Capabilities".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Adds and removes POSIX capabilities from running containers.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "add".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Added capabilities".to_owned()),
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
                        "drop".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Removed capabilities".to_owned()),
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
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
