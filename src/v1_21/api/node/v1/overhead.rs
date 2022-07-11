// Generated from definition io.k8s.api.node.v1.Overhead

/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    pub pod_fixed: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

}

#[cfg(feature = "dsl")]
impl Overhead  {
    /// Set [`Self::pod_fixed`]
    pub  fn pod_fixed_set(&mut self, pod_fixed: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.pod_fixed = pod_fixed.into(); self
    }

    pub  fn pod_fixed(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.pod_fixed.is_none() { self.pod_fixed = Some(Default::default()) }
        self.pod_fixed.as_mut().unwrap()
    }

    /// Modify [`Self::pod_fixed`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pod_fixed_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.pod_fixed.is_none() { self.pod_fixed = Some(Default::default()) };
        func(self.pod_fixed.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::pod_fixed`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn pod_fixed_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.pod_fixed.is_none() {
            self.pod_fixed = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.pod_fixed.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::pod_fixed`]
    pub  fn pod_fixed_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.pod_fixed.is_none() { self.pod_fixed = Some(std::collections::BTreeMap::new()); }
         let pod_fixed = &mut self.pod_fixed.as_mut().unwrap();
         for (name, value) in other.borrow() {
             pod_fixed.insert(name.to_owned(), value.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Overhead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_fixed,
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
                            "podFixed" => Field::Key_pod_fixed,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Overhead;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Overhead")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_pod_fixed: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_fixed => value_pod_fixed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Overhead {
                    pod_fixed: value_pod_fixed,
                })
            }
        }

        deserializer.deserialize_struct(
            "Overhead",
            &[
                "podFixed",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Overhead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Overhead",
            self.pod_fixed.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.pod_fixed {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podFixed", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Overhead {
    fn schema_name() -> String {
        "io.k8s.api.node.v1.Overhead".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Overhead structure represents the resource overhead associated with running a pod.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "podFixed".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PodFixed represents the fixed resource overhead associated with running a pod.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
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
