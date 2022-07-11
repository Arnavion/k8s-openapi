// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.StorageVersionStatus

/// API server instances report the versions they can decode and the version they encode objects to when persisting objects in the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageVersionStatus {
    /// If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.
    pub common_encoding_version: Option<String>,

    /// The latest available observations of the storageVersion's state.
    pub conditions: Option<Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>>,

    /// The reported versions per API server instance.
    pub storage_versions: Option<Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>>,

}

#[cfg(feature = "dsl")]
impl StorageVersionStatus  {
    /// Set [`Self::common_encoding_version`]
    pub  fn common_encoding_version_set(&mut self, common_encoding_version: impl Into<Option<String>>) -> &mut Self {
        self.common_encoding_version = common_encoding_version.into(); self
    }

    pub  fn common_encoding_version(&mut self) -> &mut String {
        if self.common_encoding_version.is_none() { self.common_encoding_version = Some(Default::default()) }
        self.common_encoding_version.as_mut().unwrap()
    }

    /// Modify [`Self::common_encoding_version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn common_encoding_version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.common_encoding_version.is_none() { self.common_encoding_version = Some(Default::default()) };
        func(self.common_encoding_version.as_mut().unwrap()); self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::apiserverinternal::v1alpha1::StorageVersionCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::apiserverinternal::v1alpha1::StorageVersionCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::storage_versions`]
    pub  fn storage_versions_set(&mut self, storage_versions: impl Into<Option<Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>>>) -> &mut Self {
        self.storage_versions = storage_versions.into(); self
    }

    pub  fn storage_versions(&mut self) -> &mut Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion> {
        if self.storage_versions.is_none() { self.storage_versions = Some(Default::default()) }
        self.storage_versions.as_mut().unwrap()
    }

    /// Modify [`Self::storage_versions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storage_versions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>)) -> &mut Self {
        if self.storage_versions.is_none() { self.storage_versions = Some(Default::default()) };
        func(self.storage_versions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::storage_versions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn storage_versions_push_with(&mut self, func: impl FnOnce(&mut crate::api::apiserverinternal::v1alpha1::ServerStorageVersion)) -> &mut Self {
        if self.storage_versions.is_none() {
            self.storage_versions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.storage_versions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::storage_versions`]
    pub  fn storage_versions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::apiserverinternal::v1alpha1::ServerStorageVersion]>) -> &mut Self {
         if self.storage_versions.is_none() { self.storage_versions = Some(Vec::new()); }
         let storage_versions = &mut self.storage_versions.as_mut().unwrap();
         for item in other.borrow() {
             storage_versions.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for StorageVersionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_common_encoding_version,
            Key_conditions,
            Key_storage_versions,
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
                            "commonEncodingVersion" => Field::Key_common_encoding_version,
                            "conditions" => Field::Key_conditions,
                            "storageVersions" => Field::Key_storage_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageVersionStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StorageVersionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_common_encoding_version: Option<String> = None;
                let mut value_conditions: Option<Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>> = None;
                let mut value_storage_versions: Option<Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_common_encoding_version => value_common_encoding_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_versions => value_storage_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageVersionStatus {
                    common_encoding_version: value_common_encoding_version,
                    conditions: value_conditions,
                    storage_versions: value_storage_versions,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageVersionStatus",
            &[
                "commonEncodingVersion",
                "conditions",
                "storageVersions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageVersionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageVersionStatus",
            self.common_encoding_version.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.storage_versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.common_encoding_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "commonEncodingVersion", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.storage_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageVersions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageVersionStatus {
    fn schema_name() -> String {
        "io.k8s.api.apiserverinternal.v1alpha1.StorageVersionStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("API server instances report the versions they can decode and the version they encode objects to when persisting objects in the backend.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "commonEncodingVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The latest available observations of the storageVersion's state.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storageVersions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The reported versions per API server instance.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>()))),
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
