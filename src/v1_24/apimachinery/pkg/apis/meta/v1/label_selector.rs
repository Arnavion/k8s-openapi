// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    pub match_expressions: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>>,

    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    pub match_labels: Option<std::collections::BTreeMap<String, String>>,

}

#[cfg(feature = "dsl")]
impl LabelSelector  {
    /// Set [`Self::match_expressions`]
    pub  fn match_expressions_set(&mut self, match_expressions: impl Into<Option<Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>>>) -> &mut Self {
        self.match_expressions = match_expressions.into(); self
    }

    pub  fn match_expressions(&mut self) -> &mut Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement> {
        if self.match_expressions.is_none() { self.match_expressions = Some(Default::default()) }
        self.match_expressions.as_mut().unwrap()
    }

    /// Modify [`Self::match_expressions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn match_expressions_with(&mut self, func: impl FnOnce(&mut Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>)) -> &mut Self {
        if self.match_expressions.is_none() { self.match_expressions = Some(Default::default()) };
        func(self.match_expressions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::match_expressions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn match_expressions_push_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement)) -> &mut Self {
        if self.match_expressions.is_none() {
            self.match_expressions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.match_expressions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::match_expressions`]
    pub  fn match_expressions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement]>) -> &mut Self {
         if self.match_expressions.is_none() { self.match_expressions = Some(Vec::new()); }
         let match_expressions = &mut self.match_expressions.as_mut().unwrap();
         for item in other.borrow() {
             match_expressions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::match_labels`]
    pub  fn match_labels_set(&mut self, match_labels: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.match_labels = match_labels.into(); self
    }

    pub  fn match_labels(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.match_labels.is_none() { self.match_labels = Some(Default::default()) }
        self.match_labels.as_mut().unwrap()
    }

    /// Modify [`Self::match_labels`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn match_labels_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.match_labels.is_none() { self.match_labels = Some(Default::default()) };
        func(self.match_labels.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::match_labels`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn match_labels_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.match_labels.is_none() {
            self.match_labels = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.match_labels.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::match_labels`]
    pub  fn match_labels_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.match_labels.is_none() { self.match_labels = Some(std::collections::BTreeMap::new()); }
         let match_labels = &mut self.match_labels.as_mut().unwrap();
         for (name, value) in other.borrow() {
             match_labels.insert(name.to_owned(), value.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for LabelSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
            Key_match_labels,
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
                            "matchExpressions" => Field::Key_match_expressions,
                            "matchLabels" => Field::Key_match_labels,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LabelSelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LabelSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_expressions: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>> = None;
                let mut value_match_labels: Option<std::collections::BTreeMap<String, String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_labels => value_match_labels = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LabelSelector {
                    match_expressions: value_match_expressions,
                    match_labels: value_match_labels,
                })
            }
        }

        deserializer.deserialize_struct(
            "LabelSelector",
            &[
                "matchExpressions",
                "matchLabels",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LabelSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LabelSelector",
            self.match_expressions.as_ref().map_or(0, |_| 1) +
            self.match_labels.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expressions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", value)?;
        }
        if let Some(value) = &self.match_labels {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchLabels", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LabelSelector {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "matchExpressions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("matchExpressions is a list of label selector requirements. The requirements are ANDed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchLabels".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is \"key\", the operator is \"In\", and the values array contains only \"value\". The requirements are ANDed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
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
