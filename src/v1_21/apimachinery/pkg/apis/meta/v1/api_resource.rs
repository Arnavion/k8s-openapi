// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIResource

/// APIResource specifies the name of a resource and whether it is namespaced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIResource {
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all')
    pub categories: Option<Vec<String>>,

    /// group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale".
    pub group: Option<String>,

    /// kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')
    pub kind: String,

    /// name is the plural name of the resource.
    pub name: String,

    /// namespaced indicates if a resource is namespaced or not.
    pub namespaced: bool,

    /// shortNames is a list of suggested short names of the resource.
    pub short_names: Option<Vec<String>>,

    /// singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.
    pub singular_name: String,

    /// The hash value of the storage version, the version this resource is converted to when written to the data store. Value must be treated as opaque by clients. Only equality comparison on the value is valid. This is an alpha feature and may change or be removed in the future. The field is populated by the apiserver only if the StorageVersionHash feature gate is enabled. This field will remain optional even if it graduates.
    pub storage_version_hash: Option<String>,

    /// verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)
    pub verbs: Vec<String>,

    /// version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)".
    pub version: Option<String>,

}

#[cfg(feature = "dsl")]
impl APIResource  {
    /// Set [`Self::categories`]
    pub  fn categories_set(&mut self, categories: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.categories = categories.into(); self
    }

    pub  fn categories(&mut self) -> &mut Vec<String> {
        if self.categories.is_none() { self.categories = Some(Default::default()) }
        self.categories.as_mut().unwrap()
    }

    /// Modify [`Self::categories`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn categories_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.categories.is_none() { self.categories = Some(Default::default()) };
        func(self.categories.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::categories`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn categories_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.categories.is_none() {
            self.categories = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.categories.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::categories`]
    pub  fn categories_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.categories.is_none() { self.categories = Some(Vec::new()); }
         let categories = &mut self.categories.as_mut().unwrap();
         for item in other.borrow() {
             categories.push(item.to_owned());
         }
         self
    }


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


    /// Set [`Self::kind`]
    pub  fn kind_set(&mut self, kind: impl Into<String>) -> &mut Self {
        self.kind = kind.into(); self
    }

    pub  fn kind(&mut self) -> &mut String {
        &mut self.kind
    }

    /// Modify [`Self::kind`] with a `func`
    pub  fn kind_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.kind); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::namespaced`]
    pub  fn namespaced_set(&mut self, namespaced: impl Into<bool>) -> &mut Self {
        self.namespaced = namespaced.into(); self
    }

    pub  fn namespaced(&mut self) -> &mut bool {
        &mut self.namespaced
    }

    /// Modify [`Self::namespaced`] with a `func`
    pub  fn namespaced_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        func(&mut self.namespaced); self
    }


    /// Set [`Self::short_names`]
    pub  fn short_names_set(&mut self, short_names: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.short_names = short_names.into(); self
    }

    pub  fn short_names(&mut self) -> &mut Vec<String> {
        if self.short_names.is_none() { self.short_names = Some(Default::default()) }
        self.short_names.as_mut().unwrap()
    }

    /// Modify [`Self::short_names`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn short_names_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.short_names.is_none() { self.short_names = Some(Default::default()) };
        func(self.short_names.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::short_names`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn short_names_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.short_names.is_none() {
            self.short_names = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.short_names.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::short_names`]
    pub  fn short_names_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.short_names.is_none() { self.short_names = Some(Vec::new()); }
         let short_names = &mut self.short_names.as_mut().unwrap();
         for item in other.borrow() {
             short_names.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::singular_name`]
    pub  fn singular_name_set(&mut self, singular_name: impl Into<String>) -> &mut Self {
        self.singular_name = singular_name.into(); self
    }

    pub  fn singular_name(&mut self) -> &mut String {
        &mut self.singular_name
    }

    /// Modify [`Self::singular_name`] with a `func`
    pub  fn singular_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.singular_name); self
    }


    /// Set [`Self::storage_version_hash`]
    pub  fn storage_version_hash_set(&mut self, storage_version_hash: impl Into<Option<String>>) -> &mut Self {
        self.storage_version_hash = storage_version_hash.into(); self
    }

    pub  fn storage_version_hash(&mut self) -> &mut String {
        if self.storage_version_hash.is_none() { self.storage_version_hash = Some(Default::default()) }
        self.storage_version_hash.as_mut().unwrap()
    }

    /// Modify [`Self::storage_version_hash`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storage_version_hash_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.storage_version_hash.is_none() { self.storage_version_hash = Some(Default::default()) };
        func(self.storage_version_hash.as_mut().unwrap()); self
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


impl<'de> crate::serde::Deserialize<'de> for APIResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_categories,
            Key_group,
            Key_kind,
            Key_name,
            Key_namespaced,
            Key_short_names,
            Key_singular_name,
            Key_storage_version_hash,
            Key_verbs,
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
                            "categories" => Field::Key_categories,
                            "group" => Field::Key_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "namespaced" => Field::Key_namespaced,
                            "shortNames" => Field::Key_short_names,
                            "singularName" => Field::Key_singular_name,
                            "storageVersionHash" => Field::Key_storage_version_hash,
                            "verbs" => Field::Key_verbs,
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
            type Value = APIResource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("APIResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_categories: Option<Vec<String>> = None;
                let mut value_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespaced: Option<bool> = None;
                let mut value_short_names: Option<Vec<String>> = None;
                let mut value_singular_name: Option<String> = None;
                let mut value_storage_version_hash: Option<String> = None;
                let mut value_verbs: Option<Vec<String>> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_categories => value_categories = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespaced => value_namespaced = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_short_names => value_short_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_singular_name => value_singular_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_version_hash => value_storage_version_hash = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIResource {
                    categories: value_categories,
                    group: value_group,
                    kind: value_kind.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    namespaced: value_namespaced.unwrap_or_default(),
                    short_names: value_short_names,
                    singular_name: value_singular_name.unwrap_or_default(),
                    storage_version_hash: value_storage_version_hash,
                    verbs: value_verbs.unwrap_or_default(),
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIResource",
            &[
                "categories",
                "group",
                "kind",
                "name",
                "namespaced",
                "shortNames",
                "singularName",
                "storageVersionHash",
                "verbs",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for APIResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIResource",
            5 +
            self.categories.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.short_names.as_ref().map_or(0, |_| 1) +
            self.storage_version_hash.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.categories {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "categories", value)?;
        }
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaced", &self.namespaced)?;
        if let Some(value) = &self.short_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shortNames", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "singularName", &self.singular_name)?;
        if let Some(value) = &self.storage_version_hash {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageVersionHash", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for APIResource {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.APIResource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("APIResource specifies the name of a resource and whether it is namespaced.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "categories".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("categories is a list of the grouped resources this resource belongs to (e.g. 'all')".to_owned()),
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
                        "group".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')".to_owned()),
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
                                description: Some("name is the plural name of the resource.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespaced".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("namespaced indicates if a resource is namespaced or not.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "shortNames".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("shortNames is a list of suggested short names of the resource.".to_owned()),
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
                        "singularName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storageVersionHash".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The hash value of the storage version, the version this resource is converted to when written to the data store. Value must be treated as opaque by clients. Only equality comparison on the value is valid. This is an alpha feature and may change or be removed in the future. The field is populated by the apiserver only if the StorageVersionHash feature gate is enabled. This field will remain optional even if it graduates.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "verbs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)".to_owned()),
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
                        "version".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "kind".to_owned(),
                    "name".to_owned(),
                    "namespaced".to_owned(),
                    "singularName".to_owned(),
                    "verbs".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
