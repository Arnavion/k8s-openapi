// Generated from definition io.k8s.api.authorization.v1.ResourceAttributes

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceAttributes {
    /// Group is the API Group of the Resource.  "*" means all.
    pub group: Option<String>,

    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    pub name: Option<String>,

    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    pub namespace: Option<String>,

    /// Resource is one of the existing resource types.  "*" means all.
    pub resource: Option<String>,

    /// Subresource is one of the existing resource types.  "" means none.
    pub subresource: Option<String>,

    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verb: Option<String>,

    /// Version is the API Version of the Resource.  "*" means all.
    pub version: Option<String>,

}

#[cfg(feature = "dsl")]
impl ResourceAttributes  {
    /// Set [`Self::group`]
    pub  fn group_set(&mut self, group: impl Into<Option<String>>) -> &mut Self {
        self.group = group.into(); self
    }

    pub  fn group(&mut self) -> &mut String {
        if self.group.is_none() { self.group = Some(Default::default()) }
        self.group.as_mut().unwrap()
    }

    /// Modify [`Self::group`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn group_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.group.is_none() { self.group = Some(Default::default()) };
        func(self.group.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<Option<String>>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        if self.name.is_none() { self.name = Some(Default::default()) }
        self.name.as_mut().unwrap()
    }

    /// Modify [`Self::name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.name.is_none() { self.name = Some(Default::default()) };
        func(self.name.as_mut().unwrap()); self
    }


    /// Set [`Self::namespace`]
    pub  fn namespace_set(&mut self, namespace: impl Into<Option<String>>) -> &mut Self {
        self.namespace = namespace.into(); self
    }

    pub  fn namespace(&mut self) -> &mut String {
        if self.namespace.is_none() { self.namespace = Some(Default::default()) }
        self.namespace.as_mut().unwrap()
    }

    /// Modify [`Self::namespace`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn namespace_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.namespace.is_none() { self.namespace = Some(Default::default()) };
        func(self.namespace.as_mut().unwrap()); self
    }


    /// Set [`Self::resource`]
    pub  fn resource_set(&mut self, resource: impl Into<Option<String>>) -> &mut Self {
        self.resource = resource.into(); self
    }

    pub  fn resource(&mut self) -> &mut String {
        if self.resource.is_none() { self.resource = Some(Default::default()) }
        self.resource.as_mut().unwrap()
    }

    /// Modify [`Self::resource`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.resource.is_none() { self.resource = Some(Default::default()) };
        func(self.resource.as_mut().unwrap()); self
    }


    /// Set [`Self::subresource`]
    pub  fn subresource_set(&mut self, subresource: impl Into<Option<String>>) -> &mut Self {
        self.subresource = subresource.into(); self
    }

    pub  fn subresource(&mut self) -> &mut String {
        if self.subresource.is_none() { self.subresource = Some(Default::default()) }
        self.subresource.as_mut().unwrap()
    }

    /// Modify [`Self::subresource`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn subresource_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.subresource.is_none() { self.subresource = Some(Default::default()) };
        func(self.subresource.as_mut().unwrap()); self
    }


    /// Set [`Self::verb`]
    pub  fn verb_set(&mut self, verb: impl Into<Option<String>>) -> &mut Self {
        self.verb = verb.into(); self
    }

    pub  fn verb(&mut self) -> &mut String {
        if self.verb.is_none() { self.verb = Some(Default::default()) }
        self.verb.as_mut().unwrap()
    }

    /// Modify [`Self::verb`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn verb_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.verb.is_none() { self.verb = Some(Default::default()) };
        func(self.verb.as_mut().unwrap()); self
    }


    /// Set [`Self::version`]
    pub  fn version_set(&mut self, version: impl Into<Option<String>>) -> &mut Self {
        self.version = version.into(); self
    }

    pub  fn version(&mut self) -> &mut String {
        if self.version.is_none() { self.version = Some(Default::default()) }
        self.version.as_mut().unwrap()
    }

    /// Modify [`Self::version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.version.is_none() { self.version = Some(Default::default()) };
        func(self.version.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_name,
            Key_namespace,
            Key_resource,
            Key_subresource,
            Key_verb,
            Key_version,
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
                            "group" => Field::Key_group,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "resource" => Field::Key_resource,
                            "subresource" => Field::Key_subresource,
                            "verb" => Field::Key_verb,
                            "version" => Field::Key_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_resource: Option<String> = None;
                let mut value_subresource: Option<String> = None;
                let mut value_verb: Option<String> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subresource => value_subresource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verb => value_verb = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceAttributes {
                    group: value_group,
                    name: value_name,
                    namespace: value_namespace,
                    resource: value_resource,
                    subresource: value_subresource,
                    verb: value_verb,
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceAttributes",
            &[
                "group",
                "name",
                "namespace",
                "resource",
                "subresource",
                "verb",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceAttributes",
            self.group.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.resource.as_ref().map_or(0, |_| 1) +
            self.subresource.as_ref().map_or(0, |_| 1) +
            self.verb.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        if let Some(value) = &self.subresource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subresource", value)?;
        }
        if let Some(value) = &self.verb {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verb", value)?;
        }
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceAttributes {
    fn schema_name() -> String {
        "io.k8s.api.authorization.v1.ResourceAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "group".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Group is the API Group of the Resource.  \"*\" means all.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is the name of the resource being requested for a \"get\" or deleted for a \"delete\". \"\" (empty) means all.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \"\" (empty) is defaulted for LocalSubjectAccessReviews \"\" (empty) is empty for cluster-scoped resources \"\" (empty) means \"all\" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resource".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Resource is one of the existing resource types.  \"*\" means all.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subresource".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Subresource is one of the existing resource types.  \"\" means none.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "verb".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "version".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Version is the API Version of the Resource.  \"*\" means all.".to_owned()),
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
