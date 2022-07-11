// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,

    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    pub values: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl LabelSelectorRequirement  {
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


    /// Set [`Self::operator`]
    pub  fn operator_set(&mut self, operator: impl Into<String>) -> &mut Self {
        self.operator = operator.into(); self
    }

    pub  fn operator(&mut self) -> &mut String {
        &mut self.operator
    }

    /// Modify [`Self::operator`] with a `func`
    pub  fn operator_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.operator); self
    }


    /// Set [`Self::values`]
    pub  fn values_set(&mut self, values: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.values = values.into(); self
    }

    pub  fn values(&mut self) -> &mut Vec<String> {
        if self.values.is_none() { self.values = Some(Default::default()) }
        self.values.as_mut().unwrap()
    }

    /// Modify [`Self::values`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn values_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.values.is_none() { self.values = Some(Default::default()) };
        func(self.values.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::values`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn values_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.values.is_none() {
            self.values = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.values.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::values`]
    pub  fn values_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.values.is_none() { self.values = Some(Vec::new()); }
         let values = &mut self.values.as_mut().unwrap();
         for item in other.borrow() {
             values.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for LabelSelectorRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_operator,
            Key_values,
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
                            "operator" => Field::Key_operator,
                            "values" => Field::Key_values,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LabelSelectorRequirement;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LabelSelectorRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_operator: Option<String> = None;
                let mut value_values: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operator => value_operator = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values => value_values = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LabelSelectorRequirement {
                    key: value_key.unwrap_or_default(),
                    operator: value_operator.unwrap_or_default(),
                    values: value_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "LabelSelectorRequirement",
            &[
                "key",
                "operator",
                "values",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LabelSelectorRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LabelSelectorRequirement",
            2 +
            self.values.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", &self.operator)?;
        if let Some(value) = &self.values {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "values", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LabelSelectorRequirement {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("key is the label key that the selector applies to.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "operator".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "values".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.".to_owned()),
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
                    "key".to_owned(),
                    "operator".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
