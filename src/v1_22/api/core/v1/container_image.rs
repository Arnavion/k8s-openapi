// Generated from definition io.k8s.api.core.v1.ContainerImage

/// Describe a container image
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerImage {
    /// Names by which this image is known. e.g. \["k8s.gcr.io/hyperkube:v1.0.7", "dockerhub.io/google_containers/hyperkube:v1.0.7"\]
    pub names: Option<Vec<String>>,

    /// The size of the image in bytes.
    pub size_bytes: Option<i64>,

}

#[cfg(feature = "dsl")]
impl ContainerImage  {
    /// Set [`Self::names`]
    pub  fn names_set(&mut self, names: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.names = names.into(); self
    }

    pub  fn names(&mut self) -> &mut Vec<String> {
        if self.names.is_none() { self.names = Some(Default::default()) }
        self.names.as_mut().unwrap()
    }

    /// Modify [`Self::names`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn names_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.names.is_none() { self.names = Some(Default::default()) };
        func(self.names.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::names`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn names_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.names.is_none() {
            self.names = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.names.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::names`]
    pub  fn names_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.names.is_none() { self.names = Some(Vec::new()); }
         let names = &mut self.names.as_mut().unwrap();
         for item in other.borrow() {
             names.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::size_bytes`]
    pub  fn size_bytes_set(&mut self, size_bytes: impl Into<Option<i64>>) -> &mut Self {
        self.size_bytes = size_bytes.into(); self
    }

    pub  fn size_bytes(&mut self) -> &mut i64 {
        if self.size_bytes.is_none() { self.size_bytes = Some(Default::default()) }
        self.size_bytes.as_mut().unwrap()
    }

    /// Modify [`Self::size_bytes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn size_bytes_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.size_bytes.is_none() { self.size_bytes = Some(Default::default()) };
        func(self.size_bytes.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ContainerImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_names,
            Key_size_bytes,
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
                            "names" => Field::Key_names,
                            "sizeBytes" => Field::Key_size_bytes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerImage;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerImage")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_names: Option<Vec<String>> = None;
                let mut value_size_bytes: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_names => value_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_size_bytes => value_size_bytes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerImage {
                    names: value_names,
                    size_bytes: value_size_bytes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerImage",
            &[
                "names",
                "sizeBytes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerImage",
            self.names.as_ref().map_or(0, |_| 1) +
            self.size_bytes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "names", value)?;
        }
        if let Some(value) = &self.size_bytes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sizeBytes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerImage {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ContainerImage".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Describe a container image".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "names".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Names by which this image is known. e.g. [\"k8s.gcr.io/hyperkube:v1.0.7\", \"dockerhub.io/google_containers/hyperkube:v1.0.7\"]".to_owned()),
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
                        "sizeBytes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The size of the image in bytes.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
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
