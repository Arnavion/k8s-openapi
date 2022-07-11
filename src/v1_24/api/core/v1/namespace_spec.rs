// Generated from definition io.k8s.api.core.v1.NamespaceSpec

/// NamespaceSpec describes the attributes on a Namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    pub finalizers: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl NamespaceSpec  {
    /// Set [`Self::finalizers`]
    pub  fn finalizers_set(&mut self, finalizers: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.finalizers = finalizers.into(); self
    }

    pub  fn finalizers(&mut self) -> &mut Vec<String> {
        if self.finalizers.is_none() { self.finalizers = Some(Default::default()) }
        self.finalizers.as_mut().unwrap()
    }

    /// Modify [`Self::finalizers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn finalizers_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.finalizers.is_none() { self.finalizers = Some(Default::default()) };
        func(self.finalizers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::finalizers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn finalizers_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.finalizers.is_none() {
            self.finalizers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.finalizers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::finalizers`]
    pub  fn finalizers_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.finalizers.is_none() { self.finalizers = Some(Vec::new()); }
         let finalizers = &mut self.finalizers.as_mut().unwrap();
         for item in other.borrow() {
             finalizers.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for NamespaceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_finalizers,
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
                            "finalizers" => Field::Key_finalizers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NamespaceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_finalizers: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_finalizers => value_finalizers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamespaceSpec {
                    finalizers: value_finalizers,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamespaceSpec",
            &[
                "finalizers",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamespaceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamespaceSpec",
            self.finalizers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.finalizers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamespaceSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NamespaceSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NamespaceSpec describes the attributes on a Namespace.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "finalizers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/".to_owned()),
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
