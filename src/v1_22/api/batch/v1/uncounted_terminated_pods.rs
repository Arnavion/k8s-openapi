// Generated from definition io.k8s.api.batch.v1.UncountedTerminatedPods

/// UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted in Job status counters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UncountedTerminatedPods {
    /// Failed holds UIDs of failed Pods.
    pub failed: Option<Vec<String>>,

    /// Succeeded holds UIDs of succeeded Pods.
    pub succeeded: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl UncountedTerminatedPods  {
    /// Set [`Self::failed`]
    pub  fn failed_set(&mut self, failed: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.failed = failed.into(); self
    }

    pub  fn failed(&mut self) -> &mut Vec<String> {
        if self.failed.is_none() { self.failed = Some(Default::default()) }
        self.failed.as_mut().unwrap()
    }

    /// Modify [`Self::failed`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn failed_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.failed.is_none() { self.failed = Some(Default::default()) };
        func(self.failed.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::failed`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn failed_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.failed.is_none() {
            self.failed = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.failed.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::failed`]
    pub  fn failed_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.failed.is_none() { self.failed = Some(Vec::new()); }
         let failed = &mut self.failed.as_mut().unwrap();
         for item in other.borrow() {
             failed.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::succeeded`]
    pub  fn succeeded_set(&mut self, succeeded: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.succeeded = succeeded.into(); self
    }

    pub  fn succeeded(&mut self) -> &mut Vec<String> {
        if self.succeeded.is_none() { self.succeeded = Some(Default::default()) }
        self.succeeded.as_mut().unwrap()
    }

    /// Modify [`Self::succeeded`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn succeeded_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.succeeded.is_none() { self.succeeded = Some(Default::default()) };
        func(self.succeeded.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::succeeded`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn succeeded_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.succeeded.is_none() {
            self.succeeded = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.succeeded.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::succeeded`]
    pub  fn succeeded_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.succeeded.is_none() { self.succeeded = Some(Vec::new()); }
         let succeeded = &mut self.succeeded.as_mut().unwrap();
         for item in other.borrow() {
             succeeded.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for UncountedTerminatedPods {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_failed,
            Key_succeeded,
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
                            "failed" => Field::Key_failed,
                            "succeeded" => Field::Key_succeeded,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = UncountedTerminatedPods;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("UncountedTerminatedPods")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_failed: Option<Vec<String>> = None;
                let mut value_succeeded: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_failed => value_failed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_succeeded => value_succeeded = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(UncountedTerminatedPods {
                    failed: value_failed,
                    succeeded: value_succeeded,
                })
            }
        }

        deserializer.deserialize_struct(
            "UncountedTerminatedPods",
            &[
                "failed",
                "succeeded",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for UncountedTerminatedPods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "UncountedTerminatedPods",
            self.failed.as_ref().map_or(0, |_| 1) +
            self.succeeded.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.failed {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failed", value)?;
        }
        if let Some(value) = &self.succeeded {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "succeeded", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for UncountedTerminatedPods {
    fn schema_name() -> String {
        "io.k8s.api.batch.v1.UncountedTerminatedPods".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted in Job status counters.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "failed".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Failed holds UIDs of failed Pods.".to_owned()),
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
                        "succeeded".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Succeeded holds UIDs of succeeded Pods.".to_owned()),
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
