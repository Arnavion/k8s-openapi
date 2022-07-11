// Generated from definition io.k8s.api.authorization.v1.ResourceRule

/// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.
    pub api_groups: Option<Vec<String>>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
    pub resource_names: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
    ///  "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
    pub resources: Option<Vec<String>>,

    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verbs: Vec<String>,

}

#[cfg(feature = "dsl")]
impl ResourceRule  {
    /// Set [`Self::api_groups`]
    pub  fn api_groups_set(&mut self, api_groups: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.api_groups = api_groups.into(); self
    }

    pub  fn api_groups(&mut self) -> &mut Vec<String> {
        if self.api_groups.is_none() { self.api_groups = Some(Default::default()) }
        self.api_groups.as_mut().unwrap()
    }

    /// Modify [`Self::api_groups`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn api_groups_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.api_groups.is_none() { self.api_groups = Some(Default::default()) };
        func(self.api_groups.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::api_groups`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn api_groups_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.api_groups.is_none() {
            self.api_groups = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.api_groups.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::api_groups`]
    pub  fn api_groups_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.api_groups.is_none() { self.api_groups = Some(Vec::new()); }
         let api_groups = &mut self.api_groups.as_mut().unwrap();
         for item in other.borrow() {
             api_groups.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::resource_names`]
    pub  fn resource_names_set(&mut self, resource_names: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.resource_names = resource_names.into(); self
    }

    pub  fn resource_names(&mut self) -> &mut Vec<String> {
        if self.resource_names.is_none() { self.resource_names = Some(Default::default()) }
        self.resource_names.as_mut().unwrap()
    }

    /// Modify [`Self::resource_names`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_names_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.resource_names.is_none() { self.resource_names = Some(Default::default()) };
        func(self.resource_names.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::resource_names`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn resource_names_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.resource_names.is_none() {
            self.resource_names = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.resource_names.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::resource_names`]
    pub  fn resource_names_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.resource_names.is_none() { self.resource_names = Some(Vec::new()); }
         let resource_names = &mut self.resource_names.as_mut().unwrap();
         for item in other.borrow() {
             resource_names.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::resources`]
    pub  fn resources_set(&mut self, resources: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.resources = resources.into(); self
    }

    pub  fn resources(&mut self) -> &mut Vec<String> {
        if self.resources.is_none() { self.resources = Some(Default::default()) }
        self.resources.as_mut().unwrap()
    }

    /// Modify [`Self::resources`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resources_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.resources.is_none() { self.resources = Some(Default::default()) };
        func(self.resources.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::resources`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn resources_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.resources.is_none() {
            self.resources = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.resources.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::resources`]
    pub  fn resources_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.resources.is_none() { self.resources = Some(Vec::new()); }
         let resources = &mut self.resources.as_mut().unwrap();
         for item in other.borrow() {
             resources.push(item.to_owned());
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


impl<'de> crate::serde::Deserialize<'de> for ResourceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_resource_names,
            Key_resources,
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
                            "apiGroups" => Field::Key_api_groups,
                            "resourceNames" => Field::Key_resource_names,
                            "resources" => Field::Key_resources,
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
            type Value = ResourceRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_resource_names: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRule {
                    api_groups: value_api_groups,
                    resource_names: value_resource_names,
                    resources: value_resources,
                    verbs: value_verbs.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceRule",
            &[
                "apiGroups",
                "resourceNames",
                "resources",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceRule",
            1 +
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.resource_names.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.resource_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceNames", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceRule {
    fn schema_name() -> String {
        "io.k8s.api.authorization.v1.ResourceRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiGroups".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  \"*\" means all.".to_owned()),
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
                        "resourceNames".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  \"*\" means all.".to_owned()),
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
                        "resources".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Resources is a list of resources this rule applies to.  \"*\" means all in the specified apiGroups.\n \"*/foo\" represents the subresource 'foo' for all resources in the specified apiGroups.".to_owned()),
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
                                description: Some("Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.".to_owned()),
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
