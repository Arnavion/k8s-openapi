// Generated from definition io.k8s.api.core.v1.ResourceQuotaSpec

/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceQuotaSpec {
    /// hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    pub hard: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
    pub scope_selector: Option<crate::api::core::v1::ScopeSelector>,

    /// A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
    pub scopes: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl ResourceQuotaSpec  {
    /// Set [`Self::hard`]
    pub  fn hard_set(&mut self, hard: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.hard = hard.into(); self
    }

    pub  fn hard(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.hard.is_none() { self.hard = Some(Default::default()) }
        self.hard.as_mut().unwrap()
    }

    /// Modify [`Self::hard`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn hard_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.hard.is_none() { self.hard = Some(Default::default()) };
        func(self.hard.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::hard`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn hard_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.hard.is_none() {
            self.hard = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.hard.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::hard`]
    pub  fn hard_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.hard.is_none() { self.hard = Some(std::collections::BTreeMap::new()); }
         let hard = &mut self.hard.as_mut().unwrap();
         for (name, value) in other.borrow() {
             hard.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::scope_selector`]
    pub  fn scope_selector_set(&mut self, scope_selector: impl Into<Option<crate::api::core::v1::ScopeSelector>>) -> &mut Self {
        self.scope_selector = scope_selector.into(); self
    }

    pub  fn scope_selector(&mut self) -> &mut crate::api::core::v1::ScopeSelector {
        if self.scope_selector.is_none() { self.scope_selector = Some(Default::default()) }
        self.scope_selector.as_mut().unwrap()
    }

    /// Modify [`Self::scope_selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scope_selector_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ScopeSelector)) -> &mut Self {
        if self.scope_selector.is_none() { self.scope_selector = Some(Default::default()) };
        func(self.scope_selector.as_mut().unwrap()); self
    }


    /// Set [`Self::scopes`]
    pub  fn scopes_set(&mut self, scopes: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.scopes = scopes.into(); self
    }

    pub  fn scopes(&mut self) -> &mut Vec<String> {
        if self.scopes.is_none() { self.scopes = Some(Default::default()) }
        self.scopes.as_mut().unwrap()
    }

    /// Modify [`Self::scopes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scopes_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.scopes.is_none() { self.scopes = Some(Default::default()) };
        func(self.scopes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::scopes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn scopes_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.scopes.is_none() {
            self.scopes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.scopes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::scopes`]
    pub  fn scopes_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.scopes.is_none() { self.scopes = Some(Vec::new()); }
         let scopes = &mut self.scopes.as_mut().unwrap();
         for item in other.borrow() {
             scopes.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ResourceQuotaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hard,
            Key_scope_selector,
            Key_scopes,
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
                            "hard" => Field::Key_hard,
                            "scopeSelector" => Field::Key_scope_selector,
                            "scopes" => Field::Key_scopes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceQuotaSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceQuotaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hard: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_scope_selector: Option<crate::api::core::v1::ScopeSelector> = None;
                let mut value_scopes: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hard => value_hard = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope_selector => value_scope_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scopes => value_scopes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceQuotaSpec {
                    hard: value_hard,
                    scope_selector: value_scope_selector,
                    scopes: value_scopes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceQuotaSpec",
            &[
                "hard",
                "scopeSelector",
                "scopes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceQuotaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceQuotaSpec",
            self.hard.as_ref().map_or(0, |_| 1) +
            self.scope_selector.as_ref().map_or(0, |_| 1) +
            self.scopes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hard {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hard", value)?;
        }
        if let Some(value) = &self.scope_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopeSelector", value)?;
        }
        if let Some(value) = &self.scopes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceQuotaSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ResourceQuotaSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceQuotaSpec defines the desired hard limits to enforce for Quota.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "hard".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/".to_owned()),
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
                    (
                        "scopeSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ScopeSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scopes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.".to_owned()),
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
