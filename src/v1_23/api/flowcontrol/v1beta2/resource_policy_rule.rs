// Generated from definition io.k8s.api.flowcontrol.v1beta2.ResourcePolicyRule

/// ResourcePolicyRule is a predicate that matches some resource requests, testing the request's verb and the target resource. A ResourcePolicyRule matches a resource request if and only if: (a) at least one member of verbs matches the request, (b) at least one member of apiGroups matches the request, (c) at least one member of resources matches the request, and (d) either (d1) the request does not specify a namespace (i.e., `Namespace==""`) and clusterScope is true or (d2) the request specifies a namespace and least one member of namespaces matches the request's namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourcePolicyRule {
    /// `apiGroups` is a list of matching API groups and may not be empty. "*" matches all API groups and, if present, must be the only entry. Required.
    pub api_groups: Vec<String>,

    /// `clusterScope` indicates whether to match requests that do not specify a namespace (which happens either because the resource is not namespaced or the request targets all namespaces). If this field is omitted or false then the `namespaces` field must contain a non-empty list.
    pub cluster_scope: Option<bool>,

    /// `namespaces` is a list of target namespaces that restricts matches.  A request that specifies a target namespace matches only if either (a) this list contains that target namespace or (b) this list contains "*".  Note that "*" matches any specified namespace but does not match a request that _does not specify_ a namespace (see the `clusterScope` field for that). This list may be empty, but only if `clusterScope` is true.
    pub namespaces: Option<Vec<String>>,

    /// `resources` is a list of matching resources (i.e., lowercase and plural) with, if desired, subresource.  For example, \[ "services", "nodes/status" \].  This list may not be empty. "*" matches all resources and, if present, must be the only entry. Required.
    pub resources: Vec<String>,

    /// `verbs` is a list of matching verbs and may not be empty. "*" matches all verbs and, if present, must be the only entry. Required.
    pub verbs: Vec<String>,

}

#[cfg(feature = "dsl")]
impl ResourcePolicyRule  {
    /// Set [`Self::api_groups`]
    pub  fn api_groups_set(&mut self, api_groups: impl Into<Vec<String>>) -> &mut Self {
        self.api_groups = api_groups.into(); self
    }

    pub  fn api_groups(&mut self) -> &mut Vec<String> {
        &mut self.api_groups
    }

    /// Modify [`Self::api_groups`] with a `func`
    pub  fn api_groups_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.api_groups); self
    }

    /// Push new element to [`Self::api_groups`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn api_groups_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.api_groups.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::api_groups`]
    pub  fn api_groups_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.api_groups.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::cluster_scope`]
    pub  fn cluster_scope_set(&mut self, cluster_scope: impl Into<Option<bool>>) -> &mut Self {
        self.cluster_scope = cluster_scope.into(); self
    }

    pub  fn cluster_scope(&mut self) -> &mut bool {
        if self.cluster_scope.is_none() { self.cluster_scope = Some(Default::default()) }
        self.cluster_scope.as_mut().unwrap()
    }

    /// Modify [`Self::cluster_scope`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cluster_scope_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.cluster_scope.is_none() { self.cluster_scope = Some(Default::default()) };
        func(self.cluster_scope.as_mut().unwrap()); self
    }


    /// Set [`Self::namespaces`]
    pub  fn namespaces_set(&mut self, namespaces: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.namespaces = namespaces.into(); self
    }

    pub  fn namespaces(&mut self) -> &mut Vec<String> {
        if self.namespaces.is_none() { self.namespaces = Some(Default::default()) }
        self.namespaces.as_mut().unwrap()
    }

    /// Modify [`Self::namespaces`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn namespaces_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.namespaces.is_none() { self.namespaces = Some(Default::default()) };
        func(self.namespaces.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::namespaces`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn namespaces_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.namespaces.is_none() {
            self.namespaces = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.namespaces.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::namespaces`]
    pub  fn namespaces_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.namespaces.is_none() { self.namespaces = Some(Vec::new()); }
         let namespaces = &mut self.namespaces.as_mut().unwrap();
         for item in other.borrow() {
             namespaces.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::resources`]
    pub  fn resources_set(&mut self, resources: impl Into<Vec<String>>) -> &mut Self {
        self.resources = resources.into(); self
    }

    pub  fn resources(&mut self) -> &mut Vec<String> {
        &mut self.resources
    }

    /// Modify [`Self::resources`] with a `func`
    pub  fn resources_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.resources); self
    }

    /// Push new element to [`Self::resources`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn resources_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.resources.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::resources`]
    pub  fn resources_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.resources.push(item.to_owned());
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


impl<'de> crate::serde::Deserialize<'de> for ResourcePolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_cluster_scope,
            Key_namespaces,
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
                            "clusterScope" => Field::Key_cluster_scope,
                            "namespaces" => Field::Key_namespaces,
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
            type Value = ResourcePolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourcePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_cluster_scope: Option<bool> = None;
                let mut value_namespaces: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_scope => value_cluster_scope = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespaces => value_namespaces = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourcePolicyRule {
                    api_groups: value_api_groups.unwrap_or_default(),
                    cluster_scope: value_cluster_scope,
                    namespaces: value_namespaces,
                    resources: value_resources.unwrap_or_default(),
                    verbs: value_verbs.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourcePolicyRule",
            &[
                "apiGroups",
                "clusterScope",
                "namespaces",
                "resources",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourcePolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourcePolicyRule",
            3 +
            self.cluster_scope.as_ref().map_or(0, |_| 1) +
            self.namespaces.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", &self.api_groups)?;
        if let Some(value) = &self.cluster_scope {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterScope", value)?;
        }
        if let Some(value) = &self.namespaces {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourcePolicyRule {
    fn schema_name() -> String {
        "io.k8s.api.flowcontrol.v1beta2.ResourcePolicyRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourcePolicyRule is a predicate that matches some resource requests, testing the request's verb and the target resource. A ResourcePolicyRule matches a resource request if and only if: (a) at least one member of verbs matches the request, (b) at least one member of apiGroups matches the request, (c) at least one member of resources matches the request, and (d) either (d1) the request does not specify a namespace (i.e., `Namespace==\"\"`) and clusterScope is true or (d2) the request specifies a namespace and least one member of namespaces matches the request's namespace.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiGroups".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`apiGroups` is a list of matching API groups and may not be empty. \"*\" matches all API groups and, if present, must be the only entry. Required.".to_owned()),
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
                        "clusterScope".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`clusterScope` indicates whether to match requests that do not specify a namespace (which happens either because the resource is not namespaced or the request targets all namespaces). If this field is omitted or false then the `namespaces` field must contain a non-empty list.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespaces".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`namespaces` is a list of target namespaces that restricts matches.  A request that specifies a target namespace matches only if either (a) this list contains that target namespace or (b) this list contains \"*\".  Note that \"*\" matches any specified namespace but does not match a request that _does not specify_ a namespace (see the `clusterScope` field for that). This list may be empty, but only if `clusterScope` is true.".to_owned()),
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
                                description: Some("`resources` is a list of matching resources (i.e., lowercase and plural) with, if desired, subresource.  For example, [ \"services\", \"nodes/status\" ].  This list may not be empty. \"*\" matches all resources and, if present, must be the only entry. Required.".to_owned()),
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
                                description: Some("`verbs` is a list of matching verbs and may not be empty. \"*\" matches all verbs and, if present, must be the only entry. Required.".to_owned()),
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
                    "apiGroups".to_owned(),
                    "resources".to_owned(),
                    "verbs".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
