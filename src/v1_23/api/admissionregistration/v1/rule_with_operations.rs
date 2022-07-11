// Generated from definition io.k8s.api.admissionregistration.v1.RuleWithOperations

/// RuleWithOperations is a tuple of Operations and Resources. It is recommended to make sure that all the tuple expansions are valid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuleWithOperations {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    pub api_groups: Option<Vec<String>>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    pub api_versions: Option<Vec<String>>,

    /// Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.
    pub operations: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.
    ///
    /// For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.
    ///
    /// If wildcard is present, the validation rule will ensure resources do not overlap with each other.
    ///
    /// Depending on the enclosing object, subresources might not be allowed. Required.
    pub resources: Option<Vec<String>>,

    /// scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "*".
    pub scope: Option<String>,

}

#[cfg(feature = "dsl")]
impl RuleWithOperations  {
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


    /// Set [`Self::api_versions`]
    pub  fn api_versions_set(&mut self, api_versions: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.api_versions = api_versions.into(); self
    }

    pub  fn api_versions(&mut self) -> &mut Vec<String> {
        if self.api_versions.is_none() { self.api_versions = Some(Default::default()) }
        self.api_versions.as_mut().unwrap()
    }

    /// Modify [`Self::api_versions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn api_versions_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.api_versions.is_none() { self.api_versions = Some(Default::default()) };
        func(self.api_versions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::api_versions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn api_versions_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.api_versions.is_none() {
            self.api_versions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.api_versions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::api_versions`]
    pub  fn api_versions_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.api_versions.is_none() { self.api_versions = Some(Vec::new()); }
         let api_versions = &mut self.api_versions.as_mut().unwrap();
         for item in other.borrow() {
             api_versions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::operations`]
    pub  fn operations_set(&mut self, operations: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.operations = operations.into(); self
    }

    pub  fn operations(&mut self) -> &mut Vec<String> {
        if self.operations.is_none() { self.operations = Some(Default::default()) }
        self.operations.as_mut().unwrap()
    }

    /// Modify [`Self::operations`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn operations_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.operations.is_none() { self.operations = Some(Default::default()) };
        func(self.operations.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::operations`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn operations_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.operations.is_none() {
            self.operations = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.operations.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::operations`]
    pub  fn operations_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.operations.is_none() { self.operations = Some(Vec::new()); }
         let operations = &mut self.operations.as_mut().unwrap();
         for item in other.borrow() {
             operations.push(item.to_owned());
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


    /// Set [`Self::scope`]
    pub  fn scope_set(&mut self, scope: impl Into<Option<String>>) -> &mut Self {
        self.scope = scope.into(); self
    }

    pub  fn scope(&mut self) -> &mut String {
        if self.scope.is_none() { self.scope = Some(Default::default()) }
        self.scope.as_mut().unwrap()
    }

    /// Modify [`Self::scope`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scope_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.scope.is_none() { self.scope = Some(Default::default()) };
        func(self.scope.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for RuleWithOperations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_api_versions,
            Key_operations,
            Key_resources,
            Key_scope,
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
                            "apiVersions" => Field::Key_api_versions,
                            "operations" => Field::Key_operations,
                            "resources" => Field::Key_resources,
                            "scope" => Field::Key_scope,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RuleWithOperations;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RuleWithOperations")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_api_versions: Option<Vec<String>> = None;
                let mut value_operations: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_scope: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_versions => value_api_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operations => value_operations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RuleWithOperations {
                    api_groups: value_api_groups,
                    api_versions: value_api_versions,
                    operations: value_operations,
                    resources: value_resources,
                    scope: value_scope,
                })
            }
        }

        deserializer.deserialize_struct(
            "RuleWithOperations",
            &[
                "apiGroups",
                "apiVersions",
                "operations",
                "resources",
                "scope",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RuleWithOperations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RuleWithOperations",
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.api_versions.as_ref().map_or(0, |_| 1) +
            self.operations.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.scope.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.api_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersions", value)?;
        }
        if let Some(value) = &self.operations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operations", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.scope {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scope", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RuleWithOperations {
    fn schema_name() -> String {
        "io.k8s.api.admissionregistration.v1.RuleWithOperations".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RuleWithOperations is a tuple of Operations and Resources. It is recommended to make sure that all the tuple expansions are valid.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiGroups".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.".to_owned()),
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
                        "apiVersions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.".to_owned()),
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
                        "operations".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.".to_owned()),
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
                                description: Some("Resources is a list of resources this rule applies to.\n\nFor example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.\n\nIf wildcard is present, the validation rule will ensure resources do not overlap with each other.\n\nDepending on the enclosing object, subresources might not be allowed. Required.".to_owned()),
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
                        "scope".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("scope specifies the scope of this rule. Valid values are \"Cluster\", \"Namespaced\", and \"*\" \"Cluster\" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. \"Namespaced\" means that only namespaced resources will match this rule. \"*\" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is \"*\".".to_owned()),
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
